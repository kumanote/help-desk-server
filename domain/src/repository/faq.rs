use crate::model::{
    FaqCategory, FaqCategoryContent, FaqCategoryId, FaqCategoryWithContents, FaqSettings,
    FaqSettingsData, PagingResult, Slug,
};

pub trait FaqRepository: Send + Sync + 'static {
    type Err;
    type Transaction;
    fn get_or_create_default_settings(
        &self,
        tx: &mut Self::Transaction,
    ) -> Result<FaqSettings, Self::Err>;
    fn update_settings(
        &self,
        tx: &mut Self::Transaction,
        settings: &mut FaqSettings,
        data: FaqSettingsData,
    ) -> Result<(), Self::Err>;
    fn create_category(
        &self,
        tx: &mut Self::Transaction,
        category: &FaqCategory,
    ) -> Result<(), Self::Err>;
    fn next_category_display_order(&self, tx: &mut Self::Transaction) -> Result<u32, Self::Err>;
    fn create_category_content(
        &self,
        tx: &mut Self::Transaction,
        category_content: &FaqCategoryContent,
    ) -> Result<(), Self::Err>;
    fn get_category_by_slug(
        &self,
        tx: &mut Self::Transaction,
        slug: &Slug,
    ) -> Result<Option<FaqCategory>, Self::Err>;
    fn search_categories_by_text(
        &self,
        tx: &mut Self::Transaction,
        text: Option<&str>,
        limit: u64,
        offset: u64,
    ) -> Result<PagingResult<FaqCategoryWithContents>, Self::Err>;
    fn get_category_with_contents_by_id(
        &self,
        tx: &mut Self::Transaction,
        id: &FaqCategoryId,
    ) -> Result<Option<FaqCategoryWithContents>, Self::Err>;
    fn update_category_with_contents(
        &self,
        tx: &mut Self::Transaction,
        category_with_contents: &mut FaqCategoryWithContents,
        slug: Slug,
        contents: Vec<FaqCategoryContent>,
    ) -> Result<(), Self::Err>;
}
