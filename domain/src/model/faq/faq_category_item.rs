use crate::model::{FaqCategoryId, FaqItemId};

#[derive(Debug, Clone)]
pub struct FaqCategoryItem {
    pub faq_category_id: FaqCategoryId,
    pub faq_item_id: FaqItemId,
    pub display_order: u32,
}

impl<'a> Into<database::entities::NewFaqCategoryItem<'a>> for &'a FaqCategoryItem {
    fn into(self) -> database::entities::NewFaqCategoryItem<'a> {
        database::entities::NewFaqCategoryItem {
            faq_category_id: &self.faq_category_id,
            faq_item_id: &self.faq_item_id,
            display_order: self.display_order,
        }
    }
}

impl From<database::entities::FaqCategoryItem> for FaqCategoryItem {
    fn from(value: database::entities::FaqCategoryItem) -> Self {
        Self {
            faq_category_id: value.faq_category_id.into(),
            faq_item_id: value.faq_item_id.into(),
            display_order: value.display_order,
        }
    }
}