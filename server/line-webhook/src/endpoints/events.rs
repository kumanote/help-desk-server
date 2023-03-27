use crate::{request_parser::ValidEvents, AppState, Result};
use axum::extract::State;

/// line incoming webhook events handler.
pub async fn handler(
    State(_state): State<AppState>,
    ValidEvents(events): ValidEvents,
) -> Result<&'static str> {
    println!("{:?}", events);
    Ok("OK")
}
