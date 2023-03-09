use database::DbConnection;
use domain::model::{FaqSettings, FaqSettingsData};
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
}
