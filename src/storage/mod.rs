
pub mod in_memory_storage;
pub mod repository;

#[derive(Debug)]
pub struct Device {
    uid: String,
    device_name: String,
}

impl Device {
    pub fn new(uid: String, device_name: String) -> Self {
        Device { uid: uid, device_name: device_name }
    }
}

impl Clone for Device {
    fn clone(&self) -> Self {
        Self { uid: self.uid.clone(), device_name: self.device_name.clone() }
    }
}

pub trait Storage {
    fn insert(&mut self, device: Device);
    fn update(&self, device: Device);
}