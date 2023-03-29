use crate::model::{InquiryContact, InquiryMessage};

pub trait InquirySearchRepository: Send + Sync + 'static {
    type Err;
    fn upsert_inquiry_contact(&self, contact: &InquiryContact) -> Result<(), Self::Err>;
    fn delete_inquiry_contact(&self, contact: &InquiryContact) -> Result<(), Self::Err>;
    fn upsert_inquiry_message(&self, message: &InquiryMessage) -> Result<(), Self::Err>;
    fn delete_inquiry_message(&self, message: &InquiryMessage) -> Result<(), Self::Err>;
}
