use crate::udev::UdevDevice;
use oci_spec::runtime::{
    LinuxDeviceBuilder, LinuxDeviceCgroupBuilder, LinuxDeviceType, Spec as RuntimeSpec,
};
use std::error::Error;
use std::os::unix::fs::{FileTypeExt, MetadataExt, PermissionsExt};

pub trait RuntimeSpecExt {
    fn from_reader(reader: impl std::io::Read) -> Result<RuntimeSpec, Box<dyn Error>>;
    fn to_string(&self) -> Result<String, Box<dyn Error>>;
}

impl RuntimeSpecExt for RuntimeSpec {
    fn from_reader(reader: impl std::io::Read) -> Result<RuntimeSpec, Box<dyn Error>> {
        Ok(serde_json::from_reader(reader)?)
    }

    fn to_string(&self) -> Result<String, Box<dyn Error>> {
        Ok(serde_json::to_string(self)?)
    }
}

pub trait RuntimeSpecUdev {
    fn add_udev_device<D: UdevDevice>(&mut self, device: D) -> Result<(), Box<dyn Error>>;
}

impl RuntimeSpecUdev for RuntimeSpec {
    fn add_udev_device<D: UdevDevice>(&mut self, device: D) -> Result<(), Box<dyn Error>> {
        let udev_path = match device.devnode() {
            Some(path) => path,
            None => return Ok(()),
        };

        let linux = self.linux_mut().get_or_insert_with(Default::default);
        let devices = linux.devices_mut().get_or_insert_with(Default::default);

        // Check if device is already in the list and return early if exists
        if devices.iter().any(|d| d.path() == udev_path) {
            return Ok(());
        };

        let major: i64 = device
            .property_value("MAJOR")
            .and_then(|s| s.to_str())
            .ok_or("Major number not available")?
            .parse()?;

        let minor: i64 = device
            .property_value("MINOR")
            .and_then(|s| s.to_str())
            .ok_or("Minor number not available")?
            .parse()?;

        let metadata = std::fs::metadata(udev_path)?;
        let file_type = metadata.file_type();
        let dev_type = if file_type.is_block_device() {
            LinuxDeviceType::B
        } else if file_type.is_char_device() {
            LinuxDeviceType::C
        } else if file_type.is_fifo() {
            LinuxDeviceType::P
        } else {
            eprintln!("Unsupported device type for {}", udev_path.display());
            return Ok(());
        };

        let new_device = LinuxDeviceBuilder::default()
            .path(udev_path)
            .typ(dev_type)
            .major(major)
            .minor(minor)
            .file_mode(metadata.permissions().mode())
            .uid(metadata.uid())
            .gid(metadata.gid())
            .build()?;

        let new_device_cgroup = LinuxDeviceCgroupBuilder::default()
            .allow(true)
            .typ(dev_type)
            .major(major)
            .minor(minor)
            .access("rwm")
            .build()?;

        devices.push(new_device);

        let resources = linux.resources_mut().get_or_insert_with(Default::default);
        let cgroup_devices = resources.devices_mut().get_or_insert_with(Default::default);
        cgroup_devices.push(new_device_cgroup);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::ffi::{OsStr, OsString};
    use std::path::{Path, PathBuf};

    struct MockUdevDevice {
        devnode: Option<PathBuf>,
        properties: HashMap<OsString, OsString>,
    }

    impl UdevDevice for &MockUdevDevice {
        fn devnode(&self) -> Option<&Path> {
            self.devnode.as_deref()
        }

        fn property_value(&self, key: &str) -> Option<&OsStr> {
            self.properties.get(OsStr::new(key)).map(|s| s.as_os_str())
        }
    }

    fn create_null_mock_device() -> MockUdevDevice {
        let mock_device = MockUdevDevice {
            devnode: Some(PathBuf::from("/dev/null")),
            properties: {
                let mut props = HashMap::new();
                props.insert(OsString::from("MAJOR"), OsString::from("1"));
                props.insert(OsString::from("MINOR"), OsString::from("3"));
                props
            },
        };
        mock_device
    }

    #[test]
    fn test_add_udev_device_with_mock() {
        let mut spec = RuntimeSpec::default();
        let mock_device = create_null_mock_device();

        spec.add_udev_device(&mock_device).unwrap();

        let linux = spec.linux().as_ref().unwrap();
        let devices = linux.devices().as_ref().unwrap();
        assert_eq!(devices.len(), 1);
        let device = &devices[0];
        assert_eq!(device.path(), Path::new("/dev/null"));
        assert_eq!(device.major(), 1);
        assert_eq!(device.minor(), 3);
        assert_eq!(device.typ(), LinuxDeviceType::C);
    }

    #[test]
    fn test_add_udev_device_already_exists() {
        let mut spec = RuntimeSpec::default();
        let mock_device = create_null_mock_device();

        spec.add_udev_device(&mock_device).unwrap();
        spec.add_udev_device(&mock_device).unwrap();

        let linux = spec.linux().as_ref().unwrap();
        let devices = linux.devices().as_ref().unwrap();
        assert_eq!(devices.len(), 1);
    }
}
