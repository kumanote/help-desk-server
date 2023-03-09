use super::ContentLocale;
use domain::model;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FaqSettings {
    pub home_url: Option<String>,
    pub supported_locales: Vec<ContentLocale>,
}

impl From<model::FaqSettings> for FaqSettings {
    fn from(value: model::FaqSettings) -> Self {
        Self {
            home_url: value.data.home_url.map(Into::into),
            supported_locales: value
                .data
                .supported_locales
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}
