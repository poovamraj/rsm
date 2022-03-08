use std::sync::mpsc::{channel, Receiver, Sender};

use super::{in_memory_storage::InMemoryDb, Device, Storage};

pub enum StorageOps {
    InsertDevice(Device),
}

pub fn get_storage_instance() -> Sender<StorageOps> {
    let db = InMemoryDb::new();
    let (tx, rx) = channel();
    tokio::spawn(receive_storage_ops(db, rx));
    tx
}

async fn receive_storage_ops(mut db: InMemoryDb, rx: Receiver<StorageOps>) {
    while let Ok(msg) = rx.recv() {
        match msg {
            StorageOps::InsertDevice(device) => db.insert(device),
        }
    }
}
