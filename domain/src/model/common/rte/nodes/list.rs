mod list_type;
mod tag;

pub use list_type::*;
pub use tag::*;

use super::{
    RteChildNode, RteDirection, RteElementFormatType, RteIndent, RteTextsHolder, RteVersion,
};
use serde::{Deserialize, Serialize};

pub type RteListStartCount = u32;
pub type RteListItemValue = u32;

/// @see https://github.com/facebook/lexical/blob/v0.9.1-next.0/packages/lexical-list/src/LexicalListNode.ts#L35
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RteListNode {
    pub children: Vec<RteChildNode>,
    pub direction: Option<RteDirection>,
    pub format: RteElementFormatType,
    pub indent: RteIndent,
    pub version: RteVersion,
    #[serde(rename = "listType")]
    pub list_type: RteListType,
    /// an ordered list starts its count
    pub start: RteListStartCount,
    pub tag: RteListNodeTag,
}

impl RteTextsHolder for RteListNode {
    fn texts(&self) -> Vec<&str> {
        let mut results = vec![];
        for child in &self.children {
            results.extend(child.texts());
        }
        results
    }
}

/// @see https://github.com/facebook/lexical/blob/v0.9.1-next.0/packages/lexical-list/src/LexicalListItemNode.ts#L49
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RteListItemNode {
    pub children: Vec<RteChildNode>,
    pub direction: Option<RteDirection>,
    pub format: RteElementFormatType,
    pub indent: RteIndent,
    pub version: RteVersion,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checked: Option<bool>,
    pub value: RteListItemValue,
}

impl RteTextsHolder for RteListItemNode {
    fn texts(&self) -> Vec<&str> {
        let mut results = vec![];
        for child in &self.children {
            results.extend(child.texts());
        }
        results
    }
}
