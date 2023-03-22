use super::{
    RteChildNode, RteDirection, RteElementFormatType, RteIndent, RteTextDetailType,
    RteTextFormatType, RteTextModeType, RteTextsHolder, RteVersion,
};
use serde::{Deserialize, Serialize};

/// @see https://github.com/facebook/lexical/blob/v0.9.1-next.0/packages/lexical-code/src/CodeNode.ts#L53
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RteCodeNode {
    pub children: Vec<RteChildNode>,
    pub direction: Option<RteDirection>,
    pub format: RteElementFormatType,
    pub indent: RteIndent,
    pub version: RteVersion,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

impl RteTextsHolder for RteCodeNode {
    fn texts(&self) -> Vec<&str> {
        let mut results = vec![];
        for child in &self.children {
            results.extend(child.texts());
        }
        results
    }
}

/// @see https://github.com/facebook/lexical/blob/v0.9.1-next.0/packages/lexical-code/src/CodeHighlightNode.ts#L49
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RteCodeHighlightNode {
    #[serde(rename = "highlightType", skip_serializing_if = "Option::is_none")]
    pub highlight_type: Option<String>,
    pub detail: RteTextDetailType,
    pub format: RteTextFormatType,
    pub mode: RteTextModeType,
    pub style: String,
    pub text: String,
    pub version: RteVersion,
}

impl RteTextsHolder for RteCodeHighlightNode {
    fn texts(&self) -> Vec<&str> {
        vec![&self.text]
    }
}
