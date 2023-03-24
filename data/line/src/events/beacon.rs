use crate::events::Source;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct BeaconEvent {
    #[serde(rename = "replyToken")]
    pub reply_token: String,
    pub mode: String,
    pub timestamp: i64,
    pub source: Source,
    pub beacon: Beacon,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Beacon {
    pub hwid: String,
    pub r#type: String,
    pub dm: Option<String>,
}
