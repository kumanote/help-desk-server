use crate::{
    model::{FaqCategoryWithContents, PagingResult},
    repository::FaqRepository,
    Error, Result,
};

const MAX_LIMIT: u64 = 100;

pub struct SearchFaqCategoryUseCaseInput {
    pub text: Option<String>,
    pub ids: Option<Vec<String>>,
    pub limit: u64,
    pub offset: u64,
}

pub type SearchFaqCategoryUseCaseOutput = PagingResult<FaqCategoryWithContents>;

pub trait SearchFaqCategoryUseCase: Send + Sync + 'static {
    type Transaction;
    type FaqRepository: FaqRepository<Err = Error, Transaction = Self::Transaction>;
    fn execute(
        &self,
        tx: &mut Self::Transaction,
        params: SearchFaqCategoryUseCaseInput,
    ) -> Result<SearchFaqCategoryUseCaseOutput>;
}

pub struct SearchFaqCategoryUseCaseImpl<FR: FaqRepository<Err = Error>> {
    faq_repository: FR,
}

impl<FR: FaqRepository<Err = Error>> SearchFaqCategoryUseCaseImpl<FR> {
    pub fn new(faq_repository: FR) -> Self {
        Self { faq_repository }
    }
}

impl<TX, FR: FaqRepository<Err = Error, Transaction = TX>> SearchFaqCategoryUseCase
    for SearchFaqCategoryUseCaseImpl<FR>
{
    type Transaction = TX;
    type FaqRepository = FR;

    fn execute(
        &self,
        tx: &mut Self::Transaction,
        params: SearchFaqCategoryUseCaseInput,
    ) -> Result<SearchFaqCategoryUseCaseOutput> {
        if params.limit > MAX_LIMIT {
            return Ok(SearchFaqCategoryUseCaseOutput::default());
        }
        let ids: Option<Vec<&str>> = params
            .ids
            .as_ref()
            .map(|ids| ids.iter().map(String::as_str).collect());
        self.faq_repository.search_categories(
            tx,
            params.text.as_deref(),
            ids.as_ref(),
            params.limit,
            params.offset,
        )
    }
}
