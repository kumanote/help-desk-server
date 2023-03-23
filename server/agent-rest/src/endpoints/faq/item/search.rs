use crate::{
    request_parser::{CurrentActiveAgent, Locale},
    schema::{PagingResult, SearchedFaqItem},
    AppState, HttpError, Result,
};
use axum::{
    extract::{Query, State},
    Json,
};
use domain::use_case::{SearchFaqItemUseCase, SearchFaqItemUseCaseImpl, SearchFaqItemUseCaseInput};
use infrastructure::FaqSearchRepositoryExecutor;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SearchFaqItemParams {
    text: Option<String>,
    limit: u64,
    offset: u64,
}

pub async fn handler(
    Locale(locale): Locale,
    CurrentActiveAgent(_agent): CurrentActiveAgent,
    State(state): State<AppState>,
    Query(params): Query<SearchFaqItemParams>,
) -> Result<Json<PagingResult<SearchedFaqItem>>> {
    let faq_search_repository = FaqSearchRepositoryExecutor::new(state.search_client.clone());
    let use_case = SearchFaqItemUseCaseImpl::new(faq_search_repository);
    let logic_input = SearchFaqItemUseCaseInput {
        text: params.text,
        limit: params.limit,
        offset: params.offset,
    };
    let logic_output = use_case
        .execute(logic_input)
        .map_err(|cause| HttpError::from((cause, &locale)))?;
    let schema: PagingResult<SearchedFaqItem> = logic_output.into();
    Ok(schema.into())
}
