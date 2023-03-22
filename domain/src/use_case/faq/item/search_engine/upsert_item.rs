use crate::{
    model::FaqItemWithContentsAndCategories, repository::FaqSearchRepository, Error, Result,
};

pub type UpsertFaqItemForSearchUseCaseInput = queue::entities::FaqItemWithContentsAndCategories;

pub trait UpsertFaqItemForSearchUseCase: Send + Sync + 'static {
    type FaqSearchRepository: FaqSearchRepository<Err = Error>;
    fn execute(&self, params: UpsertFaqItemForSearchUseCaseInput) -> Result<()>;
}

pub struct UpsertFaqItemForSearchUseCaseImpl<FSR: FaqSearchRepository<Err = Error>> {
    faq_search_repository: FSR,
}

impl<FSR: FaqSearchRepository<Err = Error>> UpsertFaqItemForSearchUseCaseImpl<FSR> {
    pub fn new(faq_search_repository: FSR) -> Self {
        Self {
            faq_search_repository,
        }
    }
}

impl<FSR: FaqSearchRepository<Err = Error>> UpsertFaqItemForSearchUseCase
    for UpsertFaqItemForSearchUseCaseImpl<FSR>
{
    type FaqSearchRepository = FSR;

    fn execute(&self, params: UpsertFaqItemForSearchUseCaseInput) -> Result<()> {
        let faq_item = FaqItemWithContentsAndCategories::from(params);
        self.faq_search_repository.upsert_faq_item(&faq_item)?;
        Ok(())
    }
}
