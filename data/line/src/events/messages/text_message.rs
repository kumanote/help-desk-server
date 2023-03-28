use crate::events::messages::{emoji::Emoji, mention::Mention};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TextMessage {
    pub id: String,
    pub text: String,
    pub emojis: Option<Vec<Emoji>>,
    pub mention: Option<Mention>,
}
