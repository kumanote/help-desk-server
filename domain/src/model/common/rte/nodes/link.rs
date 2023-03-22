use super::{
    RteChildNode, RteDirection, RteElementFormatType, RteIndent, RteTextsHolder, RteVersion,
};
use serde::{Deserialize, Serialize};

/// @see https://github.com/facebook/lexical/blob/v0.9.1-next.0/packages/lexical-link/src/index.ts#L39
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RteLinkNode {
    pub children: Vec<RteChildNode>,
    pub direction: Option<RteDirection>,
    pub format: RteElementFormatType,
    pub indent: RteIndent,
    pub version: RteVersion,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

impl RteTextsHolder for RteLinkNode {
    fn texts(&self) -> Vec<&str> {
        let mut results = vec![];
        for child in &self.children {
            results.extend(child.texts());
        }
        results
    }
}
