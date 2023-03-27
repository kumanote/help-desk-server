use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LineChannelDetails {
    pub line_user_id: String,
}
