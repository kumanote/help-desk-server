mod tag;
pub use tag::*;

use super::{
    RteChildNode, RteDirection, RteElementFormatType, RteIndent, RteTextsHolder, RteVersion,
};
use serde::{Deserialize, Serialize};

/// @see https://github.com/facebook/lexical/blob/v0.9.1-next.0/packages/lexical-rich-text/src/index.ts#L96
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RteHeadingNode {
    pub children: Vec<RteChildNode>,
    pub direction: Option<RteDirection>,
    pub format: RteElementFormatType,
    pub indent: RteIndent,
    pub version: RteVersion,
    pub tag: RteHeadingNodeTag,
}

impl RteTextsHolder for RteHeadingNode {
    fn texts(&self) -> Vec<&str> {
        let mut results = vec![];
        for child in &self.children {
            results.extend(child.texts());
        }
        results
    }
}
