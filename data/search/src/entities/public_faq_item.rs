use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicFaqItem {
    pub faq_item_id: String,
    pub locale: String,
    pub title: String,
    pub body: String,
}
