use crate::model::{FaqCategoryId, FaqItemId, Slug};
use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct FaqItem {
    pub id: FaqItemId,
    pub slug: Slug,
    pub faq_category_id: FaqCategoryId,
    pub is_published: bool,
    pub display_order: u32,
    pub published_at: NaiveDateTime,
    pub last_updated_at: NaiveDateTime,
}
