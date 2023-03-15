use crate::{model::FaqCategoryId, repository::FaqRepository, Error, Result};
use std::str::FromStr;

pub struct ReorderFaqCategoryUseCaseInput {
    pub id: String,
    pub target_id: String,
    /// append or prepend flag
    /// case true then [id] category will be ordered after [target_id]
    /// case false then [id] category will be ordered before [target_id]
    pub append: bool,
}

pub trait ReorderFaqCategoryUseCase: Send + Sync + 'static {
    type Transaction;
    type FaqRepository: FaqRepository<Err = Error, Transaction = Self::Transaction>;
    fn execute(
        &self,
        tx: &mut Self::Transaction,
        params: ReorderFaqCategoryUseCaseInput,
    ) -> Result<()>;
}

pub struct ReorderFaqCategoryUseCaseImpl<FR: FaqRepository<Err = Error>> {
    faq_repository: FR,
}

impl<FR: FaqRepository<Err = Error>> ReorderFaqCategoryUseCaseImpl<FR> {
    pub fn new(faq_repository: FR) -> Self {
        Self { faq_repository }
    }
}

impl<TX, FR: FaqRepository<Err = Error, Transaction = TX>> ReorderFaqCategoryUseCase
    for ReorderFaqCategoryUseCaseImpl<FR>
{
    type Transaction = TX;
    type FaqRepository = FR;

    fn execute(
        &self,
        tx: &mut Self::Transaction,
        params: ReorderFaqCategoryUseCaseInput,
    ) -> Result<()> {
        let id = FaqCategoryId::from_str(&params.id).map_err(|_| Error::InvalidRequest)?;
        let target_id =
            FaqCategoryId::from_str(&params.target_id).map_err(|_| Error::InvalidRequest)?;
        let objective = self.faq_repository.get_category_by_id(tx, &id)?;
        if objective.is_none() {
            return Err(Error::InvalidRequest);
        }
        let objective = objective.unwrap();
        let target = self.faq_repository.get_category_by_id(tx, &target_id)?;
        if target.is_none() {
            return Err(Error::InvalidRequest);
        }
        let target = target.unwrap();
        self.faq_repository
            .reorder_faq_category(tx, objective, target, params.append)
    }
}
