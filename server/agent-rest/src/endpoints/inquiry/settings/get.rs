use crate::{
    request_parser::{CurrentActiveAgent, Locale},
    schema::InquirySettings,
    AppState, HttpError, Result,
};
use axum::{extract::State, Json};
use domain::use_case::{GetInquirySettingsUseCase, GetInquirySettingsUseCaseImpl};
use infrastructure::InquiryRepositoryImpl;

pub async fn handler(
    Locale(locale): Locale,
    CurrentActiveAgent(_agent): CurrentActiveAgent,
    State(state): State<AppState>,
) -> Result<Json<Option<InquirySettings>>> {
    let mut db_connection = state.db_connection_pool.get()?;
    let inquiry_repository = InquiryRepositoryImpl;
    let use_case = GetInquirySettingsUseCaseImpl::new(inquiry_repository);
    let logic_output = use_case
        .execute(&mut db_connection)
        .map_err(|cause| HttpError::from((cause, &locale)))?;
    let schema = logic_output.map(InquirySettings::from);
    Ok(schema.into())
}
