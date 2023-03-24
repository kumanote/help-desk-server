use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Source {
    #[serde(flatten)]
    pub r#type: SourceType,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "type")]
pub enum SourceType {
    #[serde(rename = "user")]
    User(User),
    #[serde(rename = "group")]
    Group(Group),
    #[serde(rename = "room")]
    Room(Room),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Group {
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Room {
    #[serde(rename = "roomId")]
    pub room_id: String,
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
}
