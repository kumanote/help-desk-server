use crate::{model::FaqSettings, repository::FaqRepository, Error, Result};

pub type GetFaqSettingsUseCaseOutput = FaqSettings;

pub trait GetFaqSettingsUseCase: Send + Sync + 'static {
    type Transaction;
    type FaqRepository: FaqRepository<Err = Error, Transaction = Self::Transaction>;
    fn execute(&self, tx: &mut Self::Transaction) -> Result<GetFaqSettingsUseCaseOutput>;
}

pub struct GetFaqSettingsUseCaseImpl<FR: FaqRepository<Err = Error>> {
    faq_repository: FR,
}

impl<FR: FaqRepository<Err = Error>> GetFaqSettingsUseCaseImpl<FR> {
    pub fn new(faq_repository: FR) -> Self {
        Self { faq_repository }
    }
}

impl<TX, FR: FaqRepository<Err = Error, Transaction = TX>> GetFaqSettingsUseCase
    for GetFaqSettingsUseCaseImpl<FR>
{
    type Transaction = TX;
    type FaqRepository = FR;

    fn execute(&self, tx: &mut Self::Transaction) -> Result<GetFaqSettingsUseCaseOutput> {
        self.faq_repository.get_or_create_default_settings(tx)
    }
}
