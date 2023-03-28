use crate::model::InquiryContact;

pub trait InquirySearchRepository: Send + Sync + 'static {
    type Err;
    fn upsert_inquiry_contact(&self, contact: &InquiryContact) -> Result<(), Self::Err>;
}
