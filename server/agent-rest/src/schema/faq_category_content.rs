use super::ContentLocale;
use domain::model;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FaqCategoryContent {
    pub faq_category_id: String,
    pub locale: ContentLocale,
    pub title: String,
}

impl From<model::FaqCategoryContent> for FaqCategoryContent {
    fn from(value: model::FaqCategoryContent) -> Self {
        Self {
            faq_category_id: value.faq_category_id.into(),
            locale: ContentLocale::from(value.locale),
            title: value.title.into(),
        }
    }
}
