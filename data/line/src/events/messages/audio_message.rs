use crate::events::messages::content_provider::ContentProvider;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AudioMessage {
    pub id: String,
    pub duration: i64,
    #[serde(rename = "contentProvider")]
    pub content_provider: ContentProvider,
}
