use crate::model::{FaqItemWithContentsAndCategories, PagingResult, SearchedFaqItem};

/// Repository for faq items for both unpublished and published
pub trait FaqSearchRepository: Send + Sync + 'static {
    type Err;
    fn upsert_faq_item(&self, faq_item: &FaqItemWithContentsAndCategories)
        -> Result<(), Self::Err>;
    fn delete_faq_item(&self, faq_item: FaqItemWithContentsAndCategories) -> Result<(), Self::Err>;
    fn search_faq_items_by_text(
        &self,
        text: Option<&str>,
        limit: u64,
        offset: u64,
    ) -> Result<PagingResult<SearchedFaqItem>, Self::Err>;
}
