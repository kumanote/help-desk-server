mod events;
mod health_check;
mod index;

use crate::AppState;
use axum::routing::{get, post};
use axum::Router;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(index::handler))
        .route("/healthz", get(health_check::handler))
        .route("/events/", post(events::handler))
        .with_state(state)
}
