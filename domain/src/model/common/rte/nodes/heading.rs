mod tag;
pub use tag::*;

use super::{RteChildNode, RteDirection, RteElementFormatType, RteIndent, RteVersion};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RteHeadingNode {
    pub children: Vec<RteChildNode>,
    pub direction: Option<RteDirection>,
    pub format: RteElementFormatType,
    pub indent: RteIndent,
    pub version: RteVersion,
    pub tag: RteHeadingNodeTag,
}
