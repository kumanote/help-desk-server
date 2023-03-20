use crate::model::FaqItemWithContentsAndCategories;

/// Repository for faq items for both unpublished and published
pub trait FaqSearchRepository: Send + Sync + 'static {
    type Err;
    fn upsert_faq_item(&self, faq_item: &FaqItemWithContentsAndCategories)
        -> Result<(), Self::Err>;
    fn delete_faq_item(&self, faq_item: &FaqItemWithContentsAndCategories)
        -> Result<(), Self::Err>;
}
