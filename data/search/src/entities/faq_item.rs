use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FaqItem {
    pub id: String,
    pub contents: Vec<FaqItemContent>,
    pub categories: Vec<FaqItemCategory>,
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
