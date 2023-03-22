use super::{RteTextsHolder, RteVersion};
use serde::{Deserialize, Serialize};

/// @see https://github.com/facebook/lexical/blob/v0.9.1-next.0/packages/lexical/src/nodes/LexicalLineBreakNode.ts#L20
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RteLinebreakNode {
    pub version: RteVersion,
}

impl RteTextsHolder for RteLinebreakNode {
    fn texts(&self) -> Vec<&str> {
        vec![]
    }
}
