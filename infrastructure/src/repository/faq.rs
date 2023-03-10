use database::DbConnection;
use domain::model::{FaqCategory, FaqCategoryContent, FaqSettings, FaqSettingsData, Slug};
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
}
