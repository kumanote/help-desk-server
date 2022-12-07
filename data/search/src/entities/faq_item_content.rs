use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FaqItemContent {
    pub faq_item_id: String,
    pub locale: String,
    pub title: String,
    pub body: serde_json::Value,
}
