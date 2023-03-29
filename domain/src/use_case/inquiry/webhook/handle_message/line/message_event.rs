use crate::{
    model::{
        InquiryChannel, InquiryChannelDetails, InquiryChannelId, InquiryContact,
        InquiryContactDetails, InquiryContactId, InquiryMessage, InquiryMessageDetails,
        InquiryMessageId, InquiryMessageSpeaker, InquiryThread, InquiryThreadDetails,
        InquiryThreadId, InquiryThreadStatus, InquiryThreadSubject, LineGroupChannelDetails,
        LineGroupThreadDetails, LineMessageDetails, LineRoomChannelDetails, LineRoomThreadDetails,
        LineUserChannelDetails, LineUserThreadDetails,
    },
    repository::{InquiryRepository, InquirySearchRepository, LineRepository},
    Error, Result,
};
use chrono::NaiveDateTime;

pub type HandleLineMessageEventUseCaseInput = line::events::MessageEvent;

pub trait HandleLineMessageEventUseCase: Send + Sync + 'static {
    type Transaction;
    type InquiryRepository: InquiryRepository<Err = Error, Transaction = Self::Transaction>;
    type InquirySearchRepository: InquirySearchRepository<Err = Error>;
    type LineRepository: LineRepository<Err = Error>;
    fn execute(
        &self,
        tx: &mut Self::Transaction,
        params: HandleLineMessageEventUseCaseInput,
    ) -> Result<()>;
}

pub struct HandleLineMessageEventUseCaseImpl<
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
    > HandleLineMessageEventUseCaseImpl<IR, ISR, LR>
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
    > HandleLineMessageEventUseCase for HandleLineMessageEventUseCaseImpl<IR, ISR, LR>
{
    type Transaction = TX;
    type InquiryRepository = IR;
    type InquirySearchRepository = ISR;
    type LineRepository = LR;

    fn execute(
        &self,
        tx: &mut Self::Transaction,
        params: HandleLineMessageEventUseCaseInput,
    ) -> Result<()> {
        let event_timestamp = NaiveDateTime::from_timestamp_millis(params.timestamp)
            .expect("the line event timestamp must be in valid timestamp milli seconds.");
        let (contact, is_contact_added, channel, thread) = match params.source.r#type {
            line::events::source::SourceType::User(line_user) => {
                let (contact, is_contact_added) = self.get_or_create_contact_by_line_user_id(
                    tx,
                    &line_user.user_id,
                    event_timestamp,
                )?;
                if contact.is_none() {
                    return Ok(());
                }
                let contact = contact.unwrap();
                let channel_details = InquiryChannelDetails::LineUser(LineUserChannelDetails {
                    line_user_id: line_user.user_id.clone(),
                });
                let channel =
                    self.get_or_create_channel_by_details(tx, channel_details, event_timestamp)?;
                let thread_details = InquiryThreadDetails::LineUser(LineUserThreadDetails {
                    line_user_id: line_user.user_id.clone(),
                    message: params.message.clone(),
                });
                let thread = self.get_or_create_thread_by_details(
                    tx,
                    &channel,
                    thread_details,
                    &params.message,
                    event_timestamp,
                )?;
                (contact, is_contact_added, channel, thread)
            },
            line::events::source::SourceType::Group(line_group) => {
                if line_group.user_id.is_none() {
                    // there must be a line user that sent this message.
                    return Ok(());
                }
                let line_user_id = line_group.user_id.as_ref().unwrap().as_str();
                let (contact, is_contact_added) =
                    self.get_or_create_contact_by_line_user_id(tx, line_user_id, event_timestamp)?;
                if contact.is_none() {
                    return Ok(());
                }
                let contact = contact.unwrap();
                let channel_details = InquiryChannelDetails::LineGroup(LineGroupChannelDetails {
                    line_group_id: line_group.group_id.clone(),
                });
                let channel =
                    self.get_or_create_channel_by_details(tx, channel_details, event_timestamp)?;
                let thread_details = InquiryThreadDetails::LineGroup(LineGroupThreadDetails {
                    line_group_id: line_group.group_id.clone(),
                    message: params.message.clone(),
                });
                let thread = self.get_or_create_thread_by_details(
                    tx,
                    &channel,
                    thread_details,
                    &params.message,
                    event_timestamp,
                )?;
                (contact, is_contact_added, channel, thread)
            },
            line::events::source::SourceType::Room(line_room) => {
                if line_room.user_id.is_none() {
                    // there must be a line user that sent this message.
                    return Ok(());
                }
                let line_user_id = line_room.user_id.as_ref().unwrap().as_str();
                let (contact, is_contact_added) =
                    self.get_or_create_contact_by_line_user_id(tx, line_user_id, event_timestamp)?;
                if contact.is_none() {
                    return Ok(());
                }
                let contact = contact.unwrap();
                let channel_details = InquiryChannelDetails::LineRoom(LineRoomChannelDetails {
                    line_room_id: line_room.room_id.clone(),
                });
                let channel =
                    self.get_or_create_channel_by_details(tx, channel_details, event_timestamp)?;
                let thread_details = InquiryThreadDetails::LineRoom(LineRoomThreadDetails {
                    line_room_id: line_room.room_id.clone(),
                    message: params.message.clone(),
                });
                let thread = self.get_or_create_thread_by_details(
                    tx,
                    &channel,
                    thread_details,
                    &params.message,
                    event_timestamp,
                )?;
                (contact, is_contact_added, channel, thread)
            },
        };
        let message_details = InquiryMessageDetails::Line(LineMessageDetails {
            message: params.message.clone(),
        });
        let (message, is_message_added) = self.get_or_create_message_by_details(
            tx,
            &contact,
            &thread,
            message_details,
            event_timestamp,
        )?;
        if is_contact_added {
            self.inquiry_repository
                .attach_channel_to_contact(tx, &contact, &channel)?;
            // update search engine documents.
            self.inquiry_search_repository
                .upsert_inquiry_contact(&contact)?;
        }
        if is_message_added {
            // update search engine documents.
            self.inquiry_search_repository
                .upsert_inquiry_message(&message)?;
        }
        Ok(())
    }
}

