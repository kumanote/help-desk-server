use crate::{
    request_parser::{CurrentActiveAgent, Locale},
    schema::FaqCategory,
    AppState, HttpError, Result,
};
use axum::{
    extract::{Path, State},
    Json,
};
use domain::use_case::{
    GetFaqCategoryUseCase, GetFaqCategoryUseCaseImpl, GetFaqCategoryUseCaseInput,
};
use infrastructure::FaqRepositoryImpl;

pub async fn handler(
    Locale(locale): Locale,
    CurrentActiveAgent(_agent): CurrentActiveAgent,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<FaqCategory>> {
    let mut db_connection = state.db_connection_pool.get()?;
    let faq_repository = FaqRepositoryImpl;
    let use_case = GetFaqCategoryUseCaseImpl::new(faq_repository);
    let logic_input = GetFaqCategoryUseCaseInput { id };
    let logic_output = use_case
        .execute(&mut db_connection, logic_input)
        .map_err(|cause| HttpError::from((cause, &locale)))?;
    let schema = FaqCategory::from(logic_output);
    Ok(schema.into())
}
