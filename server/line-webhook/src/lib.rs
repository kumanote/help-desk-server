mod error;
use error::HttpError;
pub use error::ServerError;
pub type Result<T> = core::result::Result<T, HttpError>;

mod config;
mod endpoints;
mod request_parser;

use anyhow::anyhow;
use hmac::Mac;
use line_webhook_config::AppConfig;
use queue::QueueConnectionPool;
use request_parser::HmacSha256;
use std::net::SocketAddr;
use tokio::signal;

#[derive(Clone)]
pub struct AppState {
    pub queue_connection_pool: QueueConnectionPool,
}

pub async fn start(app_config: AppConfig) -> std::result::Result<(), ServerError> {
    // set global app config
    config::set_app_config(app_config);
    let app_config = config::app_config();

    // check line channel secret
    if let Err(_) = HmacSha256::new_from_slice(app_config.line_webhook.channel_secret.as_bytes()) {
        return Err(ServerError::ImproperConfigError {
            cause: format!(
                "invalid line channel_secret config value: {}",
                &app_config.line_webhook.channel_secret
            ),
        });
    }

    // build global app state
    let queue_connection_pool = queue::new_pool(
        &app_config.queue.url,
        app_config.queue.max_connection_pool_size,
    )?;
    let app_state = AppState {
        queue_connection_pool,
    };
    let app = endpoints::router(app_state);
    let addr: SocketAddr = app_config
        .line_webhook
        .bind_address
        .as_str()
        .parse()
        .map_err(|_| ServerError::ImproperConfigError {
            cause: format!(
                "bind_address {} is in improper format...",
                &app_config.line_webhook.bind_address
            ),
        })?;
    println!(
        "server listening on {}",
        &app_config.line_webhook.bind_address
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
