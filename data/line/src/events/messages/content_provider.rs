use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ContentProvider {
    #[serde(flatten)]
    pub r#type: ContentProviderType,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "type")]
pub enum ContentProviderType {
    #[serde(rename = "external")]
    External(External),
    #[serde(other)]
    Other,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct External {
    #[serde(rename = "originalContentUrl")]
    pub original_content_url: String,
    #[serde(rename = "previewImageUrl")]
    pub preview_image_url: String,
}
