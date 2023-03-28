mod inquiry_channel_details;
pub use inquiry_channel_details::*;

use crate::model::InquiryChannelId;
use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct InquiryChannel {
    pub id: InquiryChannelId,
    pub details: InquiryChannelDetails,
    pub is_active: bool,
    pub activated_at: NaiveDateTime,
    pub deactivated_at: Option<NaiveDateTime>,
}

impl<'a> Into<database::entities::NewInquiryChannel<'a>> for &'a InquiryChannel {
    fn into(self) -> database::entities::NewInquiryChannel<'a> {
        database::entities::NewInquiryChannel {
            id: &self.id,
            inquiry_channel_type: self.details.as_type(),
            inquiry_channel_type_id: self.details.as_type_id(),
            details: (&self.details).into(),
            is_active: self.is_active,
            activated_at: self.activated_at,
            deactivated_at: self.deactivated_at,
        }
    }
}

impl From<database::entities::InquiryChannel> for InquiryChannel {
    fn from(value: database::entities::InquiryChannel) -> Self {
        Self {
            id: value.id.into(),
            details: value.details.into(),
            is_active: value.is_active,
            activated_at: value.activated_at,
            deactivated_at: value.deactivated_at,
        }
    }
}
