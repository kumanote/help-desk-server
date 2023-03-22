use crate::model::{FaqCategoryId, FaqCategoryTitle, FaqContentLocale};

#[derive(Debug, Clone)]
pub struct FaqCategoryContent {
    pub faq_category_id: FaqCategoryId,
    pub locale: FaqContentLocale,
    pub title: FaqCategoryTitle,
}

impl<'a> Into<database::entities::NewFaqCategoryContent<'a>> for &'a FaqCategoryContent {
    fn into(self) -> database::entities::NewFaqCategoryContent<'a> {
        database::entities::NewFaqCategoryContent {
            faq_category_id: &self.faq_category_id,
            locale: &self.locale,
            title: &self.title,
        }
    }
}

impl From<database::entities::FaqCategoryContent> for FaqCategoryContent {
    fn from(value: database::entities::FaqCategoryContent) -> Self {
        Self {
            faq_category_id: value.faq_category_id.into(),
            locale: value.locale.into(),
            title: value.title.into(),
        }
    }
}

impl From<&database::entities::FaqCategoryContent> for FaqCategoryContent {
    fn from(value: &database::entities::FaqCategoryContent) -> Self {
        Self {
            faq_category_id: value.faq_category_id.clone().into(),
            locale: value.locale.clone().into(),
            title: value.title.clone().into(),
        }
    }
}

impl Into<queue::entities::FaqCategoryContent> for FaqCategoryContent {
    fn into(self) -> queue::entities::FaqCategoryContent {
        queue::entities::FaqCategoryContent {
            faq_category_id: self.faq_category_id.into(),
            locale: self.locale.into(),
            title: self.title.into(),
        }
    }
}

impl From<queue::entities::FaqCategoryContent> for FaqCategoryContent {
    fn from(value: queue::entities::FaqCategoryContent) -> Self {
        Self {
            faq_category_id: value.faq_category_id.into(),
            locale: value.locale.into(),
            title: value.title.into(),
        }
    }
}
