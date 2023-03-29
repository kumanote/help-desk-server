use crate::{
    model::{InquiryIncomingEvent, InquiryIncomingEvents},
    repository::InquiryJobRepository,
    Error, Result,
};

pub type EnqueueWebhookInquiryMessageUseCaseInput = InquiryIncomingEvents;

pub trait EnqueueWebhookInquiryMessageUseCase: Send + Sync + 'static {
    type InquiryJobRepository: InquiryJobRepository<Err = Error>;
    fn execute(&self, params: EnqueueWebhookInquiryMessageUseCaseInput) -> Result<()>;
}

pub struct EnqueueWebhookInquiryMessageUseCaseImpl<IJR: InquiryJobRepository<Err = Error>> {
    inquiry_job_repository: IJR,
}

impl<IJR: InquiryJobRepository<Err = Error>> EnqueueWebhookInquiryMessageUseCaseImpl<IJR> {
    pub fn new(inquiry_job_repository: IJR) -> Self {
        Self {
            inquiry_job_repository,
        }
    }
}

impl<IJR: InquiryJobRepository<Err = Error>> EnqueueWebhookInquiryMessageUseCase
    for EnqueueWebhookInquiryMessageUseCaseImpl<IJR>
{
    type InquiryJobRepository = IJR;

    fn execute(&self, params: EnqueueWebhookInquiryMessageUseCaseInput) -> Result<()> {
        match params {
            InquiryIncomingEvents::Line(events) => {
                for event in events {
                    handle_line_event(&self.inquiry_job_repository, event)?;
                }
            },
        }
        Ok(())
    }
}

fn handle_line_event<IJR: InquiryJobRepository<Err = Error>>(
    inquiry_job_repository: &IJR,
    event: line::events::Event,
) -> Result<()> {
    let need_to_handle = match &event.r#type {
        line::events::EventType::UnsendEvent(_) => true,
        line::events::EventType::FollowEvent(_) => true,
        line::events::EventType::UnFollowEvent(_) => true,
        line::events::EventType::JoinEvent(_) => true,
        line::events::EventType::LeaveEvent(_) => true,
        line::events::EventType::MemberJoinEvent(_) => false,
        line::events::EventType::MemberLeaveEvent(_) => false,
        line::events::EventType::PostBackEvent(_) => false,
        line::events::EventType::VideoPlayCompleteEvent(_) => false,
        line::events::EventType::BeaconEvent(_) => false,
        line::events::EventType::AccountLinkEvent(_) => false,
        line::events::EventType::ThingsEvent(_) => false,
        line::events::EventType::MessageEvent(_) => true,
        line::events::EventType::Other => false,
    };
    if need_to_handle {
        inquiry_job_repository.register(&InquiryIncomingEvent::Line(event))?;
    } else {
        // TODO output warning logs
    }
    Ok(())
}
