use crate::{AppState, HttpError, Result};
use axum::extract::State;
use serde_json::json;

pub async fn handler(State(state): State<AppState>) -> Result<&'static str> {
    let db_ok = if let Ok(mut conn) = state.db_connection_pool.get() {
        database::adapters::health_check(&mut conn)
    } else {
        false
    };
    if !db_ok {
        return Err(HttpError::ServiceUnavailable {
            detail: json!("Oops... currently the system is unavailable..."),
        }
        .into());
    }

    let cache_ok = if let Ok(mut conn) = state.cache_connection_pool.get() {
        cache::adapters::health_check(&mut conn)
    } else {
        false
    };
    if !cache_ok {
        return Err(HttpError::ServiceUnavailable {
            detail: json!("Oops... currently the system is unavailable..."),
        }
        .into());
    }
    Ok("OK")
}
