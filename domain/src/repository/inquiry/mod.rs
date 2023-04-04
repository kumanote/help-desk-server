mod job;
pub use job::*;

use crate::model::{
    InquiryChannel, InquiryContact, InquiryMessage, InquirySettings, InquirySettingsData,
    InquiryThread,
};
use chrono::NaiveDateTime;

pub trait InquiryRepository: Send + Sync + 'static {
    type Err;
    type Transaction;
    fn get_settings(
        &self,
        tx: &mut Self::Transaction,
    ) -> Result<Option<InquirySettings>, Self::Err>;
    fn upsert_settings(
        &self,
        tx: &mut Self::Transaction,
        settings: &mut InquirySettings,
        data: InquirySettingsData,
    ) -> Result<(), Self::Err>;
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
    fn get_channel_by_type_and_type_id(
        &self,
        tx: &mut Self::Transaction,
        inquiry_channel_type: &str,
        inquiry_channel_type_id: &str,
    ) -> Result<Option<InquiryChannel>, Self::Err>;
    fn update_channel_on_deactivated(
        &self,
        tx: &mut Self::Transaction,
        channel: &mut InquiryChannel,
        deactivated_at: NaiveDateTime,
    ) -> Result<(), Self::Err>;
    fn update_channel_on_reactivated(
        &self,
        tx: &mut Self::Transaction,
        channel: &mut InquiryChannel,
        activated_at: NaiveDateTime,
    ) -> Result<(), Self::Err>;
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
    fn get_thread_by_type_and_type_id(
        &self,
        tx: &mut Self::Transaction,
        inquiry_thread_type: &str,
        inquiry_thread_type_id: &str,
    ) -> Result<Option<InquiryThread>, Self::Err>;
    fn create_message(
        &self,
        tx: &mut Self::Transaction,
        message: &InquiryMessage,
    ) -> Result<(), Self::Err>;
    fn get_message_by_type_and_type_id(
        &self,
        tx: &mut Self::Transaction,
        inquiry_message_type: &str,
        inquiry_message_type_id: &str,
    ) -> Result<Option<InquiryMessage>, Self::Err>;
    fn update_message_on_canceled(
        &self,
        tx: &mut Self::Transaction,
        message: &mut InquiryMessage,
        canceled_at: NaiveDateTime,
    ) -> Result<(), Self::Err>;
}
