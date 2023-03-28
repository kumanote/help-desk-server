mod job;
pub use job::*;

use crate::model::{
    InquiryChannel, InquiryChannelDetails, InquiryContact, InquiryThread, InquiryThreadDetails,
};

pub trait InquiryRepository: Send + Sync + 'static {
    type Err;
    type Transaction;
    fn create_contact(
        &self,
        tx: &mut Self::Transaction,
        contact: &InquiryContact,
    ) -> Result<(), Self::Err>;
    fn get_contact_by_line_user_id(
        &self,
        tx: &mut Self::Transaction,
        line_user_id: &str,
    ) -> Result<Option<InquiryContact>, Self::Err>;
    fn create_channel(
        &self,
        tx: &mut Self::Transaction,
        channel: &InquiryChannel,
    ) -> Result<(), Self::Err>;
    fn get_channel_by_details(
        &self,
        tx: &mut Self::Transaction,
        details: &InquiryChannelDetails,
    ) -> Result<Option<InquiryChannel>, Self::Err>;
    fn attach_channel_to_contact(
        &self,
        tx: &mut Self::Transaction,
        contact: &InquiryContact,
        channel: &InquiryChannel,
    ) -> Result<(), Self::Err>;
    fn create_thread(
        &self,
        tx: &mut Self::Transaction,
        thread: &InquiryThread,
    ) -> Result<(), Self::Err>;
    fn get_thread_by_details(
        &self,
        tx: &mut Self::Transaction,
        details: &InquiryThreadDetails,
    ) -> Result<Option<InquiryThread>, Self::Err>;
}
