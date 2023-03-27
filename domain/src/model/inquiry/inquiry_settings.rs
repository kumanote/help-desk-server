use crate::model::{InquirySettingsId, Url};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct InquirySettings {
    pub id: InquirySettingsId,
    pub data: InquirySettingsData,
}

impl Default for InquirySettings {
    fn default() -> Self {
        Self {
            id: InquirySettingsId::generate(),
            data: InquirySettingsData::default(),
        }
    }
}

impl<'a> Into<database::entities::NewInquirySettings<'a>> for &'a InquirySettings {
    fn into(self) -> database::entities::NewInquirySettings<'a> {
        database::entities::NewInquirySettings {
            id: &self.id,
            data: (&self.data).into(),
        }
    }
}

impl From<database::entities::InquirySettings> for InquirySettings {
    fn from(value: database::entities::InquirySettings) -> Self {
        Self {
            id: value.id.into(),
            data: value.data.into(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InquirySettingsData {
    pub line: InquiryLineSettings,
    pub notification: InquiryNotificationSettings,
}

impl From<serde_json::Value> for InquirySettingsData {
    fn from(value: serde_json::Value) -> Self {
        serde_json::from_value(value).unwrap()
    }
}

impl Into<serde_json::Value> for InquirySettingsData {
    fn into(self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}

impl Into<serde_json::Value> for &InquirySettingsData {
    fn into(self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InquiryLineSettings {
    pub enabled: bool,
    /// URL for end user can add friend by tap/click the link.
    /// such as "https://lin.ee/xxxxxxx"
    pub friend_url: Option<Url>,
    /// QR code URL for end user can add friend
    /// such as "https://qr-official.line.me/gs/M_xxxxxxxx_GW.png"
    pub friend_qr_code_url: Option<Url>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InquiryNotificationSettings {
    /// Slack incoming webhook url
    /// such as "https://hooks.slack.com/services/T00000000/B00000000/XXXXXXXXXXXXXXXXXXXXXXXX"
    pub slack_webhook_url: Option<Url>,
}
