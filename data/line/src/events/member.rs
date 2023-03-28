use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Member {
    pub r#type: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}
