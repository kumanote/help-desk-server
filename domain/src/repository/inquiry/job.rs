use crate::model::InquiryIncomingEvent;

/// Repository for inquiry background tasks.
pub trait InquiryJobRepository: Send + Sync + 'static {
    type Err;
    fn register(&self, inquiry_incoming_event: &InquiryIncomingEvent) -> Result<(), Self::Err>;
}
