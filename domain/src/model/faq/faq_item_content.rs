use crate::model::{FaqItemBody, FaqItemId, FaqItemTitle, Locale};

#[derive(Debug, Clone)]
pub struct FaqItemContent {
    pub faq_item_id: FaqItemId,
    pub locale: Locale,
    pub title: FaqItemTitle,
    pub body: FaqItemBody,
}
