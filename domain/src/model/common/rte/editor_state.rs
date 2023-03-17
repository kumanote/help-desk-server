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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        let serialized = r#"{"root":{"children":[{"children":[],"direction":"ltr","format":"center","indent":0,"type":"heading","version":1,"tag":"h1"}],"direction":"ltr","format":"","indent":0,"type":"root","version":1}}"#;
        let deserialized: RteEditorState = serde_json::from_str(serialized).unwrap();
        let re_serialized = serde_json::to_string(&deserialized).unwrap();
        let re_deserialized: RteEditorState = serde_json::from_str(&re_serialized).unwrap();
        assert_eq!(deserialized, re_deserialized);
    }
}
