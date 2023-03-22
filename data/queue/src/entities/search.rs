use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// Parameters for search engine update documents background task
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Search {
    UpsertFaqItem(FaqItemWithContentsAndCategories),
    DeleteFaqItem(FaqItemWithContentsAndCategories),
    UpsertPublicFaqItem(FaqItemContent),
    DeletePublicFaqItem(FaqItemContent),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FaqItemWithContentsAndCategories {
    pub id: String,
    pub slug: String,
    pub is_published: bool,
    pub published_at: Option<NaiveDateTime>,
    pub last_updated_at: Option<NaiveDateTime>,
    pub contents: Vec<FaqItemContent>,
    pub categories: Vec<FaqCategoryItemWithCategory>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FaqItemContent {
    pub faq_item_id: String,
    pub locale: String,
    pub title: String,
    pub body: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FaqCategoryItemWithCategory {
    pub faq_category_id: String,
    pub faq_item_id: String,
    pub display_order: u32,
    pub category: FaqCategoryWithContents,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FaqCategoryWithContents {
    pub id: String,
    pub slug: String,
    pub display_order: u32,
    pub contents: Vec<FaqCategoryContent>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FaqCategoryContent {
    pub faq_category_id: String,
    pub locale: String,
    pub title: String,
}
