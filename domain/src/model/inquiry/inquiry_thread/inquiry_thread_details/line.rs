use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LineUserThreadDetails {
    pub line_user_id: String,
    pub message: line::events::messages::Message,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LineGroupThreadDetails {
    pub line_group_id: String,
    pub message: line::events::messages::Message,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LineRoomThreadDetails {
    pub line_room_id: String,
    pub message: line::events::messages::Message,
}
