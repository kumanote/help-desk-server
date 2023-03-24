use crate::{config, AppState, HttpError, Result};
use anyhow::anyhow;
use axum::{
    async_trait,
    body::{Bytes, HttpBody},
    extract::FromRequest,
    headers::{HeaderMap, HeaderValue},
    http::Request,
    BoxError,
};
use base64::{engine::general_purpose, Engine as _};
use hmac::{Hmac, Mac};
use line::events::Events;
use sha2::Sha256;

/// Extract posted line webhook data from request body and
/// validate data using signature extracted from "x-line-signature" header value.
#[derive(Debug)]
pub struct ValidEvents(pub Events);

pub type HmacSha256 = Hmac<Sha256>;

const X_LINE_SIGNATURE: &'static str = "x-line-signature";

fn extract_signature_string(headers: &HeaderMap<HeaderValue>) -> Option<String> {
    if let Some(value) = headers.get(X_LINE_SIGNATURE) {
        if let Ok(s) = value.to_str() {
            Some(s.to_owned())
        } else {
            None
        }
    } else {
        None
    }
}

fn validate_signature(channel_secret: &str, signature: &str, body: &str) -> bool {
    let mut mac = HmacSha256::new_from_slice(channel_secret.as_bytes()).unwrap();
    mac.update(body.as_bytes());
    general_purpose::STANDARD.encode(&mac.finalize().into_bytes().to_vec()) == signature
}

#[async_trait]
impl<B> FromRequest<AppState, B> for ValidEvents
where
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = HttpError;
    async fn from_request(req: Request<B>, state: &AppState) -> Result<Self> {
        let signature = extract_signature_string(req.headers());
        if signature.is_none() {
            return Err(HttpError::Unauthorized);
        }
        let signature = signature.unwrap();
        let body_bytes = Bytes::from_request(req, state).await.map_err(|err| {
            HttpError::InternalServerError {
                cause: anyhow!(err),
            }
        })?;
        let body = std::str::from_utf8(&body_bytes).map_err(|_| HttpError::Unauthorized)?;
        let app_config = config::app_config();
        if !validate_signature(&app_config.line_webhook.channel_secret, &signature, body) {
            return Err(HttpError::Unauthorized);
        }
        let value: Events =
            serde_json::from_str(body).map_err(|err| HttpError::InternalServerError {
                cause: anyhow!(err),
            })?;
        Ok(Self(value))
    }
}
