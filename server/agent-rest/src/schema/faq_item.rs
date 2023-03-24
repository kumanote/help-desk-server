use super::{FaqCategoryItem, FaqItemContent};
use domain::model;
use serde::Serialize;

pub type SearchedFaqItem = model::SearchedFaqItem;

#[derive(Debug, Serialize)]
pub struct FaqItem {
    pub id: String,
    pub slug: String,
    pub is_published: bool,
    pub published_at: Option<i64>,
    pub last_updated_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<Vec<FaqItemContent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<FaqCategoryItem>>,
}

impl From<model::FaqItem> for FaqItem {
    fn from(value: model::FaqItem) -> Self {
        Self {
            id: value.id.into(),
            slug: value.slug.into(),
            is_published: value.is_published,
            published_at: value.published_at.map(|dt| dt.timestamp()),
            last_updated_at: value.last_updated_at.map(|dt| dt.timestamp()),
            contents: None,
            categories: None,
        }
    }
}

impl From<model::FaqItemWithContentsAndCategories> for FaqItem {
    fn from(value: model::FaqItemWithContentsAndCategories) -> Self {
        Self {
            id: value.id.into(),
            slug: value.slug.into(),
            is_published: value.is_published,
            published_at: value.published_at.map(|dt| dt.timestamp()),
            last_updated_at: value.last_updated_at.map(|dt| dt.timestamp()),
            contents: Some(value.contents.into_iter().map(Into::into).collect()),
            categories: Some(value.categories.into_iter().map(Into::into).collect()),
        }
    }
}

impl From<model::FaqItemWithContents> for FaqItem {
    fn from(value: model::FaqItemWithContents) -> Self {
        Self {
            id: value.id.into(),
            slug: value.slug.into(),
            is_published: value.is_published,
            published_at: value.published_at.map(|dt| dt.timestamp()),
            last_updated_at: value.last_updated_at.map(|dt| dt.timestamp()),
            contents: Some(value.contents.into_iter().map(Into::into).collect()),
            categories: None,
        }
    }
}
