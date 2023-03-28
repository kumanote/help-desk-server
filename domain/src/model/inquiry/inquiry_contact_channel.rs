use crate::model::{InquiryChannelId, InquiryContactId};

#[derive(Debug, Clone)]
pub struct InquiryContactChannel {
    pub inquiry_contact_id: InquiryContactId,
    pub inquiry_channel_id: InquiryChannelId,
    pub display_order: u32,
}

impl<'a> Into<database::entities::NewInquiryContactChannel<'a>> for &'a InquiryContactChannel {
    fn into(self) -> database::entities::NewInquiryContactChannel<'a> {
        database::entities::NewInquiryContactChannel {
            inquiry_contact_id: &self.inquiry_contact_id,
            inquiry_channel_id: &self.inquiry_channel_id,
            display_order: self.display_order,
        }
    }
}

impl From<database::entities::InquiryContactChannel> for InquiryContactChannel {
    fn from(value: database::entities::InquiryContactChannel) -> Self {
        Self {
            inquiry_contact_id: value.inquiry_contact_id.into(),
            inquiry_channel_id: value.inquiry_channel_id.into(),
            display_order: value.display_order,
        }
    }
}

impl From<&database::entities::InquiryContactChannel> for InquiryContactChannel {
    fn from(value: &database::entities::InquiryContactChannel) -> Self {
        Self {
            inquiry_contact_id: value.inquiry_contact_id.clone().into(),
            inquiry_channel_id: value.inquiry_channel_id.clone().into(),
            display_order: value.display_order,
        }
    }
}
