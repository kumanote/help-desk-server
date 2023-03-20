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
        let serialized = r#"{
  "root": {
    "children": [
      {
        "type": "list",
        "children": [
          {
            "type": "listitem",
            "children": [
              {
                "type": "text",
                "detail": 0,
                "format": 16,
                "mode": "normal",
                "style": "",
                "text": "aaa",
                "version": 1
              }
            ],
            "direction": "ltr",
            "format": "",
            "indent": 0,
            "version": 1,
            "value": 1
          }
        ],
        "direction": "ltr",
        "format": "",
        "indent": 0,
        "version": 1,
        "listType": "bullet",
        "start": 1,
        "tag": "ul"
      },
      {
        "type": "code",
        "children": [
          {
            "type": "code-highlight",
            "detail": 0,
            "format": 0,
            "mode": "normal",
            "style": "",
            "text": "a",
            "version": 1
          }
        ],
        "direction": "ltr",
        "format": "",
        "indent": 0,
        "version": 1,
        "language": "plain"
      }
    ],
    "direction": "ltr",
    "format": "",
    "indent": 0,
    "type": "root",
    "version": 1
  }
}"#;
        let json_value: serde_json::Value = serde_json::from_str(serialized).unwrap();
        let deserialized: RteEditorState = serde_json::from_str(serialized).unwrap();
        let re_serialized: serde_json::Value = deserialized.into();
        assert_eq!(json_value, re_serialized);
    }
}
