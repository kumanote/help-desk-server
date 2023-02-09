use crate::{request_parser::Locale, schema::Workspace, AppState, HttpError, Result};
use axum::{
    debug_handler,
    extract::{Json, State},
};
use database::Connection;
use domain::use_case::{InitWorkspaceUseCase, InitWorkspaceUseCaseImpl, InitWorkspaceUseCaseInput};
use infrastructure::{
    AgentRepositoryImpl, GroupRepositoryImpl, RoleForGroupRepositoryImpl, RoleRepositoryImpl,
    WorkspaceRepositoryImpl,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct InitWorkspaceParams {
    workspace_name: String,
    first_agent_email: String,
    first_agent_password: String,
    first_agent_name: String,
}

#[debug_handler]
pub async fn handler(
    Locale(locale): Locale,
    State(state): State<AppState>,
    Json(params): Json<InitWorkspaceParams>,
) -> Result<Json<Workspace>> {
    let mut db_connection = state.db_connection_pool.get()?;
    let workspace_repository = WorkspaceRepositoryImpl;
    let agent_repository = AgentRepositoryImpl;
    let group_repository = GroupRepositoryImpl;
    let role_repository = RoleRepositoryImpl::new(state.cache_connection_pool.clone());
    let role_for_group_repository = RoleForGroupRepositoryImpl;
    let use_case = InitWorkspaceUseCaseImpl::new(
        workspace_repository,
        agent_repository,
        group_repository,
        role_repository,
        role_for_group_repository,
    );
    let logic_input = InitWorkspaceUseCaseInput {
        workspace_name: params.workspace_name,
        first_agent_email: params.first_agent_email,
        first_agent_password: params.first_agent_password,
        first_agent_name: params.first_agent_name,
        first_agent_locale: locale.clone(),
    };
    let logic_output = db_connection.transaction(|db_connection| {
        use_case
            .execute(db_connection, logic_input)
            .map_err(|cause| HttpError::from((cause, &locale)))
    })?;
    let schema = Workspace::from(logic_output);
    Ok(schema.into())
}
