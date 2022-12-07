use serde::{Deserialize, Serialize};

/// A plain object representation of an entity attribution.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RteEntityRange {
    pub key: u32,
    pub offset: u32,
    pub length: u32,
}
