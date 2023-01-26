use crate::{HttpError, Result};
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{header::ACCEPT_LANGUAGE, request::Parts},
};
use domain::model;
use std::str::FromStr;

/// Extract locale from http request headers
#[derive(Debug, Clone, Copy, Default)]
pub struct Locale(pub model::Locale);

#[async_trait]
impl<S> FromRequestParts<S> for Locale
where
    S: Sized + Sync,
{
    type Rejection = HttpError;
    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self> {
        let locale = if let Some(value) = parts.headers.get(ACCEPT_LANGUAGE) {
            if let Ok(s) = value.to_str() {
                model::Locale::from_str(s).unwrap_or_default()
            } else {
                model::Locale::default()
            }
        } else {
            model::Locale::default()
        };
        Ok(Self(locale))
    }
}
