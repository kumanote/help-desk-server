use crate::model::{FaqContentLocale, FaqItemBody, FaqItemId, FaqItemTitle};

#[derive(Debug, Clone)]
pub struct FaqItemContent {
    pub faq_item_id: FaqItemId,
    pub locale: FaqContentLocale,
    pub title: FaqItemTitle,
    pub body: FaqItemBody,
}

impl<'a> Into<database::entities::NewFaqItemContent<'a>> for &'a FaqItemContent {
    fn into(self) -> database::entities::NewFaqItemContent<'a> {
        database::entities::NewFaqItemContent {
            faq_item_id: &self.faq_item_id,
            locale: &self.locale,
            title: &self.title,
            body: (&self.body).into(),
        }
    }
}

impl Into<search::entities::PublicFaqItem> for &FaqItemContent {
    fn into(self) -> search::entities::PublicFaqItem {
        search::entities::PublicFaqItem {
            faq_item_id: self.faq_item_id.to_string(),
            locale: self.locale.to_string(),
            title: self.title.to_string(),
            body: self.body.text(),
        }
    }
}

impl From<database::entities::FaqItemContent> for FaqItemContent {
    fn from(value: database::entities::FaqItemContent) -> Self {
        Self {
            faq_item_id: value.faq_item_id.into(),
            locale: value.locale.into(),
            title: value.title.into(),
            body: value.body.into(),
        }
    }
}

impl From<&database::entities::FaqItemContent> for FaqItemContent {
    fn from(value: &database::entities::FaqItemContent) -> Self {
        Self {
            faq_item_id: value.faq_item_id.clone().into(),
            locale: value.locale.clone().into(),
            title: value.title.clone().into(),
            body: value.body.clone().into(),
        }
    }
}

impl Into<queue::entities::FaqItemContent> for FaqItemContent {
    fn into(self) -> queue::entities::FaqItemContent {
        queue::entities::FaqItemContent {
            faq_item_id: self.faq_item_id.to_string(),
            locale: self.locale.to_string(),
            title: self.title.to_string(),
            body: self.body.into(),
        }
    }
}

impl From<queue::entities::FaqItemContent> for FaqItemContent {
    fn from(value: queue::entities::FaqItemContent) -> Self {
        Self {
            faq_item_id: value.faq_item_id.into(),
            locale: value.locale.into(),
            title: value.title.into(),
            body: value.body.into(),
        }
    }
}
