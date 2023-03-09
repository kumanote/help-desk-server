use crate::model::{FaqContentLocale, FaqSettingsId, Url};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct FaqSettings {
    pub id: FaqSettingsId,
    pub data: FaqSettingsData,
}

impl<'a> Into<database::entities::NewFaqSettings<'a>> for &'a FaqSettings {
    fn into(self) -> database::entities::NewFaqSettings<'a> {
        database::entities::NewFaqSettings {
            id: &self.id,
            data: (&self.data).into(),
        }
    }
}

impl From<database::entities::FaqSettings> for FaqSettings {
    fn from(value: database::entities::FaqSettings) -> Self {
        Self {
            id: value.id.into(),
            data: value.data.into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FaqSettingsData {
    /// service url (so that user can go back to service web site)
    pub home_url: Option<Url>,
    pub supported_locales: Vec<FaqContentLocale>,
}

impl From<serde_json::Value> for FaqSettingsData {
    fn from(value: serde_json::Value) -> Self {
        serde_json::from_value(value).unwrap()
    }
}

impl Into<serde_json::Value> for FaqSettingsData {
    fn into(self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}

impl Into<serde_json::Value> for &FaqSettingsData {
    fn into(self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}
