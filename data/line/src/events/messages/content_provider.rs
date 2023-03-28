use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContentProvider {
    #[serde(flatten)]
    pub r#type: ContentProviderType,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum ContentProviderType {
    #[serde(rename = "external")]
    External(External),
    #[serde(other)]
    Other,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct External {
    #[serde(rename = "originalContentUrl")]
    pub original_content_url: String,
    #[serde(rename = "previewImageUrl")]
    pub preview_image_url: String,
}
