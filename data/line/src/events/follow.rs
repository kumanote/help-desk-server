use crate::events::Source;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FollowEvent {
    #[serde(rename = "replyToken")]
    pub reply_token: Option<String>,
    pub mode: String,
    pub timestamp: i64,
    pub source: Source,
}
