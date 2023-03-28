use crate::{request_parser::ValidEvents, AppState, Result};
use axum::extract::State;
use domain::use_case::{
    EnqueueWebhookInquiryMessageUseCase, EnqueueWebhookInquiryMessageUseCaseImpl,
    EnqueueWebhookInquiryMessageUseCaseInput,
};
use infrastructure::InquiryJobRepositoryImpl;

/// line incoming webhook events handler.
pub async fn handler(
    State(state): State<AppState>,
    ValidEvents(events): ValidEvents,
) -> Result<&'static str> {
    let inquiry_job_repository = InquiryJobRepositoryImpl::new(state.queue_connection_pool.clone());
    let use_case = EnqueueWebhookInquiryMessageUseCaseImpl::new(inquiry_job_repository);
    let logic_input = EnqueueWebhookInquiryMessageUseCaseInput::Line(events.events);
    use_case.execute(logic_input)?;
    Ok("OK")
}
