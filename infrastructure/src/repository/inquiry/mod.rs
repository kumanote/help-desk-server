mod job;
pub use job::*;

use chrono::NaiveDateTime;
use database::DbConnection;
use domain::model::{
    InquiryChannel, InquiryContact, InquiryContactChannel, InquiryMessage, InquirySettings,
    InquirySettingsData, InquiryThread,
};
use domain::repository::InquiryRepository;

pub struct InquiryRepositoryImpl;

impl InquiryRepository for InquiryRepositoryImpl {
    type Err = domain::Error;
    type Transaction = DbConnection;

    fn get_settings(
        &self,
        tx: &mut Self::Transaction,
    ) -> Result<Option<InquirySettings>, Self::Err> {
        let entity = database::adapters::inquiry_settings::get(tx)?;
        Ok(entity.map(Into::into))
    }

    fn upsert_settings(
        &self,
        tx: &mut Self::Transaction,
        settings: &mut InquirySettings,
        data: InquirySettingsData,
    ) -> Result<(), Self::Err> {
        settings.data = data;
        database::adapters::inquiry_settings::upsert(tx, settings.into())?;
        Ok(())
    }

    fn create_contact(
        &self,
        tx: &mut Self::Transaction,
        contact: &InquiryContact,
    ) -> Result<(), Self::Err> {
        database::adapters::inquiry_contact::create(tx, contact.into())?;
        Ok(())
    }

    fn get_contact_by_line_user_id(
        &self,
        tx: &mut Self::Transaction,
        line_user_id: &str,
    ) -> Result<Option<InquiryContact>, Self::Err> {
        let entity = database::adapters::inquiry_contact::get_by_line_user_id(tx, line_user_id)?;
        Ok(entity.map(Into::into))
    }

    fn create_channel(
        &self,
        tx: &mut Self::Transaction,
        channel: &InquiryChannel,
    ) -> Result<(), Self::Err> {
        database::adapters::inquiry_channel::create(tx, channel.into())?;
        Ok(())
    }

    fn get_channel_by_type_and_type_id(
        &self,
        tx: &mut Self::Transaction,
        inquiry_channel_type: &str,
        inquiry_channel_type_id: &str,
    ) -> Result<Option<InquiryChannel>, Self::Err> {
        let entity = database::adapters::inquiry_channel::get_by_inquiry_channel_type_and_id(
            tx,
            inquiry_channel_type,
            inquiry_channel_type_id,
        )?;
        Ok(entity.map(Into::into))
    }

    fn update_channel_on_deactivated(
        &self,
        tx: &mut Self::Transaction,
        channel: &mut InquiryChannel,
        deactivated_at: NaiveDateTime,
    ) -> Result<(), Self::Err> {
        let is_active = false;
        let deactivated_at = Some(deactivated_at);
        let _ = database::adapters::inquiry_channel::update_is_canceled_and_canceled_at_by_id(
            tx,
            is_active,
            channel.activated_at,
            deactivated_at,
            &channel.id,
        )?;
        channel.is_active = is_active;
        channel.deactivated_at = deactivated_at;
        Ok(())
    }

    fn update_channel_on_reactivated(
        &self,
        tx: &mut Self::Transaction,
        channel: &mut InquiryChannel,
        activated_at: NaiveDateTime,
    ) -> Result<(), Self::Err> {
        let is_active = true;
        let deactivated_at = None;
        let _ = database::adapters::inquiry_channel::update_is_canceled_and_canceled_at_by_id(
            tx,
            is_active,
            activated_at,
            deactivated_at,
            &channel.id,
        )?;
        channel.is_active = is_active;
        channel.activated_at = activated_at;
        channel.deactivated_at = deactivated_at;
        Ok(())
    }

    fn attach_channel_to_contact(
        &self,
        tx: &mut Self::Transaction,
        contact: &InquiryContact,
        channel: &InquiryChannel,
    ) -> Result<(), Self::Err> {
        let current_max_display_order = database::adapters::inquiry_contact_channel::get_max_display_order_by_inquiry_contact_id(
            tx,
            &contact.id,
        )?;
        let display_order = current_max_display_order.unwrap_or_default() + 1;
        let contact_channel = InquiryContactChannel {
            inquiry_contact_id: contact.id.clone(),
            inquiry_channel_id: channel.id.clone(),
            display_order,
        };
        database::adapters::inquiry_contact_channel::create(tx, (&contact_channel).into())?;
        Ok(())
    }

    fn create_thread(
        &self,
        tx: &mut Self::Transaction,
        thread: &InquiryThread,
    ) -> Result<(), Self::Err> {
        database::adapters::inquiry_thread::create(tx, thread.into())?;
        Ok(())
    }

    fn get_thread_by_type_and_type_id(
        &self,
        tx: &mut Self::Transaction,
        inquiry_thread_type: &str,
        inquiry_thread_type_id: &str,
    ) -> Result<Option<InquiryThread>, Self::Err> {
        let entity = database::adapters::inquiry_thread::get_by_inquiry_channel_type_and_id(
            tx,
            inquiry_thread_type,
            inquiry_thread_type_id,
        )?;
        Ok(entity.map(Into::into))
    }

    fn create_message(
        &self,
        tx: &mut Self::Transaction,
        message: &InquiryMessage,
    ) -> Result<(), Self::Err> {
        database::adapters::inquiry_message::create(tx, message.into())?;
        Ok(())
    }

    fn get_message_by_type_and_type_id(
        &self,
        tx: &mut Self::Transaction,
        inquiry_message_type: &str,
        inquiry_message_type_id: &str,
    ) -> Result<Option<InquiryMessage>, Self::Err> {
        let entity = database::adapters::inquiry_message::get_by_inquiry_message_type_and_id(
            tx,
            inquiry_message_type,
            inquiry_message_type_id,
        )?;
        Ok(entity.map(Into::into))
    }

    fn update_message_on_canceled(
        &self,
        tx: &mut Self::Transaction,
        message: &mut InquiryMessage,
        canceled_at: NaiveDateTime,
    ) -> Result<(), Self::Err> {
        let is_canceled = true;
        let canceled_at = Some(canceled_at);
        let _ = database::adapters::inquiry_message::update_is_canceled_and_canceled_at_by_id(
            tx,
            is_canceled,
            canceled_at,
            &message.id,
        )?;
        message.is_canceled = is_canceled;
        message.canceled_at = canceled_at;
        Ok(())
    }
}
