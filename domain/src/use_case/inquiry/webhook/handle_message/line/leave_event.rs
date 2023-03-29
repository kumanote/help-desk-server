use crate::{
    model::{InquiryChannelDetails, LineGroupChannelDetails, LineRoomChannelDetails},
    repository::InquiryRepository,
    Error, Result,
};
use chrono::NaiveDateTime;

pub type HandleLineLeaveEventUseCaseInput = line::events::LeaveEvent;

pub trait HandleLineLeaveEventUseCase: Send + Sync + 'static {
    type Transaction;
    type InquiryRepository: InquiryRepository<Err = Error, Transaction = Self::Transaction>;
    fn execute(
        &self,
        tx: &mut Self::Transaction,
        params: HandleLineLeaveEventUseCaseInput,
    ) -> Result<()>;
}

pub struct HandleLineLeaveEventUseCaseImpl<IR: InquiryRepository<Err = Error>> {
    inquiry_repository: IR,
}

impl<IR: InquiryRepository<Err = Error>> HandleLineLeaveEventUseCaseImpl<IR> {
    pub fn new(inquiry_repository: IR) -> Self {
        Self { inquiry_repository }
    }
}

impl<TX, IR: InquiryRepository<Err = Error, Transaction = TX>> HandleLineLeaveEventUseCase
    for HandleLineLeaveEventUseCaseImpl<IR>
{
    type Transaction = TX;
    type InquiryRepository = IR;

    fn execute(
        &self,
        tx: &mut Self::Transaction,
        params: HandleLineLeaveEventUseCaseInput,
    ) -> Result<()> {
        let event_timestamp = NaiveDateTime::from_timestamp_millis(params.timestamp)
            .expect("the line event timestamp must be in valid timestamp milli seconds.");
        let channel = match params.source.r#type {
            line::events::source::SourceType::User(_) => {
                unreachable!()
            },
            line::events::source::SourceType::Group(line_group) => {
                // case: line group admin remove the line official account from the group
                let channel_details = InquiryChannelDetails::LineGroup(LineGroupChannelDetails {
                    line_group_id: line_group.group_id.clone(),
                });
                self.inquiry_repository.get_channel_by_type_and_type_id(
                    tx,
                    channel_details.as_type(),
                    channel_details.as_type_id(),
                )?
            },
            line::events::source::SourceType::Room(line_room) => {
                let channel_details = InquiryChannelDetails::LineRoom(LineRoomChannelDetails {
                    line_room_id: line_room.room_id.clone(),
                });
                self.inquiry_repository.get_channel_by_type_and_type_id(
                    tx,
                    channel_details.as_type(),
                    channel_details.as_type_id(),
                )?
            },
        };
        if channel.is_none() {
            return Ok(());
        }
        let mut channel = channel.unwrap();
        self.inquiry_repository
            .update_channel_on_deactivated(tx, &mut channel, event_timestamp)?;
        Ok(())
    }
}
