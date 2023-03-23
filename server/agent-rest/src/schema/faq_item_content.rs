use super::ContentLocale;
use domain::model;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FaqItemContent {
    pub faq_item_id: String,
    pub locale: ContentLocale,
    pub title: String,
    pub body: String,
}

impl From<model::FaqItemContent> for FaqItemContent {
    fn from(value: model::FaqItemContent) -> Self {
        Self {
            faq_item_id: value.faq_item_id.into(),
            locale: ContentLocale::from(value.locale),
            title: value.title.into(),
            body: value.body.into(),
        }
    }
}
