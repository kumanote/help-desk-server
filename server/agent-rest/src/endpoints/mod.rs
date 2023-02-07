mod auth;
mod health_check;
mod index;
mod workspace;

use crate::AppState;
use axum::routing::{get, post};
use axum::Router;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(index::handler))
        .route("/healthz", get(health_check::handler))
        .route(
            "/workspace",
            get(workspace::get::handler).post(workspace::init::handler),
        )
        .route(
            "/auth",
            post(auth::login::handler).delete(auth::logout::handler),
        )
        .with_state(state)
}
