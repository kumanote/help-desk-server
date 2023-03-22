use crate::model::FaqItemContent;

/// Repository for published faq items
pub trait PublicFaqSearchRepository: Send + Sync + 'static {
    type Err;
    fn upsert_faq_item_content(&self, faq_item_content: &FaqItemContent) -> Result<(), Self::Err>;
    fn delete_faq_item_content(&self, faq_item_content: FaqItemContent) -> Result<(), Self::Err>;
}
