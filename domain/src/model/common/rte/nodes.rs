mod heading;

pub use heading::*;

use serde::{Deserialize, Serialize};

use super::{RteDirection, RteElementFormatType, RteIndent, RteVersion};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RteRootNode {
    pub children: Vec<RteChildNode>,
    pub direction: Option<RteDirection>,
    pub format: RteElementFormatType,
    pub indent: RteIndent,
    #[serde(rename = "type")]
    pub r#type: String,
    pub version: RteVersion,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum RteChildNode {
    #[serde(rename = "heading")]
    Heading(RteHeadingNode),
}
