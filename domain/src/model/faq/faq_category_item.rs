use crate::model::{FaqCategoryId, FaqCategoryWithContents, FaqItemId, FaqItemWithContents};

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

impl From<&database::entities::FaqCategoryItem> for FaqCategoryItem {
    fn from(value: &database::entities::FaqCategoryItem) -> Self {
        Self {
            faq_category_id: value.faq_category_id.clone().into(),
            faq_item_id: value.faq_item_id.clone().into(),
            display_order: value.display_order,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FaqCategoryItemWithCategory {
    pub faq_category_id: FaqCategoryId,
    pub faq_item_id: FaqItemId,
    pub display_order: u32,
    pub category: FaqCategoryWithContents,
}

impl From<(FaqCategoryItem, FaqCategoryWithContents)> for FaqCategoryItemWithCategory {
    fn from((category_item, category): (FaqCategoryItem, FaqCategoryWithContents)) -> Self {
        Self {
            faq_category_id: category_item.faq_category_id,
            faq_item_id: category_item.faq_item_id,
            display_order: category_item.display_order,
            category,
        }
    }
}

impl Into<queue::entities::FaqCategoryItemWithCategory> for FaqCategoryItemWithCategory {
    fn into(self) -> queue::entities::FaqCategoryItemWithCategory {
        queue::entities::FaqCategoryItemWithCategory {
            faq_category_id: self.faq_category_id.into(),
            faq_item_id: self.faq_item_id.to_string(),
            display_order: self.display_order,
            category: self.category.into(),
        }
    }
}

impl From<queue::entities::FaqCategoryItemWithCategory> for FaqCategoryItemWithCategory {
    fn from(value: queue::entities::FaqCategoryItemWithCategory) -> Self {
        Self {
            faq_category_id: value.faq_category_id.into(),
            faq_item_id: value.faq_item_id.into(),
            display_order: value.display_order,
            category: value.category.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FaqCategoryItemWithItem {
    pub faq_category_id: FaqCategoryId,
    pub faq_item_id: FaqItemId,
    pub display_order: u32,
    pub item: FaqItemWithContents,
}

impl From<(FaqCategoryItem, FaqItemWithContents)> for FaqCategoryItemWithItem {
    fn from((category_item, item): (FaqCategoryItem, FaqItemWithContents)) -> Self {
        Self {
            faq_category_id: category_item.faq_category_id,
            faq_item_id: category_item.faq_item_id,
            display_order: category_item.display_order,
            item,
        }
    }
}
