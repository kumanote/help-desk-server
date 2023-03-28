use crate::events::{Member, Source};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MemberLeaveEvent {
    pub mode: String,
    pub timestamp: i64,
    pub source: Source,
    pub left: Left,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Left {
    pub members: Vec<Member>,
}
