use crate::model::{FaqCategoryId, FaqCategoryTitle, Locale};

#[derive(Debug, Clone)]
pub struct FaqCategoryContent {
    pub faq_category_id: FaqCategoryId,
    pub locale: Locale,
    pub title: FaqCategoryTitle,
}
