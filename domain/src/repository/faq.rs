use crate::model::{
    FaqCategory, FaqCategoryContent, FaqCategoryId, FaqCategoryItem, FaqCategoryItemWithCategory,
    FaqCategoryItemWithItem, FaqCategoryWithContents, FaqItem, FaqItemContent, FaqItemId,
    FaqItemWithContentsAndCategories, FaqSettings, FaqSettingsData, PagingResult, Slug,
};
use chrono::NaiveDateTime;

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
    fn search_categories(
        &self,
        tx: &mut Self::Transaction,
        text: Option<&str>,
        ids: Option<&Vec<&str>>,
        limit: u64,
        offset: u64,
    ) -> Result<PagingResult<FaqCategoryWithContents>, Self::Err>;
    fn get_category_by_id(
        &self,
        tx: &mut Self::Transaction,
        id: &FaqCategoryId,
    ) -> Result<Option<FaqCategory>, Self::Err>;
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
    fn delete_category_with_contents(
        &self,
        tx: &mut Self::Transaction,
        category_with_contents: FaqCategoryWithContents,
    ) -> Result<(), Self::Err>;
    fn reorder_faq_category(
        &self,
        tx: &mut Self::Transaction,
        objective: FaqCategory,
        target: FaqCategory,
        append: bool,
    ) -> Result<(), Self::Err>;
    fn create_item(&self, tx: &mut Self::Transaction, item: &FaqItem) -> Result<(), Self::Err>;
    fn get_item_by_slug(
        &self,
        tx: &mut Self::Transaction,
        slug: &Slug,
    ) -> Result<Option<FaqItem>, Self::Err>;
    fn get_item_with_contents_and_categories_by_id(
        &self,
        tx: &mut Self::Transaction,
        id: &FaqItemId,
    ) -> Result<Option<FaqItemWithContentsAndCategories>, Self::Err>;
    fn search_items_by_category_id(
        &self,
        tx: &mut Self::Transaction,
        category_id: &FaqCategoryId,
        limit: u64,
        offset: u64,
    ) -> Result<PagingResult<FaqCategoryItemWithItem>, Self::Err>;
    fn delete_item_with_contents_and_categories(
        &self,
        tx: &mut Self::Transaction,
        item_with_contents_and_categories: FaqItemWithContentsAndCategories,
    ) -> Result<(), Self::Err>;
    fn update_item_with_contents_and_categories(
        &self,
        tx: &mut Self::Transaction,
        item_with_contents_and_categories: &mut FaqItemWithContentsAndCategories,
        slug: Slug,
        is_published: bool,
        published_at: Option<NaiveDateTime>,
        last_updated_at: Option<NaiveDateTime>,
        contents: Vec<FaqItemContent>,
        categories: Vec<FaqCategoryItemWithCategory>,
    ) -> Result<(), Self::Err>;
    fn create_item_content(
        &self,
        tx: &mut Self::Transaction,
        item_content: &FaqItemContent,
    ) -> Result<(), Self::Err>;
    fn next_category_item_display_order(
        &self,
        tx: &mut Self::Transaction,
        faq_category_id: &FaqCategoryId,
    ) -> Result<u32, Self::Err>;
    fn create_category_item(
        &self,
        tx: &mut Self::Transaction,
        category_item: &FaqCategoryItem,
    ) -> Result<(), Self::Err>;
    fn delete_category_item(
        &self,
        tx: &mut Self::Transaction,
        category_item: FaqCategoryItem,
    ) -> Result<(), Self::Err>;
    fn get_category_item_by_pk(
        &self,
        tx: &mut Self::Transaction,
        faq_category_id: &FaqCategoryId,
        faq_item_id: &FaqItemId,
    ) -> Result<Option<FaqCategoryItem>, Self::Err>;
    fn reorder_category_item(
        &self,
        tx: &mut Self::Transaction,
        objective: FaqCategoryItem,
        target: FaqCategoryItem,
        append: bool,
    ) -> Result<(), Self::Err>;
}
