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
    UpdateFaqCategoryContent as UpdateFaqCategoryContentUseCaseInput, UpdateFaqCategoryUseCase,
    UpdateFaqCategoryUseCaseImpl, UpdateFaqCategoryUseCaseInput,
};
use infrastructure::FaqRepositoryImpl;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateFaqCategoryParams {
    slug: String,
    contents: Vec<UpdateFaqCategoryContent>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateFaqCategoryContent {
    locale: String,
    title: String,
}

pub async fn handler(
    Locale(locale): Locale,
    CurrentActiveAgent(_agent): CurrentActiveAgent,
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(params): Json<UpdateFaqCategoryParams>,
) -> Result<Json<FaqCategory>> {
    let mut db_connection = state.db_connection_pool.get()?;
    let faq_repository = FaqRepositoryImpl;
    let use_case = UpdateFaqCategoryUseCaseImpl::new(faq_repository);
    let logic_input = UpdateFaqCategoryUseCaseInput {
        id,
        slug: params.slug,
        contents: params
            .contents
            .into_iter()
            .map(|item| UpdateFaqCategoryContentUseCaseInput {
                locale: item.locale,
                title: item.title,
            })
            .collect(),
    };
    let logic_output = use_case
        .execute(&mut db_connection, logic_input)
        .map_err(|cause| HttpError::from((cause, &locale)))?;
    let schema = FaqCategory::from(logic_output);
    Ok(schema.into())
}
