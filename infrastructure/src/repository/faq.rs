use database::DbConnection;
use domain::model::{
    FaqCategory, FaqCategoryContent, FaqCategoryWithContents, FaqSettings, FaqSettingsData,
    PagingResult, Slug,
};
use domain::repository::FaqRepository;

pub struct FaqRepositoryImpl;

impl FaqRepository for FaqRepositoryImpl {
    type Err = domain::Error;
    type Transaction = DbConnection;

    fn get_or_create_default_settings(
        &self,
        tx: &mut Self::Transaction,
    ) -> Result<FaqSettings, Self::Err> {
        if let Some(entity) = database::adapters::faq_settings::get(tx)? {
            Ok(entity.into())
        } else {
            let new_model = FaqSettings::default();
            database::adapters::faq_settings::create(tx, (&new_model).into())?;
            Ok(new_model)
        }
    }

    fn update_settings(
        &self,
        tx: &mut Self::Transaction,
        settings: &mut FaqSettings,
        data: FaqSettingsData,
    ) -> Result<(), Self::Err> {
        database::adapters::faq_settings::update(tx, &(&data).into(), &settings.id)?;
        settings.data = data;
        Ok(())
    }

    fn create_category(
        &self,
        tx: &mut Self::Transaction,
        category: &FaqCategory,
    ) -> Result<(), Self::Err> {
        database::adapters::faq_category::create(tx, category.into())?;
        Ok(())
    }

    fn next_category_display_order(&self, tx: &mut Self::Transaction) -> Result<u32, Self::Err> {
        let current_max = database::adapters::faq_category::get_max_display_order(tx)?.unwrap_or(0);
        Ok(current_max + 1)
    }

    fn create_category_content(
        &self,
        tx: &mut Self::Transaction,
        category_content: &FaqCategoryContent,
    ) -> Result<(), Self::Err> {
        database::adapters::faq_category_content::create(tx, category_content.into())?;
        Ok(())
    }

    fn get_category_by_slug(
        &self,
        tx: &mut Self::Transaction,
        slug: &Slug,
    ) -> Result<Option<FaqCategory>, Self::Err> {
        let entity = database::adapters::faq_category::get_by_slug(tx, &slug)?;
        Ok(entity.map(Into::into))
    }

    fn search_categories_by_text(
        &self,
        tx: &mut Self::Transaction,
        text: Option<&str>,
        limit: u64,
        offset: u64,
    ) -> Result<PagingResult<FaqCategoryWithContents>, Self::Err> {
        let (total, category_entities) = database::adapters::faq_category::search_by_text(
            tx,
            text,
            limit as i64,
            offset as i64,
        )?;
        let category_ids: Vec<&str> = category_entities
            .iter()
            .map(|category| category.id.as_str())
            .collect();
        let content_entities =
            database::adapters::faq_category_content::get_list_by_faq_category_ids(
                tx,
                &category_ids,
            )?;
        let list = category_entities
            .into_iter()
            .map(|category_entity| {
                let contents = content_entities
                    .iter()
                    .filter(|c| c.faq_category_id.as_str() == category_entity.id.as_str())
                    .map(FaqCategoryContent::from)
                    .collect();
                let category = FaqCategory::from(category_entity);
                FaqCategoryWithContents::from((category, contents))
            })
            .collect();
        Ok(PagingResult {
            total: total as u64,
            list,
        })
    }
}
