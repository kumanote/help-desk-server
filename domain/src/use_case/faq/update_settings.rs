use crate::{
    model::{FaqContentLocale, FaqSettings, FaqSettingsData, Url},
    repository::FaqRepository,
    Error, Result,
};
use std::str::FromStr;

pub struct UpdateFaqSettingsUseCaseInput {
    pub home_url: Option<String>,
    pub supported_locales: Vec<String>,
}

pub type UpdateFaqSettingsUseCaseOutput = FaqSettings;

pub trait UpdateFaqSettingsUseCase: Send + Sync + 'static {
    type Transaction;
    type FaqRepository: FaqRepository<Err = Error, Transaction = Self::Transaction>;
    fn execute(
        &self,
        tx: &mut Self::Transaction,
        params: UpdateFaqSettingsUseCaseInput,
    ) -> Result<UpdateFaqSettingsUseCaseOutput>;
}

pub struct UpdateFaqSettingsUseCaseImpl<FR: FaqRepository<Err = Error>> {
    faq_repository: FR,
}

impl<FR: FaqRepository<Err = Error>> UpdateFaqSettingsUseCaseImpl<FR> {
    pub fn new(faq_repository: FR) -> Self {
        Self { faq_repository }
    }
}

impl<TX, FR: FaqRepository<Err = Error, Transaction = TX>> UpdateFaqSettingsUseCase
    for UpdateFaqSettingsUseCaseImpl<FR>
{
    type Transaction = TX;
    type FaqRepository = FR;

    fn execute(
        &self,
        tx: &mut Self::Transaction,
        params: UpdateFaqSettingsUseCaseInput,
    ) -> Result<UpdateFaqSettingsUseCaseOutput> {
        let mut settings = self.faq_repository.get_or_create_default_settings(tx)?;
        // validate inputs
        let home_url = if let Some(s) = params.home_url.as_deref() {
            Some(Url::from_str(s)?)
        } else {
            None
        };
        let mut supported_locales = vec![];
        for locale_str in params.supported_locales {
            supported_locales.push(FaqContentLocale::from_str(&locale_str)?)
        }
        // update
        let data = FaqSettingsData {
            home_url,
            supported_locales,
        };
        self.faq_repository
            .update_settings(tx, &mut settings, data)?;
        Ok(settings)
    }
}
