use crate::events::Source;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct UnsendEvent {
    pub mode: String,
    pub timestamp: i64,
    pub source: Source,
    pub unsend: Unsend,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Unsend {
    #[serde(rename = "messageId")]
    pub message_id: String,
}
