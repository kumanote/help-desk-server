use crate::events::Source;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UnsendEvent {
    pub mode: String,
    pub timestamp: i64,
    pub source: Source,
    pub unsend: Unsend,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Unsend {
    #[serde(rename = "messageId")]
    pub message_id: String,
}
