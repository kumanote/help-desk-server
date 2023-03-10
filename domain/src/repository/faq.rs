use crate::model::{FaqCategory, FaqCategoryContent, FaqSettings, FaqSettingsData, Slug};

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
}
