use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Mention {
    pub mentionees: Vec<Mentionee>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Mentionee {
    pub index: i64,
    pub length: i64,
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
}
