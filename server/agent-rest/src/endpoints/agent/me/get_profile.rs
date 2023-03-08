use crate::{request_parser::CurrentActiveAgent, schema::Agent, AppState, Result};
use axum::{
    debug_handler,
    extract::{Json, State},
};

#[debug_handler]
pub async fn handler(
    CurrentActiveAgent(agent): CurrentActiveAgent,
    State(_): State<AppState>,
) -> Result<Json<Agent>> {
    let schema = Agent::from(agent);
    Ok(schema.into())
}
