use super::RteInlineStyleType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RteInlineStyleRange {
    pub style: RteInlineStyleType,
    pub offset: u32,
    pub length: u32,
}
