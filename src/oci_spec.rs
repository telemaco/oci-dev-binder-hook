use oci_spec::runtime::{
    LinuxDeviceBuilder, LinuxDeviceCgroupBuilder, LinuxDeviceType, Spec as RuntimeSpec,
};
use std::error::Error;
use std::os::unix::fs::{FileTypeExt, MetadataExt, PermissionsExt};
use udev::Device;

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
    fn add_udev_device(self, device: Device) -> Result<RuntimeSpec, Box<dyn Error>>;
}

impl RuntimeSpecUdev for RuntimeSpec {
    fn add_udev_device(mut self, device: Device) -> Result<RuntimeSpec, Box<dyn Error>> {
        let udev_path = match device.devnode() {
            Some(path) => path,
            None => return Ok(self),
        };

        let linux = self.linux_mut().get_or_insert_with(Default::default);
        let devices = linux.devices_mut().get_or_insert_with(Default::default);

        // Check if device is already in the list and return early if exists
        if devices.iter().any(|d| d.path() == udev_path) {
            return Ok(self);
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
            return Ok(self);
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

        Ok(self)
    }
}
