use super::RteRootNode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RteEditorState {
    pub root: RteRootNode,
}

impl From<serde_json::Value> for RteEditorState {
    fn from(value: serde_json::Value) -> Self {
        serde_json::from_value(value).unwrap()
    }
}

impl Into<serde_json::Value> for RteEditorState {
    fn into(self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}

impl Into<serde_json::Value> for &RteEditorState {
    fn into(self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}
