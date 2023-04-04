use crate::{
    request_parser::{CurrentActiveAgent, Locale},
    schema::InquirySettings,
    AppState, HttpError, Result,
};
use axum::{extract::State, Json};
use domain::use_case::{
    CreateOrUpdateInquirySettingsUseCase, CreateOrUpdateInquirySettingsUseCaseImpl,
    CreateOrUpdateInquirySettingsUseCaseInput, InquiryLineSettingsInput,
    InquiryNotificationSettingsInput,
};
use infrastructure::InquiryRepositoryImpl;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateOrUpdateInquirySettingsParams {
    pub line: InquiryLineSettingsParams,
    pub notification: InquiryNotificationSettingsParams,
}

#[derive(Debug, Deserialize)]
pub struct InquiryLineSettingsParams {
    pub enabled: bool,
    pub friend_url: Option<String>,
    pub friend_qr_code_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct InquiryNotificationSettingsParams {
    pub slack_webhook_url: Option<String>,
}

pub async fn handler(
    Locale(locale): Locale,
    CurrentActiveAgent(_agent): CurrentActiveAgent,
    State(state): State<AppState>,
    Json(params): Json<CreateOrUpdateInquirySettingsParams>,
) -> Result<Json<InquirySettings>> {
    let mut db_connection = state.db_connection_pool.get()?;
    let inquiry_repository = InquiryRepositoryImpl;
    let use_case = CreateOrUpdateInquirySettingsUseCaseImpl::new(inquiry_repository);
    let logic_input = CreateOrUpdateInquirySettingsUseCaseInput {
        line: InquiryLineSettingsInput {
            enabled: params.line.enabled,
            friend_url: params.line.friend_url,
            friend_qr_code_url: params.line.friend_qr_code_url,
        },
        notification: InquiryNotificationSettingsInput {
            slack_webhook_url: params.notification.slack_webhook_url,
        },
    };
    let logic_output = use_case
        .execute(&mut db_connection, logic_input)
        .map_err(|cause| HttpError::from((cause, &locale)))?;
    let schema = InquirySettings::from(logic_output);
    Ok(schema.into())
}
