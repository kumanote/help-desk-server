use crate::{
    request_parser::{CurrentActiveAgent, Locale},
    schema::Agent,
    AppState, HttpError, Result,
};
use axum::{
    debug_handler,
    extract::{Json, State},
};
use domain::use_case::{
    AgentUpdateProfileUseCase, AgentUpdateProfileUseCaseImpl, AgentUpdateProfileUseCaseInput,
};
use infrastructure::AgentRepositoryImpl;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateProfileParams {
    email: String,
    name: String,
}

#[debug_handler]
pub async fn handler(
    Locale(locale): Locale,
    CurrentActiveAgent(agent): CurrentActiveAgent,
    State(state): State<AppState>,
    Json(params): Json<UpdateProfileParams>,
) -> Result<Json<Agent>> {
    let mut db_connection = state.db_connection_pool.get()?;
    let agent_repository = AgentRepositoryImpl;
    let use_case = AgentUpdateProfileUseCaseImpl::new(agent_repository);
    let logic_input = AgentUpdateProfileUseCaseInput {
        email: params.email,
        name: params.name,
    };
    let mut agent = agent.clone();
    use_case
        .execute(&mut db_connection, &mut agent, logic_input)
        .map_err(|cause| HttpError::from((cause, &locale)))?;
    let schema = Agent::from(agent);
    Ok(schema.into())
}
