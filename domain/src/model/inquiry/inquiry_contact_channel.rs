use crate::model::{InquiryChannelId, InquiryContactId};

#[derive(Debug, Clone)]
pub struct InquiryContactChannel {
    pub inquiry_contact_id: InquiryContactId,
    pub inquiry_channel_id: InquiryChannelId,
    pub display_order: u32,
}
