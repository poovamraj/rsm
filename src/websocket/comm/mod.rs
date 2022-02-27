use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "message")]
pub enum Comms<'a> {
    RegisterClient {
        uid: &'a str
    },
}
