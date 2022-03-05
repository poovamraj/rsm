use std::{collections::HashMap};
use super::{Device, Storage};

#[derive(Debug)]
pub struct InMemoryDb {
    map: HashMap<String, Device>
}

impl InMemoryDb {
    pub fn new() -> Self {
        InMemoryDb {
            map: HashMap::new()
        }
    }
}

impl Storage for InMemoryDb {
    fn insert(self: &mut InMemoryDb, device: Device) {
        let c = device.clone();
        self.map.insert(c.uid, device);
        dbg!(self);
    }

    fn update(&self, _: Device) {
         
    }
}