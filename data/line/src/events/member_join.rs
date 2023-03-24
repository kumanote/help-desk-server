use crate::events::{Member, Source};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct MemberJoinEvent {
    #[serde(rename = "replyToken")]
    pub reply_token: String,
    pub mode: String,
    pub timestamp: i64,
    pub source: Source,
    pub joined: Joined,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Joined {
    pub members: Vec<Member>,
}
