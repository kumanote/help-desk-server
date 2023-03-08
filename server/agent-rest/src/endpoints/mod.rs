mod agent;
mod auth;
mod health_check;
mod index;
mod workspace;

use crate::AppState;
use axum::routing::{get, post, put};
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
        .route("/auth/scopes/", get(auth::get_scopes::handler))
        .route("/agent/me", get(agent::me::get_profile::handler))
        .route(
            "/agent/me/password",
            put(agent::me::change_password::handler),
        )
        .with_state(state)
}
