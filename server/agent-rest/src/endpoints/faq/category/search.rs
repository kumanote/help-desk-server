use crate::{
    request_parser::{CurrentActiveAgent, Locale},
    schema::{FaqCategory, PagingResult},
    AppState, HttpError, Result,
};
use axum::{
    extract::{Query, State},
    Json,
};
use domain::use_case::{
    SearchFaqCategoryUseCase, SearchFaqCategoryUseCaseImpl, SearchFaqCategoryUseCaseInput,
};
use infrastructure::FaqRepositoryImpl;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SearchFaqCategoryParams {
    text: Option<String>,
    ids: Option<Vec<String>>,
    limit: u64,
    offset: u64,
}

pub async fn handler(
    Locale(locale): Locale,
    CurrentActiveAgent(_agent): CurrentActiveAgent,
    State(state): State<AppState>,
    Query(params): Query<SearchFaqCategoryParams>,
) -> Result<Json<PagingResult<FaqCategory>>> {
    let mut db_connection = state.db_connection_pool.get()?;
    let faq_repository = FaqRepositoryImpl;
    let use_case = SearchFaqCategoryUseCaseImpl::new(faq_repository);
    let logic_input = SearchFaqCategoryUseCaseInput {
        text: params.text,
        ids: params.ids,
        limit: params.limit,
        offset: params.offset,
    };
    let logic_output = use_case
        .execute(&mut db_connection, logic_input)
        .map_err(|cause| HttpError::from((cause, &locale)))?;
    let schema: PagingResult<FaqCategory> = logic_output.into();
    Ok(schema.into())
}
