use crate::{
    model::{InquiryChannelDetails, LineUserChannelDetails},
    repository::InquiryRepository,
    Error, Result,
};
use chrono::NaiveDateTime;

pub type HandleLineUnFollowEventUseCaseInput = line::events::UnFollowEvent;

pub trait HandleLineUnFollowEventUseCase: Send + Sync + 'static {
    type Transaction;
    type InquiryRepository: InquiryRepository<Err = Error, Transaction = Self::Transaction>;
    fn execute(
        &self,
        tx: &mut Self::Transaction,
        params: HandleLineUnFollowEventUseCaseInput,
    ) -> Result<()>;
}

pub struct HandleLineUnFollowEventUseCaseImpl<IR: InquiryRepository<Err = Error>> {
    inquiry_repository: IR,
}

impl<IR: InquiryRepository<Err = Error>> HandleLineUnFollowEventUseCaseImpl<IR> {
    pub fn new(inquiry_repository: IR) -> Self {
        Self { inquiry_repository }
    }
}

impl<TX, IR: InquiryRepository<Err = Error, Transaction = TX>> HandleLineUnFollowEventUseCase
    for HandleLineUnFollowEventUseCaseImpl<IR>
{
    type Transaction = TX;
    type InquiryRepository = IR;

    fn execute(
        &self,
        tx: &mut Self::Transaction,
        params: HandleLineUnFollowEventUseCaseInput,
    ) -> Result<()> {
        let event_timestamp = NaiveDateTime::from_timestamp_millis(params.timestamp)
            .expect("the line event timestamp must be in valid timestamp milli seconds.");
        let channel = match params.source.r#type {
            line::events::source::SourceType::User(line_user) => {
                let channel_details = InquiryChannelDetails::LineUser(LineUserChannelDetails {
                    line_user_id: line_user.user_id.clone(),
                });
                self.inquiry_repository.get_channel_by_type_and_type_id(
                    tx,
                    channel_details.as_type(),
                    channel_details.as_type_id(),
                )?
            },
            line::events::source::SourceType::Group(_) => {
                unreachable!()
            },
            line::events::source::SourceType::Room(_) => {
                unreachable!()
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
