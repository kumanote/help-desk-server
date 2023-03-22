mod mode_type;
pub use mode_type::*;

use super::{RteTextsHolder, RteVersion};
use serde::{Deserialize, Serialize};

pub type RteTextDetailType = u32;
pub type RteTextFormatType = u32;

/// @see https://github.com/facebook/lexical/blob/v0.9.1-next.0/packages/lexical/src/nodes/LexicalTextNode.ts#L69
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RteTextNode {
    pub detail: RteTextDetailType,
    pub format: RteTextFormatType,
    pub mode: RteTextModeType,
    pub style: String,
    pub text: String,
    pub version: RteVersion,
}

impl RteTextsHolder for RteTextNode {
    fn texts(&self) -> Vec<&str> {
        vec![&self.text]
    }
}
