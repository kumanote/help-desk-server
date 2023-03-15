use crate::{
    request_parser::{CurrentActiveAgent, Locale},
    AppState, HttpError, Result,
};
use axum::{extract::State, Json};
use domain::use_case::{
    ReorderFaqCategoryUseCase, ReorderFaqCategoryUseCaseImpl, ReorderFaqCategoryUseCaseInput,
};
use infrastructure::FaqRepositoryImpl;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ReorderFaqCategoryParams {
    id: String,
    target_id: String,
    append: bool,
}

pub async fn handler(
    Locale(locale): Locale,
    CurrentActiveAgent(_agent): CurrentActiveAgent,
    State(state): State<AppState>,
    Json(params): Json<ReorderFaqCategoryParams>,
) -> Result<&'static str> {
    let mut db_connection = state.db_connection_pool.get()?;
    let faq_repository = FaqRepositoryImpl;
    let use_case = ReorderFaqCategoryUseCaseImpl::new(faq_repository);
    let logic_input = ReorderFaqCategoryUseCaseInput {
        id: params.id,
        target_id: params.target_id,
        append: params.append,
    };
    use_case
        .execute(&mut db_connection, logic_input)
        .map_err(|cause| HttpError::from((cause, &locale)))?;
    Ok("OK")
}
