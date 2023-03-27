use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Source {
    #[serde(flatten)]
    pub r#type: SourceType,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum SourceType {
    #[serde(rename = "user")]
    User(User),
    #[serde(rename = "group")]
    Group(Group),
    #[serde(rename = "room")]
    Room(Room),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Group {
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Room {
    #[serde(rename = "roomId")]
    pub room_id: String,
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
}
