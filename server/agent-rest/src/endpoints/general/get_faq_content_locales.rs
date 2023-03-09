use crate::{request_parser::CurrentActiveAgent, schema::ContentLocale, AppState, Result};
use axum::{debug_handler, extract::State, Json};

#[debug_handler]
pub async fn handler(
    CurrentActiveAgent(_): CurrentActiveAgent,
    State(_): State<AppState>,
) -> Result<Json<Vec<ContentLocale>>> {
    let schema: Vec<ContentLocale> = domain::model::FaqContentLocale::all()
        .into_iter()
        .map(Into::into)
        .collect();
    Ok(schema.into())
}
