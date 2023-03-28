mod line;
pub use self::line::*;

use serde::{Deserialize, Serialize};

const TYPE_LINE: &'static str = "line";

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum InquiryMessageDetails {
    Line(LineMessageDetails),
}

impl InquiryMessageDetails {
    pub fn as_type(&self) -> &'static str {
        match self {
            Self::Line(_) => TYPE_LINE,
        }
    }
}

impl From<serde_json::Value> for InquiryMessageDetails {
    fn from(value: serde_json::Value) -> Self {
        serde_json::from_value(value).unwrap()
    }
}

impl Into<serde_json::Value> for InquiryMessageDetails {
    fn into(self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}

impl Into<serde_json::Value> for &InquiryMessageDetails {
    fn into(self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}
