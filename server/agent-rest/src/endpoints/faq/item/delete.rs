use crate::{
    request_parser::{CurrentActiveAgent, Locale},
    AppState, HttpError, Result,
};
use axum::extract::{Path, State};
use domain::use_case::{DeleteFaqItemUseCase, DeleteFaqItemUseCaseImpl, DeleteFaqItemUseCaseInput};
use infrastructure::{
    FaqRepositoryImpl, FaqSearchRepositoryDelegator, PublicFaqSearchRepositoryDelegator,
};

pub async fn handler(
    Locale(locale): Locale,
    CurrentActiveAgent(_agent): CurrentActiveAgent,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<&'static str> {
    let mut db_connection = state.db_connection_pool.get()?;
    let faq_repository = FaqRepositoryImpl;
    let faq_search_repository =
        FaqSearchRepositoryDelegator::new(state.queue_connection_pool.clone());
    let public_faq_search_repository =
        PublicFaqSearchRepositoryDelegator::new(state.queue_connection_pool.clone());
    let use_case = DeleteFaqItemUseCaseImpl::new(
        faq_repository,
        faq_search_repository,
        public_faq_search_repository,
    );
    let logic_input = DeleteFaqItemUseCaseInput { id };
    use_case
        .execute(&mut db_connection, logic_input)
        .map_err(|cause| HttpError::from((cause, &locale)))?;
    Ok("OK")
}
