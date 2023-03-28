use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LineUserChannelDetails {
    pub line_user_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LineGroupChannelDetails {
    pub line_group_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LineRoomChannelDetails {
    pub line_room_id: String,
}
