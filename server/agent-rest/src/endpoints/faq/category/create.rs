use crate::{
    request_parser::{CurrentActiveAgent, Locale},
    schema::FaqCategory,
    AppState, HttpError, Result,
};
use axum::{extract::State, Json};
use domain::use_case::{
    CreateFaqCategoryContent as CreateFaqCategoryContentUseCaseInput, CreateFaqCategoryUseCase,
    CreateFaqCategoryUseCaseImpl, CreateFaqCategoryUseCaseInput,
};
use infrastructure::FaqRepositoryImpl;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateFaqCategoryParams {
    slug: String,
    contents: Vec<CreateFaqCategoryContent>,
}

#[derive(Debug, Deserialize)]
pub struct CreateFaqCategoryContent {
    locale: String,
    title: String,
}

pub async fn handler(
    Locale(locale): Locale,
    CurrentActiveAgent(_agent): CurrentActiveAgent,
    State(state): State<AppState>,
    Json(params): Json<CreateFaqCategoryParams>,
) -> Result<Json<FaqCategory>> {
    let mut db_connection = state.db_connection_pool.get()?;
    let faq_repository = FaqRepositoryImpl;
    let use_case = CreateFaqCategoryUseCaseImpl::new(faq_repository);
    let logic_input = CreateFaqCategoryUseCaseInput {
        slug: params.slug,
        contents: params
            .contents
            .into_iter()
            .map(|item| CreateFaqCategoryContentUseCaseInput {
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
