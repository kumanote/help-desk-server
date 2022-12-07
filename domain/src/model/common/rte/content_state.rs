use super::{RteContentBlock, RteEntity};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RteContentState {
    pub blocks: Vec<RteContentBlock>,
    #[serde(rename = "entityMap")]
    pub entity_map: BTreeMap<String, RteEntity>,
}
