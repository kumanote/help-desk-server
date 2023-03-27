use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Mention {
    pub mentionees: Vec<Mentionee>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Mentionee {
    pub index: i64,
    pub length: i64,
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
}
