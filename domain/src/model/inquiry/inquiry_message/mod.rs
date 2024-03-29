mod inquiry_message_details;
mod inquiry_message_speaker;

pub use inquiry_message_details::*;
pub use inquiry_message_speaker::*;

use crate::model::{InquiryMessageId, InquiryThreadId};
use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct InquiryMessage {
    pub id: InquiryMessageId,
    pub inquiry_thread_id: InquiryThreadId,
    pub reply_inquiry_message_id: Option<InquiryMessageId>,
    pub details: InquiryMessageDetails,
    pub speaker: InquiryMessageSpeaker,
    pub is_canceled: bool,
    pub created_at: NaiveDateTime,
    pub canceled_at: Option<NaiveDateTime>,
}

impl<'a> Into<database::entities::NewInquiryMessage<'a>> for &'a InquiryMessage {
    fn into(self) -> database::entities::NewInquiryMessage<'a> {
        database::entities::NewInquiryMessage {
            id: &self.id,
            inquiry_thread_id: &self.inquiry_thread_id,
            reply_inquiry_message_id: self.reply_inquiry_message_id.as_deref(),
            inquiry_message_type: self.details.as_type(),
            inquiry_message_type_id: self.details.as_type_id(),
            details: (&self.details).into(),
            speaker_type: self.speaker.as_type(),
            inquiry_contact_id: self.speaker.inquiry_contact_id().map(AsRef::as_ref),
            agent_id: self.speaker.agent_id().map(AsRef::as_ref),
            is_canceled: self.is_canceled,
            created_at: self.created_at,
            canceled_at: self.canceled_at,
        }
    }
}

impl From<database::entities::InquiryMessage> for InquiryMessage {
    fn from(value: database::entities::InquiryMessage) -> Self {
        Self {
            id: value.id.into(),
            inquiry_thread_id: value.inquiry_thread_id.into(),
            reply_inquiry_message_id: value.reply_inquiry_message_id.map(Into::into),
            details: value.details.into(),
            speaker: InquiryMessageSpeaker::new(
                value.speaker_type,
                value.inquiry_contact_id,
                value.agent_id,
            ),
            is_canceled: value.is_canceled,
            created_at: value.created_at,
            canceled_at: value.canceled_at,
        }
    }
}

impl Into<search::entities::InquiryMessage> for &InquiryMessage {
    fn into(self) -> search::entities::InquiryMessage {
        search::entities::InquiryMessage {
            id: self.id.to_string(),
            details: (&self.details).into(),
        }
    }
}
