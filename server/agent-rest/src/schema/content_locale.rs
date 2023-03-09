use domain::model;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ContentLocale {
    pub value: String,
    pub text: String,
}

impl From<model::ContentLocale> for ContentLocale {
    fn from(value: model::ContentLocale) -> Self {
        let text = value.text().to_string();
        Self {
            value: value.into(),
            text,
        }
    }
}
