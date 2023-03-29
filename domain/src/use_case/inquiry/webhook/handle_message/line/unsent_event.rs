use crate::{model::INQUIRY_MESSAGE_TYPE_LINE, repository::InquiryRepository, Error, Result};
use chrono::NaiveDateTime;

pub type HandleLineUnsentEventUseCaseInput = line::events::UnsendEvent;

pub trait HandleLineUnsentEventUseCase: Send + Sync + 'static {
    type Transaction;
    type InquiryRepository: InquiryRepository<Err = Error, Transaction = Self::Transaction>;
    fn execute(
        &self,
        tx: &mut Self::Transaction,
        params: HandleLineUnsentEventUseCaseInput,
    ) -> Result<()>;
}

pub struct HandleLineUnsentEventUseCaseImpl<IR: InquiryRepository<Err = Error>> {
    inquiry_repository: IR,
}

impl<IR: InquiryRepository<Err = Error>> HandleLineUnsentEventUseCaseImpl<IR> {
    pub fn new(inquiry_repository: IR) -> Self {
        Self { inquiry_repository }
    }
}

impl<TX, IR: InquiryRepository<Err = Error, Transaction = TX>> HandleLineUnsentEventUseCase
    for HandleLineUnsentEventUseCaseImpl<IR>
{
    type Transaction = TX;
    type InquiryRepository = IR;

    fn execute(
        &self,
        tx: &mut Self::Transaction,
        params: HandleLineUnsentEventUseCaseInput,
    ) -> Result<()> {
        let event_timestamp = NaiveDateTime::from_timestamp_millis(params.timestamp)
            .expect("the line event timestamp must be in valid timestamp milli seconds.");
        let message_id = params.unsend.message_id.as_str();
        let inquiry_message = self.inquiry_repository.get_message_by_type_and_type_id(
            tx,
            INQUIRY_MESSAGE_TYPE_LINE,
            message_id,
        )?;
        if inquiry_message.is_none() {
            return Ok(());
        }
        let mut inquiry_message = inquiry_message.unwrap();
        self.inquiry_repository.update_message_on_canceled(
            tx,
            &mut inquiry_message,
            event_timestamp,
        )?;
        Ok(())
    }
}
