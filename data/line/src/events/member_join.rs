use crate::events::{Member, Source};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MemberJoinEvent {
    #[serde(rename = "replyToken")]
    pub reply_token: String,
    pub mode: String,
    pub timestamp: i64,
    pub source: Source,
    pub joined: Joined,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Joined {
    pub members: Vec<Member>,
}
