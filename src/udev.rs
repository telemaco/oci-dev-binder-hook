use std::ffi::OsStr;
use std::path::Path;

pub trait UdevDevice {
    fn devnode(&self) -> Option<&Path>;
    fn property_value(&self, key: &str) -> Option<&OsStr>;
}

impl UdevDevice for udev::Device {
    fn devnode(&self) -> Option<&Path> {
        self.devnode()
    }

    fn property_value(&self, key: &str) -> Option<&OsStr> {
        self.property_value(key)
    }
}
