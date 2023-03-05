use crate::{
    config, constants, request_parser::Locale, schema::AgentAccessToken, AppState, HttpError,
    Result,
};
use axum::{
    debug_handler,
    extract::{Json, State},
};
use axum_client_ip::InsecureClientIp;
use domain::use_case::{AgentLoginUseCase, AgentLoginUseCaseImpl, AgentLoginUseCaseInput};
use infrastructure::{AgentLoginRepositoryImpl, AgentRepositoryImpl};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginParams {
    username: String,
    password: String,
}

#[debug_handler]
pub async fn handler(
    InsecureClientIp(client_ip): InsecureClientIp,
    Locale(locale): Locale,
    State(state): State<AppState>,
    Json(params): Json<LoginParams>,
) -> Result<Json<AgentAccessToken>> {
    let config = config::app_config();
    let mut db_connection = state.db_connection_pool.get()?;
    let agent_repository = AgentRepositoryImpl;
    let agent_login_repository = AgentLoginRepositoryImpl::new(state.cache_connection_pool.clone());
    let use_case = AgentLoginUseCaseImpl::new(agent_repository, agent_login_repository);
    let logic_input = AgentLoginUseCaseInput {
        username: params.username,
        password: params.password,
        client_ip: client_ip.to_string(),
        secret: config.agent_rest.secret_key.clone(),
        access_token_duration: constants::get_agent_access_token_expire_duration(),
        username_failure_limit: constants::AGENT_LOGIN_USERNAME_FAILURE_LIMIT,
        ip_failure_limit: constants::AGENT_LOGIN_IP_FAILURE_LIMIT,
    };
    let logic_output = use_case
        .execute(&mut db_connection, logic_input)
        .map_err(|cause| HttpError::from((cause, &locale)))?;
    let schema = AgentAccessToken::from(logic_output);
    Ok(schema.into())
}
