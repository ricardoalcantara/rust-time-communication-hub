use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct HubEventMessage {
    id: String,
    payload: String,
    read: bool,
}

impl HubEventMessage {
    pub fn new(id: String, payload: String, read: bool) -> HubEventMessage {
        HubEventMessage { id, payload, read }
    }
}
