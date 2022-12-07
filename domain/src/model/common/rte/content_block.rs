use super::{RteBlockType, RteEntityRange, RteInlineStyleRange};
use serde::{Deserialize, Serialize};

/// Rich text editor's content block
/// compatible with Dart.js ContentBlock
/// @see https://github.com/DefinitelyTyped/DefinitelyTyped/blob/master/types/draft-js/index.d.ts for more information.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RteContentBlock {
    pub key: String,
    pub text: String,
    #[serde(rename = "type")]
    pub r#type: RteBlockType,
    pub depth: u32,
    #[serde(rename = "inlineStyleRanges")]
    pub inline_style_ranges: Vec<RteInlineStyleRange>,
    #[serde(rename = "entityRanges")]
    pub entity_ranges: Vec<RteEntityRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Map<String, serde_json::Value>>,
}
