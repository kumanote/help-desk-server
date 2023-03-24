use super::{FaqCategory, FaqItem};
use domain::model;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FaqCategoryItem {
    pub faq_category_id: String,
    pub faq_item_id: String,
    pub display_order: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<FaqCategory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<FaqItem>,
}

impl From<model::FaqCategoryItem> for FaqCategoryItem {
    fn from(value: model::FaqCategoryItem) -> Self {
        Self {
            faq_category_id: value.faq_category_id.into(),
            faq_item_id: value.faq_item_id.into(),
            display_order: value.display_order,
            category: None,
            item: None,
        }
    }
}

impl From<model::FaqCategoryItemWithCategory> for FaqCategoryItem {
    fn from(value: model::FaqCategoryItemWithCategory) -> Self {
        Self {
            faq_category_id: value.faq_category_id.into(),
            faq_item_id: value.faq_item_id.into(),
            display_order: value.display_order,
            category: Some(FaqCategory::from(value.category)),
            item: None,
        }
    }
}

impl From<model::FaqCategoryItemWithItem> for FaqCategoryItem {
    fn from(value: model::FaqCategoryItemWithItem) -> Self {
        Self {
            faq_category_id: value.faq_category_id.into(),
            faq_item_id: value.faq_item_id.into(),
            display_order: value.display_order,
            category: None,
            item: Some(FaqItem::from(value.item)),
        }
    }
}
