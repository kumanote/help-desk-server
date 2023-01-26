use crate::{request_parser::Locale, schema::Workspace, AppState, HttpError, Result};
use axum::{extract::State, Json};
use domain::use_case::{GetWorkspaceUseCase, GetWorkspaceUseCaseImpl};
use infrastructure::WorkspaceRepositoryImpl;

pub async fn handler(
    Locale(locale): Locale,
    State(state): State<AppState>,
) -> Result<Json<Option<Workspace>>> {
    let mut db_connection = state.db_connection_pool.get()?;
    let workspace_repository = WorkspaceRepositoryImpl;
    let use_case = GetWorkspaceUseCaseImpl::new(workspace_repository);
    let logic_output = use_case
        .execute(&mut db_connection)
        .map_err(|cause| HttpError::from((cause, &locale)))?;
    let schema: Option<Workspace> = logic_output.map(Into::into);
    Ok(schema.into())
}
