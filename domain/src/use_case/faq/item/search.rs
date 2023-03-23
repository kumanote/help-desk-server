use crate::{
    model::{PagingResult, SearchedFaqItem},
    repository::FaqSearchRepository,
    Error, Result,
};

const MAX_LIMIT: u64 = 100;

pub struct SearchFaqItemUseCaseInput {
    pub text: Option<String>,
    pub limit: u64,
    pub offset: u64,
}

pub type SearchFaqItemUseCaseOutput = PagingResult<SearchedFaqItem>;

pub trait SearchFaqItemUseCase: Send + Sync + 'static {
    type FaqSearchRepository: FaqSearchRepository<Err = Error>;
    fn execute(&self, params: SearchFaqItemUseCaseInput) -> Result<SearchFaqItemUseCaseOutput>;
}

pub struct SearchFaqItemUseCaseImpl<FSR: FaqSearchRepository<Err = Error>> {
    faq_search_repository: FSR,
}

impl<FSR: FaqSearchRepository<Err = Error>> SearchFaqItemUseCaseImpl<FSR> {
    pub fn new(faq_search_repository: FSR) -> Self {
        Self {
            faq_search_repository,
        }
    }
}

impl<FSR: FaqSearchRepository<Err = Error>> SearchFaqItemUseCase for SearchFaqItemUseCaseImpl<FSR> {
    type FaqSearchRepository = FSR;

    fn execute(&self, params: SearchFaqItemUseCaseInput) -> Result<SearchFaqItemUseCaseOutput> {
        if params.limit > MAX_LIMIT {
            return Ok(SearchFaqItemUseCaseOutput::default());
        }
        self.faq_search_repository.search_faq_items_by_text(
            params.text.as_deref(),
            params.limit,
            params.offset,
        )
    }
}
