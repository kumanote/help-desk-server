use crate::{
    request_parser::{AgentAccessToken, CurrentActiveAgent, Locale},
    AppState, HttpError, Result,
};
use axum::{debug_handler, extract::State};
use domain::use_case::{AgentLogoutUseCase, AgentLogoutUseCaseImpl, AgentLogoutUseCaseInput};
use infrastructure::AgentLoginRepositoryImpl;

#[debug_handler]
pub async fn handler(
    Locale(locale): Locale,
    CurrentActiveAgent(agent): CurrentActiveAgent,
    AgentAccessToken(agent_access_token): AgentAccessToken,
    State(state): State<AppState>,
) -> Result<&'static str> {
    let mut db_connection = state.db_connection_pool.get()?;
    let agent_login_repository = AgentLoginRepositoryImpl::new(state.cache_connection_pool.clone());
    let use_case = AgentLogoutUseCaseImpl::new(agent_login_repository);
    let logic_input = AgentLogoutUseCaseInput {
        access_token: agent_access_token.unwrap(),
    };
    use_case
        .execute(&mut db_connection, &agent, logic_input)
        .map_err(|cause| HttpError::from((cause, &locale)))?;
    Ok("OK")
}
