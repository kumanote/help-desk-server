use super::{RteContentBlock, RteEntity};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RteContentState {
    pub blocks: Vec<RteContentBlock>,
    #[serde(rename = "entityMap")]
    pub entity_map: BTreeMap<String, RteEntity>,
}

impl From<serde_json::Value> for RteContentState {
    fn from(value: serde_json::Value) -> Self {
        serde_json::from_value(value).unwrap()
    }
}

impl Into<serde_json::Value> for RteContentState {
    fn into(self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}

impl Into<serde_json::Value> for &RteContentState {
    fn into(self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}
