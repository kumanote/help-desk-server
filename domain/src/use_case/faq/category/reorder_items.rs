use crate::{
    model::{FaqCategoryId, FaqItemId},
    repository::FaqRepository,
    Error, Result,
};
use std::str::FromStr;

pub struct ReorderFaqItemByCategoryUseCaseInput {
    pub faq_category_id: String,
    pub faq_item_id: String,
    pub target_faq_item_id: String,
    /// append or prepend flag
    /// case true then [faq_item_id] category will be ordered after [target_faq_item_id]
    /// case false then [faq_item_id] category will be ordered before [target_faq_item_id]
    pub append: bool,
}

pub trait ReorderFaqItemByCategoryUseCase: Send + Sync + 'static {
    type Transaction;
    type FaqRepository: FaqRepository<Err = Error, Transaction = Self::Transaction>;
    fn execute(
        &self,
        tx: &mut Self::Transaction,
        params: ReorderFaqItemByCategoryUseCaseInput,
    ) -> Result<()>;
}

pub struct ReorderFaqItemByCategoryUseCaseImpl<FR: FaqRepository<Err = Error>> {
    faq_repository: FR,
}

impl<FR: FaqRepository<Err = Error>> ReorderFaqItemByCategoryUseCaseImpl<FR> {
    pub fn new(faq_repository: FR) -> Self {
        Self { faq_repository }
    }
}

impl<TX, FR: FaqRepository<Err = Error, Transaction = TX>> ReorderFaqItemByCategoryUseCase
    for ReorderFaqItemByCategoryUseCaseImpl<FR>
{
    type Transaction = TX;
    type FaqRepository = FR;

    fn execute(
        &self,
        tx: &mut Self::Transaction,
        params: ReorderFaqItemByCategoryUseCaseInput,
    ) -> Result<()> {
        let faq_category_id =
            FaqCategoryId::from_str(&params.faq_category_id).map_err(|_| Error::InvalidRequest)?;
        let faq_item_id =
            FaqItemId::from_str(&params.faq_item_id).map_err(|_| Error::InvalidRequest)?;
        let target_faq_item_id =
            FaqItemId::from_str(&params.target_faq_item_id).map_err(|_| Error::InvalidRequest)?;
        let objective =
            self.faq_repository
                .get_category_item_by_pk(tx, &faq_category_id, &faq_item_id)?;
        if objective.is_none() {
            return Err(Error::InvalidRequest);
        }
        let objective = objective.unwrap();
        let target = self.faq_repository.get_category_item_by_pk(
            tx,
            &faq_category_id,
            &target_faq_item_id,
        )?;
        if target.is_none() {
            return Err(Error::InvalidRequest);
        }
        let target = target.unwrap();
        self.faq_repository
            .reorder_category_item(tx, objective, target, params.append)
    }
}
