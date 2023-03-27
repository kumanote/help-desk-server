use crate::events::Source;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct AccountLinkEvent {
    #[serde(rename = "replyToken")]
    pub reply_token: String,
    pub mode: String,
    pub timestamp: i64,
    pub source: Source,
    pub link: Link,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Link {
    pub result: String,
    pub nonce: String,
}
