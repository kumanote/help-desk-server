use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LineThreadDetails {
    pub line_user_id: String,
    pub message: line::events::messages::Message,
}
