use super::{RteTextsHolder, RteVersion};
use serde::{Deserialize, Serialize};

pub type RteImageSize = u32;

/// @see https://github.com/kumanote/help-desk-web/blob/main/src/components/forms/editors/RichTextEditor/nodes/ImageNode.tsx#L30
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RteImageNode {
    pub src: String,
    #[serde(rename = "altText")]
    pub alt_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<RteImageSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<RteImageSize>,
    #[serde(rename = "maxWidth", skip_serializing_if = "Option::is_none")]
    pub max_width: Option<RteImageSize>,
    pub version: RteVersion,
}

impl RteTextsHolder for RteImageNode {
    fn texts(&self) -> Vec<&str> {
        vec![]
    }
}
