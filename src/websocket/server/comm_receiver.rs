use std::sync::mpsc::Sender;

use serde_json;

use crate::{
    storage::{repository::StorageOps, Device},
    websocket::comm::Comms,
};

pub fn comm_receiver(storage_sender: Sender<StorageOps>, message: String) {
    match serde_json::from_str::<Comms>(&message) {
        Ok(comm) => match comm {
            Comms::RegisterClient { uid, device_name } => {
                let _ =
                    storage_sender.send(StorageOps::InsertDevice(Device::new(uid, device_name)));
            }
        },
        Err(_) => {
            println!("Unknown message sent");
        }
    }
}
