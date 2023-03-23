use crate::{
    model::{FaqItemId, FaqItemWithContentsAndCategories},
    repository::FaqRepository,
    Error, Result,
};
use std::str::FromStr;

pub struct GetFaqItemUseCaseInput {
    pub id: String,
}

pub type GetFaqItemUseCaseOutput = FaqItemWithContentsAndCategories;

pub trait GetFaqItemUseCase: Send + Sync + 'static {
    type Transaction;
    type FaqRepository: FaqRepository<Err = Error, Transaction = Self::Transaction>;
    fn execute(
        &self,
        tx: &mut Self::Transaction,
        params: GetFaqItemUseCaseInput,
    ) -> Result<GetFaqItemUseCaseOutput>;
}

pub struct GetFaqItemUseCaseImpl<FR: FaqRepository<Err = Error>> {
    faq_repository: FR,
}

impl<FR: FaqRepository<Err = Error>> GetFaqItemUseCaseImpl<FR> {
    pub fn new(faq_repository: FR) -> Self {
        Self { faq_repository }
    }
}

impl<TX, FR: FaqRepository<Err = Error, Transaction = TX>> GetFaqItemUseCase
    for GetFaqItemUseCaseImpl<FR>
{
    type Transaction = TX;
    type FaqRepository = FR;

    fn execute(
        &self,
        tx: &mut Self::Transaction,
        params: GetFaqItemUseCaseInput,
    ) -> Result<GetFaqItemUseCaseOutput> {
        let id = FaqItemId::from_str(&params.id).map_err(|_| Error::DataNotFound)?;
        let detail = self
            .faq_repository
            .get_item_with_contents_and_categories_by_id(tx, &id)?;
        if detail.is_none() {
            return Err(Error::DataNotFound);
        }
        Ok(detail.unwrap())
    }
}
