use crate::{
    request_parser::{CurrentActiveAgent, Locale},
    schema::FaqItem,
    AppState, HttpError, Result,
};
use axum::{extract::State, Json};
use domain::use_case::{
    CreateFaqItemContent as CreateFaqItemContentUseCaseInput, CreateFaqItemUseCase,
    CreateFaqItemUseCaseImpl, CreateFaqItemUseCaseInput,
};
use infrastructure::{
    FaqRepositoryImpl, FaqSearchRepositoryDelegator, PublicFaqSearchRepositoryDelegator,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateFaqItemParams {
    slug: String,
    is_published: bool,
    contents: Vec<CreateFaqItemContent>,
    categories: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateFaqItemContent {
    locale: String,
    title: String,
    body: String,
}

pub async fn handler(
    Locale(locale): Locale,
    CurrentActiveAgent(_agent): CurrentActiveAgent,
    State(state): State<AppState>,
    Json(params): Json<CreateFaqItemParams>,
) -> Result<Json<FaqItem>> {
    let mut db_connection = state.db_connection_pool.get()?;
    let faq_repository = FaqRepositoryImpl;
    let faq_search_repository =
        FaqSearchRepositoryDelegator::new(state.queue_connection_pool.clone());
    let public_faq_search_repository =
        PublicFaqSearchRepositoryDelegator::new(state.queue_connection_pool.clone());
    let use_case = CreateFaqItemUseCaseImpl::new(
        faq_repository,
        faq_search_repository,
        public_faq_search_repository,
    );
    let logic_input = CreateFaqItemUseCaseInput {
        slug: params.slug,
        is_published: params.is_published,
        contents: params
            .contents
            .into_iter()
            .map(|item| CreateFaqItemContentUseCaseInput {
                locale: item.locale,
                title: item.title,
                body: item.body,
            })
            .collect(),
        categories: params.categories,
    };
    let logic_output = use_case
        .execute(&mut db_connection, logic_input)
        .map_err(|cause| HttpError::from((cause, &locale)))?;
    let schema = FaqItem::from(logic_output);
    Ok(schema.into())
}
