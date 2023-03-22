use crate::{
    model::FaqItemWithContentsAndCategories, repository::FaqSearchRepository, Error, Result,
};

pub type DeleteFaqItemForSearchUseCaseInput = queue::entities::FaqItemWithContentsAndCategories;

pub trait DeleteFaqItemForSearchUseCase: Send + Sync + 'static {
    type FaqSearchRepository: FaqSearchRepository<Err = Error>;
    fn execute(&self, params: DeleteFaqItemForSearchUseCaseInput) -> Result<()>;
}

pub struct DeleteFaqItemForSearchUseCaseImpl<FSR: FaqSearchRepository<Err = Error>> {
    faq_search_repository: FSR,
}

impl<FSR: FaqSearchRepository<Err = Error>> DeleteFaqItemForSearchUseCaseImpl<FSR> {
    pub fn new(faq_search_repository: FSR) -> Self {
        Self {
            faq_search_repository,
        }
    }
}

impl<FSR: FaqSearchRepository<Err = Error>> DeleteFaqItemForSearchUseCase
    for DeleteFaqItemForSearchUseCaseImpl<FSR>
{
    type FaqSearchRepository = FSR;

    fn execute(&self, params: DeleteFaqItemForSearchUseCaseInput) -> Result<()> {
        let faq_item = FaqItemWithContentsAndCategories::from(params);
        self.faq_search_repository.delete_faq_item(faq_item)?;
        Ok(())
    }
}
