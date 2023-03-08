use crate::{
    request_parser::{CurrentActiveAgent, Locale},
    AppState, HttpError, Result,
};
use axum::{
    debug_handler,
    extract::{Json, State},
};
use domain::use_case::{
    AgentChangePasswordUseCase, AgentChangePasswordUseCaseImpl, AgentChangePasswordUseCaseInput,
};
use infrastructure::AgentRepositoryImpl;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ChangePasswordParams {
    current_password: String,
    new_password: String,
}

#[debug_handler]
pub async fn handler(
    Locale(locale): Locale,
    CurrentActiveAgent(agent): CurrentActiveAgent,
    State(state): State<AppState>,
    Json(params): Json<ChangePasswordParams>,
) -> Result<&'static str> {
    let mut db_connection = state.db_connection_pool.get()?;
    let agent_repository = AgentRepositoryImpl;
    let use_case = AgentChangePasswordUseCaseImpl::new(agent_repository);
    let logic_input = AgentChangePasswordUseCaseInput {
        current_password: params.current_password,
        new_password: params.new_password,
    };
    let mut agent = agent.clone();
    use_case
        .execute(&mut db_connection, &mut agent, logic_input)
        .map_err(|cause| HttpError::from((cause, &locale)))?;
    Ok("OK")
}
