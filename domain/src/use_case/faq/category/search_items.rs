use crate::model::FaqCategoryId;
use crate::{
    model::{FaqCategoryItemWithItem, PagingResult},
    repository::FaqRepository,
    Error, Result,
};
use std::str::FromStr;

const MAX_LIMIT: u64 = 100;

pub struct SearchFaqItemByCategoryUseCaseInput {
    pub faq_category_id: String,
    pub limit: u64,
    pub offset: u64,
}

pub type SearchFaqItemByCategoryUseCaseOutput = PagingResult<FaqCategoryItemWithItem>;

pub trait SearchFaqItemByCategoryUseCase: Send + Sync + 'static {
    type Transaction;
    type FaqRepository: FaqRepository<Err = Error, Transaction = Self::Transaction>;
    fn execute(
        &self,
        tx: &mut Self::Transaction,
        params: SearchFaqItemByCategoryUseCaseInput,
    ) -> Result<SearchFaqItemByCategoryUseCaseOutput>;
}

pub struct SearchFaqItemByCategoryUseCaseImpl<FR: FaqRepository<Err = Error>> {
    faq_repository: FR,
}

impl<FR: FaqRepository<Err = Error>> SearchFaqItemByCategoryUseCaseImpl<FR> {
    pub fn new(faq_repository: FR) -> Self {
        Self { faq_repository }
    }
}

impl<TX, FR: FaqRepository<Err = Error, Transaction = TX>> SearchFaqItemByCategoryUseCase
    for SearchFaqItemByCategoryUseCaseImpl<FR>
{
    type Transaction = TX;
    type FaqRepository = FR;

    fn execute(
        &self,
        tx: &mut Self::Transaction,
        params: SearchFaqItemByCategoryUseCaseInput,
    ) -> Result<SearchFaqItemByCategoryUseCaseOutput> {
        if params.limit > MAX_LIMIT {
            return Ok(SearchFaqItemByCategoryUseCaseOutput::default());
        }
        let category_id =
            FaqCategoryId::from_str(&params.faq_category_id).map_err(|_| Error::InvalidRequest)?;
        self.faq_repository.search_items_by_category_id(
            tx,
            &category_id,
            params.limit,
            params.offset,
        )
    }
}
