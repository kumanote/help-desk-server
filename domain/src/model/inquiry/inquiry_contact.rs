use crate::model::{InquiryContactId, Memo};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

pub type InquiryContactMemo = Memo;
pub type InquiryLineProfile = line::objects::Profile;

#[derive(Debug, Clone)]
pub struct InquiryContact {
    pub id: InquiryContactId,
    pub details: InquiryContactDetails,
    pub memo: Option<InquiryContactMemo>,
    pub created_at: NaiveDateTime,
}

impl<'a> Into<database::entities::NewInquiryContact<'a>> for &'a InquiryContact {
    fn into(self) -> database::entities::NewInquiryContact<'a> {
        database::entities::NewInquiryContact {
            id: &self.id,
            line_user_id: self.details.as_line_user_id(),
            details: (&self.details).into(),
            memo: self.memo.as_deref(),
            created_at: self.created_at,
        }
    }
}

impl From<database::entities::InquiryContact> for InquiryContact {
    fn from(value: database::entities::InquiryContact) -> Self {
        Self {
            id: value.id.into(),
            details: value.details.into(),
            memo: value.memo.map(Into::into),
            created_at: value.created_at,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InquiryContactDetails {
    pub line_profile: Option<InquiryLineProfile>,
}

impl InquiryContactDetails {
    pub fn as_line_user_id(&self) -> Option<&str> {
        if let Some(line_profile) = &self.line_profile {
            line_profile.user_id.as_deref()
        } else {
            None
        }
    }
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
