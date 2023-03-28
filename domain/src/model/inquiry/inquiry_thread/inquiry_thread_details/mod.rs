mod line;
pub use self::line::*;

use serde::{Deserialize, Serialize};

const TYPE_LINE_USER: &'static str = "line_user";
const TYPE_LINE_GROUP: &'static str = "line_group";
const TYPE_LINE_ROOM: &'static str = "line_room";

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum InquiryThreadDetails {
    LineUser(LineUserThreadDetails),
    LineGroup(LineGroupThreadDetails),
    LineRoom(LineRoomThreadDetails),
}

impl InquiryThreadDetails {
    pub fn as_type(&self) -> &'static str {
        match self {
            Self::LineUser(_) => TYPE_LINE_USER,
            Self::LineGroup(_) => TYPE_LINE_GROUP,
            Self::LineRoom(_) => TYPE_LINE_ROOM,
        }
    }

    pub fn as_type_id(&self) -> &str {
        match self {
            Self::LineUser(detail) => &detail.line_user_id,
            Self::LineGroup(detail) => &detail.line_group_id,
            Self::LineRoom(detail) => &detail.line_room_id,
        }
    }
}

impl From<serde_json::Value> for InquiryThreadDetails {
    fn from(value: serde_json::Value) -> Self {
        serde_json::from_value(value).unwrap()
    }
}

impl Into<serde_json::Value> for InquiryThreadDetails {
    fn into(self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}

impl Into<serde_json::Value> for &InquiryThreadDetails {
    fn into(self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}
