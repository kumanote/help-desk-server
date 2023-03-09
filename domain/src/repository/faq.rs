use crate::model::{FaqSettings, FaqSettingsData};

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
}
