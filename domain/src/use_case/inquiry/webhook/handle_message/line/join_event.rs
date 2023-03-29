use crate::{
    model::{
        InquiryChannel, InquiryChannelDetails, InquiryChannelId, InquiryContact,
        InquiryContactDetails, InquiryContactId, LineGroupChannelDetails, LineRoomChannelDetails,
    },
    repository::{InquiryRepository, InquirySearchRepository, LineRepository},
    Error, Result,
};
use chrono::NaiveDateTime;

pub type HandleLineJoinEventUseCaseInput = line::events::JoinEvent;

pub trait HandleLineJoinEventUseCase: Send + Sync + 'static {
    type Transaction;
    type InquiryRepository: InquiryRepository<Err = Error, Transaction = Self::Transaction>;
    type InquirySearchRepository: InquirySearchRepository<Err = Error>;
    type LineRepository: LineRepository<Err = Error>;
    fn execute(
        &self,
        tx: &mut Self::Transaction,
        params: HandleLineJoinEventUseCaseInput,
    ) -> Result<()>;
}

pub struct HandleLineJoinEventUseCaseImpl<
    IR: InquiryRepository<Err = Error>,
    ISR: InquirySearchRepository<Err = Error>,
    LR: LineRepository<Err = Error>,
> {
    inquiry_repository: IR,
    inquiry_search_repository: ISR,
    line_repository: LR,
}

impl<
        IR: InquiryRepository<Err = Error>,
        ISR: InquirySearchRepository<Err = Error>,
        LR: LineRepository<Err = Error>,
    > HandleLineJoinEventUseCaseImpl<IR, ISR, LR>
{
    pub fn new(
        inquiry_repository: IR,
        inquiry_search_repository: ISR,
        line_repository: LR,
    ) -> Self {
        Self {
            inquiry_repository,
            inquiry_search_repository,
            line_repository,
        }
    }
}

impl<
        TX,
        IR: InquiryRepository<Err = Error, Transaction = TX>,
        ISR: InquirySearchRepository<Err = Error>,
        LR: LineRepository<Err = Error>,
    > HandleLineJoinEventUseCase for HandleLineJoinEventUseCaseImpl<IR, ISR, LR>
{
    type Transaction = TX;
    type InquiryRepository = IR;
    type InquirySearchRepository = ISR;
    type LineRepository = LR;

    fn execute(
        &self,
        tx: &mut Self::Transaction,
        params: HandleLineJoinEventUseCaseInput,
    ) -> Result<()> {
        let event_timestamp = NaiveDateTime::from_timestamp_millis(params.timestamp)
            .expect("the line event timestamp must be in valid timestamp milli seconds.");
        let (contact, is_contact_added, channel) = match params.source.r#type {
            line::events::source::SourceType::User(_) => {
                unreachable!()
            },
            line::events::source::SourceType::Group(line_group) => {
                let (contact, is_contact_added) = if let Some(line_user_id) =
                    line_group.user_id.as_deref()
                {
                    self.get_or_create_contact_by_line_user_id(tx, line_user_id, event_timestamp)?
                } else {
                    (None, false)
                };
                let channel_details = InquiryChannelDetails::LineGroup(LineGroupChannelDetails {
                    line_group_id: line_group.group_id.clone(),
                });
                let channel =
                    self.get_or_create_channel_by_details(tx, channel_details, event_timestamp)?;
                (contact, is_contact_added, channel)
            },
            line::events::source::SourceType::Room(line_room) => {
                let (contact, is_contact_added) = if let Some(line_user_id) =
                    line_room.user_id.as_deref()
                {
                    self.get_or_create_contact_by_line_user_id(tx, line_user_id, event_timestamp)?
                } else {
                    (None, false)
                };
                let channel_details = InquiryChannelDetails::LineRoom(LineRoomChannelDetails {
                    line_room_id: line_room.room_id.clone(),
                });
                let channel =
                    self.get_or_create_channel_by_details(tx, channel_details, event_timestamp)?;
                (contact, is_contact_added, channel)
            },
        };
        if is_contact_added {
            let contact = contact.as_ref().unwrap();
            self.inquiry_repository
                .attach_channel_to_contact(tx, contact, &channel)?;
            // update search engine documents.
            self.inquiry_search_repository
                .upsert_inquiry_contact(contact)?;
        }
        Ok(())
    }
}

impl<
        TX,
        IR: InquiryRepository<Err = Error, Transaction = TX>,
        ISR: InquirySearchRepository<Err = Error>,
        LR: LineRepository<Err = Error>,
    > HandleLineJoinEventUseCaseImpl<IR, ISR, LR>
{
    fn get_or_create_contact_by_line_user_id(
        &self,
        tx: &mut TX,
        line_user_id: &str,
        created_at: NaiveDateTime,
    ) -> Result<(Option<InquiryContact>, bool)> {
        match self
            .inquiry_repository
            .get_contact_by_line_user_id(tx, line_user_id)?
        {
            Some(contact) => Ok((Some(contact), false)),
            None => {
                // get line profile
                let line_profile = self.line_repository.get_profile(line_user_id)?;
                if line_profile.is_none() {
                    // the user has already blocked the talk channel.
                    // no need to handle data anymore.
                    return Ok((None, false));
                }
                let contact = InquiryContact {
                    id: InquiryContactId::generate(),
                    details: InquiryContactDetails { line_profile },
                    memo: None,
                    created_at,
                };
                self.inquiry_repository.create_contact(tx, &contact)?;
                Ok((Some(contact), true))
            },
        }
    }

    fn get_or_create_channel_by_details(
        &self,
        tx: &mut TX,
        details: InquiryChannelDetails,
        activated_at: NaiveDateTime,
    ) -> Result<InquiryChannel> {
        match self.inquiry_repository.get_channel_by_type_and_type_id(
            tx,
            details.as_type(),
            details.as_type_id(),
        )? {
            Some(mut channel) => {
                if !channel.is_active {
                    // if the channel has been deactivated, reactivate it.
                    self.inquiry_repository.update_channel_on_reactivated(
                        tx,
                        &mut channel,
                        activated_at,
                    )?;
                }
                Ok(channel)
            },
            None => {
                let channel = InquiryChannel {
                    id: InquiryChannelId::generate(),
                    details,
                    is_active: true,
                    activated_at,
                    deactivated_at: None,
                };
                self.inquiry_repository.create_channel(tx, &channel)?;
                Ok(channel)
            },
        }
    }
}
