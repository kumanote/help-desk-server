use serde::{Deserialize, Serialize};

/// Parameters for search engine update documents background task
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Search {
    UpsertFaqItem {
        id: String,
        contents: Vec<FaqItemContent>,
        categories: Vec<FaqItemCategory>,
    },
    DeleteFaqItem {
        id: String,
    },
    UpsertPublicFaqItem {
        id: String,
        locale: String,
        title: String,
        body: String,
    },
    DeletePublicFaqItem {
        id: String,
        locale: String,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FaqItemCategory {
    pub locale: String,
    pub title: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FaqItemContent {
    pub locale: String,
    pub title: String,
    pub body: String,
}