impl<
        TX,
        IR: InquiryRepository<Err = Error, Transaction = TX>,
        ISR: InquirySearchRepository<Err = Error>,
        LR: LineRepository<Err = Error>,
    > HandleLineMessageEventUseCaseImpl<IR, ISR, LR>
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
        match self
            .inquiry_repository
            .get_channel_by_details(tx, &details)?
        {
            Some(channel) => Ok(channel),
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

    fn get_or_create_thread_by_details(
        &self,
        tx: &mut TX,
        channel: &InquiryChannel,
        details: InquiryThreadDetails,
        message: &line::events::messages::Message,
        opened_at: NaiveDateTime,
    ) -> Result<InquiryThread> {
        match self
            .inquiry_repository
            .get_thread_by_details(tx, &details)?
        {
            Some(thread) => Ok(thread),
            None => {
                let subject = InquiryThreadSubject::from(&message.r#type);
                let thread = InquiryThread {
                    id: InquiryThreadId::generate(),
                    inquiry_channel_id: channel.id.clone(),
                    subject,
                    details,
                    status: InquiryThreadStatus::Open,
                    assigned_agent_id: None,
                    opened_at,
                    closed_at: None,
                };
                self.inquiry_repository.create_thread(tx, &thread)?;
                Ok(thread)
            },
        }
    }

    fn get_or_create_message_by_details(
        &self,
        tx: &mut TX,
        contact: &InquiryContact,
        thread: &InquiryThread,
        details: InquiryMessageDetails,
        created_at: NaiveDateTime,
    ) -> Result<(InquiryMessage, bool)> {
        match self
            .inquiry_repository
            .get_message_by_details(tx, &details)?
        {
            Some(message) => Ok((message, false)),
            None => {
                let message = InquiryMessage {
                    id: InquiryMessageId::generate(),
                    inquiry_thread_id: thread.id.clone(),
                    reply_inquiry_message_id: None,
                    speaker: InquiryMessageSpeaker::Contact(contact.id.clone()),
                    details,
                    created_at,
                };
                self.inquiry_repository.create_message(tx, &message)?;
                Ok((message, true))
            },
        }
    }
}
