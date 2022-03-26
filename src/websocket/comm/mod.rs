use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "message")]
pub enum Comms {
    RegisterClient { uid: String, device_name: String },
    ConnectUi
}
