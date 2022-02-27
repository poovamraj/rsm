use serde_json;

use crate::websocket::comm::{Comms};

pub fn comm_receiver(message: String) {
    if let Ok(comm) = serde_json::from_str::<Comms>(&message) {
        match comm {
            Comms::RegisterClient { uid } => {
                dbg!(uid);
            },
        }
    } else {
        println!("Unknown message sent");
    }
}