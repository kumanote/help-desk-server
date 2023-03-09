use crate::{
    request_parser::{CurrentActiveAgent, Locale},
    schema::FaqSettings,
    AppState, HttpError, Result,
};
use axum::{extract::State, Json};
use domain::use_case::{
    UpdateFaqSettingsUseCase, UpdateFaqSettingsUseCaseImpl, UpdateFaqSettingsUseCaseInput,
};
use infrastructure::FaqRepositoryImpl;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateFaqSettingsParams {
    pub home_url: Option<String>,
    pub supported_locales: Vec<String>,
}

pub async fn handler(
    Locale(locale): Locale,
    CurrentActiveAgent(_agent): CurrentActiveAgent,
    State(state): State<AppState>,
    Json(params): Json<UpdateFaqSettingsParams>,
) -> Result<Json<FaqSettings>> {
    let mut db_connection = state.db_connection_pool.get()?;
    let faq_repository = FaqRepositoryImpl;
    let use_case = UpdateFaqSettingsUseCaseImpl::new(faq_repository);
    let logic_input = UpdateFaqSettingsUseCaseInput {
        home_url: params.home_url,
        supported_locales: params.supported_locales,
    };
    let logic_output = use_case
        .execute(&mut db_connection, logic_input)
        .map_err(|cause| HttpError::from((cause, &locale)))?;
    let schema = FaqSettings::from(logic_output);
    Ok(schema.into())
}
