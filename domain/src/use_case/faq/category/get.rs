use crate::{
    model::{FaqCategoryId, FaqCategoryWithContents},
    repository::FaqRepository,
    Error, Result,
};
use std::str::FromStr;

pub struct GetFaqCategoryUseCaseInput {
    pub id: String,
}

pub type GetFaqCategoryUseCaseOutput = FaqCategoryWithContents;

pub trait GetFaqCategoryUseCase: Send + Sync + 'static {
    type Transaction;
    type FaqRepository: FaqRepository<Err = Error, Transaction = Self::Transaction>;
    fn execute(
        &self,
        tx: &mut Self::Transaction,
        params: GetFaqCategoryUseCaseInput,
    ) -> Result<GetFaqCategoryUseCaseOutput>;
}

pub struct GetFaqCategoryUseCaseImpl<FR: FaqRepository<Err = Error>> {
    faq_repository: FR,
}

impl<FR: FaqRepository<Err = Error>> GetFaqCategoryUseCaseImpl<FR> {
    pub fn new(faq_repository: FR) -> Self {
        Self { faq_repository }
    }
}

impl<TX, FR: FaqRepository<Err = Error, Transaction = TX>> GetFaqCategoryUseCase
    for GetFaqCategoryUseCaseImpl<FR>
{
    type Transaction = TX;
    type FaqRepository = FR;

    fn execute(
        &self,
        tx: &mut Self::Transaction,
        params: GetFaqCategoryUseCaseInput,
    ) -> Result<GetFaqCategoryUseCaseOutput> {
        let id = FaqCategoryId::from_str(&params.id).map_err(|_| Error::DataNotFound)?;
        let detail = self
            .faq_repository
            .get_category_with_contents_by_id(tx, &id)?;
        if detail.is_none() {
            return Err(Error::DataNotFound);
        }
        Ok(detail.unwrap())
    }
}
