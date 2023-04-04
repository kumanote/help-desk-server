use crate::{model::InquirySettings, repository::InquiryRepository, Error, Result};

pub type GetInquirySettingsUseCaseOutput = Option<InquirySettings>;

pub trait GetInquirySettingsUseCase: Send + Sync + 'static {
    type Transaction;
    type InquiryRepository: InquiryRepository<Err = Error, Transaction = Self::Transaction>;
    fn execute(&self, tx: &mut Self::Transaction) -> Result<GetInquirySettingsUseCaseOutput>;
}

pub struct GetInquirySettingsUseCaseImpl<FR: InquiryRepository<Err = Error>> {
    inquiry_repository: FR,
}

impl<FR: InquiryRepository<Err = Error>> GetInquirySettingsUseCaseImpl<FR> {
    pub fn new(inquiry_repository: FR) -> Self {
        Self { inquiry_repository }
    }
}

impl<TX, FR: InquiryRepository<Err = Error, Transaction = TX>> GetInquirySettingsUseCase
    for GetInquirySettingsUseCaseImpl<FR>
{
    type Transaction = TX;
    type InquiryRepository = FR;

    fn execute(&self, tx: &mut Self::Transaction) -> Result<GetInquirySettingsUseCaseOutput> {
        self.inquiry_repository.get_settings(tx)
    }
}
