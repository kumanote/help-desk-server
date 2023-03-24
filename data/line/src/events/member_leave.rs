use crate::events::{Member, Source};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct MemberLeaveEvent {
    pub mode: String,
    pub timestamp: i64,
    pub source: Source,
    pub left: Left,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Left {
    pub members: Vec<Member>,
}
