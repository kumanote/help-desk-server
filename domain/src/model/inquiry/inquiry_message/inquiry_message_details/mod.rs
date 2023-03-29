mod line;
pub use self::line::*;

use ::line::events::messages::MessageType;
use serde::{Deserialize, Serialize};

pub const INQUIRY_MESSAGE_TYPE_LINE: &'static str = "line";

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum InquiryMessageDetails {
    Line(LineMessageDetails),
}

impl InquiryMessageDetails {
    pub fn as_type(&self) -> &'static str {
        match self {
            Self::Line(_) => INQUIRY_MESSAGE_TYPE_LINE,
        }
    }

    pub fn as_type_id(&self) -> &str {
        match self {
            Self::Line(detail) => match &detail.message.r#type {
                MessageType::TextMessage(inner) => &inner.id,
                MessageType::ImageMessage(inner) => &inner.id,
                MessageType::VideoMessage(inner) => &inner.id,
                MessageType::AudioMessage(inner) => &inner.id,
                MessageType::FileMessage(inner) => &inner.id,
                MessageType::LocationMessage(inner) => &inner.id,
                MessageType::StickerMessage(inner) => &inner.id,
            },
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

impl Into<search::entities::InquiryMessageDetailItem> for &InquiryMessageDetails {
    fn into(self) -> search::entities::InquiryMessageDetailItem {
        match &self {
            InquiryMessageDetails::Line(detail) => match &detail.message.r#type {
                MessageType::TextMessage(inner) => {
                    search::entities::InquiryMessageDetailItem::Line(
                        search::entities::InquiryMessageLineDetail {
                            text: Some(inner.text.clone()),
                        },
                    )
                },
                MessageType::FileMessage(inner) => {
                    search::entities::InquiryMessageDetailItem::Line(
                        search::entities::InquiryMessageLineDetail {
                            text: Some(inner.file_name.clone()),
                        },
                    )
                },
                MessageType::LocationMessage(inner) => {
                    search::entities::InquiryMessageDetailItem::Line(
                        search::entities::InquiryMessageLineDetail {
                            text: Some(inner.address.clone()),
                        },
                    )
                },
                _ => search::entities::InquiryMessageDetailItem::Line(
                    search::entities::InquiryMessageLineDetail::default(),
                ),
            },
        }
    }
}
