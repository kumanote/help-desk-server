use crate::{
    request_parser::{CurrentActiveAgent, Locale},
    AppState, HttpError, Result,
};
use axum::{
    extract::{Path, State},
    Json,
};
use domain::use_case::{
    ReorderFaqItemByCategoryUseCase, ReorderFaqItemByCategoryUseCaseImpl,
    ReorderFaqItemByCategoryUseCaseInput,
};
use infrastructure::FaqRepositoryImpl;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ReorderFaqItemByCategoryParams {
    faq_item_id: String,
    target_faq_item_id: String,
    append: bool,
}

pub async fn handler(
    Locale(locale): Locale,
    CurrentActiveAgent(_agent): CurrentActiveAgent,
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(params): Json<ReorderFaqItemByCategoryParams>,
) -> Result<&'static str> {
    let mut db_connection = state.db_connection_pool.get()?;
    let faq_repository = FaqRepositoryImpl;
    let use_case = ReorderFaqItemByCategoryUseCaseImpl::new(faq_repository);
    let logic_input = ReorderFaqItemByCategoryUseCaseInput {
        faq_category_id: id,
        faq_item_id: params.faq_item_id,
        target_faq_item_id: params.target_faq_item_id,
        append: params.append,
    };
    use_case
        .execute(&mut db_connection, logic_input)
        .map_err(|cause| HttpError::from((cause, &locale)))?;
    Ok("OK")
}
