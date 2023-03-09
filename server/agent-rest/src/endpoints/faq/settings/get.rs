use crate::{
    request_parser::{CurrentActiveAgent, Locale},
    schema::FaqSettings,
    AppState, HttpError, Result,
};
use axum::{extract::State, Json};
use domain::use_case::{GetFaqSettingsUseCase, GetFaqSettingsUseCaseImpl};
use infrastructure::FaqRepositoryImpl;

pub async fn handler(
    Locale(locale): Locale,
    CurrentActiveAgent(_agent): CurrentActiveAgent,
    State(state): State<AppState>,
) -> Result<Json<FaqSettings>> {
    let mut db_connection = state.db_connection_pool.get()?;
    let faq_repository = FaqRepositoryImpl;
    let use_case = GetFaqSettingsUseCaseImpl::new(faq_repository);
    let logic_output = use_case
        .execute(&mut db_connection)
        .map_err(|cause| HttpError::from((cause, &locale)))?;
    let schema = FaqSettings::from(logic_output);
    Ok(schema.into())
}
