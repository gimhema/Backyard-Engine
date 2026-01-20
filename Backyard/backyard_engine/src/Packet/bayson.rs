// Backyard Json Packet Module
// Provides functionality for handling JSON packets in the Backyard system.
use serde::{Deserialize, Serialize};
use serde_json::Result as JsonResult;

#[derive(Serialize, Deserialize, Debug)]
pub struct BaysonPacket {
    pub header: String,
    pub payload: serde_json::Value,
}

