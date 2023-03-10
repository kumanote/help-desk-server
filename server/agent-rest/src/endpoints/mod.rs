mod agent;
mod auth;
mod faq;
mod general;
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
        .route(
            "/agents/me",
            get(agent::me::get_profile::handler).put(agent::me::update_profile::handler),
        )
        .route(
            "/agents/me/password",
            put(agent::me::change_password::handler),
        )
        .route(
            "/faq/settings",
            get(faq::settings::get::handler).put(faq::settings::update::handler),
        )
        .route("/faq/categories/", post(faq::category::create::handler))
        .route(
            "/general/faq_content_locales/",
            get(general::get_faq_content_locales::handler),
        )
        .with_state(state)
}
