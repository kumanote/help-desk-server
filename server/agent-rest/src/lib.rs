mod error;
use error::HttpError;
pub use error::ServerError;
pub type Result<T> = core::result::Result<T, HttpError>;

mod config;
mod endpoints;
mod request_parser;
mod schema;

use agent_rest_config::AppConfig;
use anyhow::anyhow;
use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use cache::CacheConnectionPool;
use database::DbConnectionPool;
use queue::QueueConnectionPool;
use std::net::SocketAddr;
use tokio::signal;
use tower_http::cors::CorsLayer;

#[derive(Clone)]
pub struct AppState {
    pub db_connection_pool: DbConnectionPool,
    pub cache_connection_pool: CacheConnectionPool,
    pub queue_connection_pool: QueueConnectionPool,
}

pub async fn start(app_config: AppConfig) -> std::result::Result<(), ServerError> {
    // set global app config
    config::set_app_config(app_config);
    let app_config = config::app_config();
    // build global app state
    let db_connection_pool = database::new_pool(
        &app_config.database.url,
        app_config.database.max_connection_pool_size,
    )?;
    let cache_connection_pool = cache::new_pool(
        &app_config.cache.url,
        app_config.cache.max_connection_pool_size,
    )?;
    let queue_connection_pool = queue::new_pool(
        &app_config.cache.url,
        app_config.cache.max_connection_pool_size,
    )?;
    let app_state = AppState {
        db_connection_pool,
        cache_connection_pool,
        queue_connection_pool,
    };
    // server running options
    let mut allow_origins: Vec<HeaderValue> = vec![];
    for origin_str in &app_config.agent_rest.allowed_origins {
        let origin =
            origin_str
                .parse::<HeaderValue>()
                .map_err(|_| ServerError::ImproperConfigError {
                    cause: format!("allow_origin {} is in improper format...", origin_str),
                })?;
        allow_origins.push(origin);
    }
    let cors = CorsLayer::new()
        .allow_origin(allow_origins)
        .allow_methods(vec![
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_headers(vec![AUTHORIZATION, ACCEPT, CONTENT_TYPE]);
    let app = endpoints::router(app_state).layer(cors);
    let addr: SocketAddr = app_config
        .agent_rest
        .bind_address
        .as_str()
        .parse()
        .map_err(|_| ServerError::ImproperConfigError {
            cause: format!(
                "bind_address {} is in improper format...",
                &app_config.agent_rest.bind_address
            ),
        })?;
    println!(
        "server listening on {}",
        &app_config.agent_rest.bind_address
    );
    // run server
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(|hyper_err| ServerError::ServerError {
            cause: anyhow!(hyper_err),
        })
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };
    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };
    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    println!("signal received, starting graceful shutdown");
}
