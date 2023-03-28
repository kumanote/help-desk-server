use crate::events::messages::content_provider::ContentProvider;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImageMessage {
    pub id: String,
    #[serde(rename = "contentProvider")]
    pub content_provider: ContentProvider,
}
