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
