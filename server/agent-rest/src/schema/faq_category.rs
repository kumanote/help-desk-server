use super::FaqCategoryContent;
use domain::model;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FaqCategory {
    pub id: String,
    pub slug: String,
    pub display_order: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<Vec<FaqCategoryContent>>,
}

impl From<model::FaqCategory> for FaqCategory {
    fn from(value: model::FaqCategory) -> Self {
        Self {
            id: value.id.into(),
            slug: value.slug.into(),
            display_order: value.display_order,
            contents: None,
        }
    }
}

impl From<model::FaqCategoryWithContents> for FaqCategory {
    fn from(value: model::FaqCategoryWithContents) -> Self {
        Self {
            id: value.id.into(),
            slug: value.slug.into(),
            display_order: value.display_order,
            contents: Some(value.contents.into_iter().map(Into::into).collect()),
        }
    }
}
