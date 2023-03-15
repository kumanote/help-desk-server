use crate::{
    request_parser::{CurrentActiveAgent, Locale},
    AppState, HttpError, Result,
};
use axum::extract::{Path, State};
use domain::use_case::{
    DeleteFaqCategoryUseCase, DeleteFaqCategoryUseCaseImpl, DeleteFaqCategoryUseCaseInput,
};
use infrastructure::FaqRepositoryImpl;

pub async fn handler(
    Locale(locale): Locale,
    CurrentActiveAgent(_agent): CurrentActiveAgent,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<&'static str> {
    let mut db_connection = state.db_connection_pool.get()?;
    let faq_repository = FaqRepositoryImpl;
    let use_case = DeleteFaqCategoryUseCaseImpl::new(faq_repository);
    let logic_input = DeleteFaqCategoryUseCaseInput { id };
    use_case
        .execute(&mut db_connection, logic_input)
        .map_err(|cause| HttpError::from((cause, &locale)))?;
    Ok("OK")
}
