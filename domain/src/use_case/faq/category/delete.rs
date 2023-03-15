use crate::{model::FaqCategoryId, repository::FaqRepository, Error, Result};
use std::str::FromStr;

pub struct DeleteFaqCategoryUseCaseInput {
    pub id: String,
}

pub trait DeleteFaqCategoryUseCase: Send + Sync + 'static {
    type Transaction;
    type FaqRepository: FaqRepository<Err = Error, Transaction = Self::Transaction>;
    fn execute(
        &self,
        tx: &mut Self::Transaction,
        params: DeleteFaqCategoryUseCaseInput,
    ) -> Result<()>;
}

pub struct DeleteFaqCategoryUseCaseImpl<FR: FaqRepository<Err = Error>> {
    faq_repository: FR,
}

impl<FR: FaqRepository<Err = Error>> DeleteFaqCategoryUseCaseImpl<FR> {
    pub fn new(faq_repository: FR) -> Self {
        Self { faq_repository }
    }
}

impl<TX, FR: FaqRepository<Err = Error, Transaction = TX>> DeleteFaqCategoryUseCase
    for DeleteFaqCategoryUseCaseImpl<FR>
{
    type Transaction = TX;
    type FaqRepository = FR;

    fn execute(
        &self,
        tx: &mut Self::Transaction,
        params: DeleteFaqCategoryUseCaseInput,
    ) -> Result<()> {
        // validate
        let id = FaqCategoryId::from_str(&params.id).map_err(|_| Error::InvalidRequest)?;
        let category_with_contents = self
            .faq_repository
            .get_category_with_contents_by_id(tx, &id)?;
        if category_with_contents.is_none() {
            return Err(Error::InvalidRequest);
        }
        let category_with_contents = category_with_contents.unwrap();
        // delete
        self.faq_repository
            .delete_category_with_contents(tx, category_with_contents)?;
        Ok(())
    }
}
