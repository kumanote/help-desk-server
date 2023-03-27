use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Member {
    pub r#type: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}
