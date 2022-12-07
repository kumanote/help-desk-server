use crate::model::{FaqCategoryId, Slug};

#[derive(Debug, Clone)]
pub struct FaqCategory {
    pub id: FaqCategoryId,
    pub slug: Slug,
    pub display_order: u32,
}
