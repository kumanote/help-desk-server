use crate::{
    request_parser::{CurrentActiveAgent, Locale},
    schema::Scope,
    AppState, HttpError, Result,
};
use axum::{debug_handler, extract::State, Json};
use domain::use_case::{AgentGetScopesUseCase, AgentGetScopesUseCaseImpl};
use infrastructure::RoleRepositoryImpl;

#[debug_handler]
pub async fn handler(
    Locale(locale): Locale,
    CurrentActiveAgent(agent): CurrentActiveAgent,
    State(state): State<AppState>,
) -> Result<Json<Vec<Scope>>> {
    let mut db_connection = state.db_connection_pool.get()?;
    let role_repository = RoleRepositoryImpl::new(state.cache_connection_pool.clone());
    let use_case = AgentGetScopesUseCaseImpl::new(role_repository);
    let logic_output = use_case
        .execute(&mut db_connection, &agent)
        .map_err(|cause| HttpError::from((cause, &locale)))?;
    let schema: Vec<Scope> = logic_output.into_iter().map(Into::into).collect();
    Ok(schema.into())
}
