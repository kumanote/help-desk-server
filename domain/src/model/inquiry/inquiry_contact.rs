use crate::model::{InquiryContactId, Memo};
use serde::{Deserialize, Serialize};

pub type InquiryContactMemo = Memo;
pub type InquiryLineProfile = line::objects::Profile;

#[derive(Debug, Clone)]
pub struct InquiryContact {
    pub id: InquiryContactId,
    pub details: InquiryContactDetails,
    pub memo: Option<InquiryContactMemo>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InquiryContactDetails {
    pub line_profile: Option<InquiryLineProfile>,
}

impl From<serde_json::Value> for InquiryContactDetails {
    fn from(value: serde_json::Value) -> Self {
        serde_json::from_value(value).unwrap()
    }
}

impl Into<serde_json::Value> for InquiryContactDetails {
    fn into(self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}

impl Into<serde_json::Value> for &InquiryContactDetails {
    fn into(self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}
