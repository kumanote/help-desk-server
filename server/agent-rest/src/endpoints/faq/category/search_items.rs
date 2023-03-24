use crate::{
    request_parser::{CurrentActiveAgent, Locale},
    schema::{FaqCategoryItem, PagingResult},
    AppState, HttpError, Result,
};
use axum::{
    extract::{Path, Query, State},
    Json,
};
use domain::use_case::{
    SearchFaqItemByCategoryUseCase, SearchFaqItemByCategoryUseCaseImpl,
    SearchFaqItemByCategoryUseCaseInput,
};
use infrastructure::FaqRepositoryImpl;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SearchFaqItemByCategoryParams {
    limit: u64,
    offset: u64,
}

pub async fn handler(
    Locale(locale): Locale,
    CurrentActiveAgent(_agent): CurrentActiveAgent,
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(params): Query<SearchFaqItemByCategoryParams>,
) -> Result<Json<PagingResult<FaqCategoryItem>>> {
    let mut db_connection = state.db_connection_pool.get()?;
    let faq_repository = FaqRepositoryImpl;
    let use_case = SearchFaqItemByCategoryUseCaseImpl::new(faq_repository);
    let logic_input = SearchFaqItemByCategoryUseCaseInput {
        faq_category_id: id,
        limit: params.limit,
        offset: params.offset,
    };
    let logic_output = use_case
        .execute(&mut db_connection, logic_input)
        .map_err(|cause| HttpError::from((cause, &locale)))?;
    let schema: PagingResult<FaqCategoryItem> = logic_output.into();
    Ok(schema.into())
}
