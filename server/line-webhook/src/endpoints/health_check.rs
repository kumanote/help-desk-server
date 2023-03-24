use crate::{AppState, Result};
use axum::extract::State;

pub async fn handler(State(_state): State<AppState>) -> Result<&'static str> {
    Ok("OK")
}
