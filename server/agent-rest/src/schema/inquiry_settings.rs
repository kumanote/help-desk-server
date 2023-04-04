use domain::model;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct InquirySettings {
    pub line: InquiryLineSettings,
    pub notification: InquiryNotificationSettings,
}

#[derive(Debug, Serialize)]
pub struct InquiryLineSettings {
    pub enabled: bool,
    pub friend_url: Option<String>,
    pub friend_qr_code_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct InquiryNotificationSettings {
    pub slack_webhook_url: Option<String>,
}

impl From<model::InquirySettings> for InquirySettings {
    fn from(value: model::InquirySettings) -> Self {
        Self {
            line: value.data.line.into(),
            notification: value.data.notification.into(),
        }
    }
}

impl From<model::InquiryLineSettings> for InquiryLineSettings {
    fn from(value: model::InquiryLineSettings) -> Self {
        Self {
            enabled: value.enabled,
            friend_url: value.friend_url.map(Into::into),
            friend_qr_code_url: value.friend_qr_code_url.map(Into::into),
        }
    }
}

impl From<model::InquiryNotificationSettings> for InquiryNotificationSettings {
    fn from(value: model::InquiryNotificationSettings) -> Self {
        Self {
            slack_webhook_url: value.slack_webhook_url.map(Into::into),
        }
    }
}
