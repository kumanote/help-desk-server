use crate::{model::FaqItemContent, repository::PublicFaqSearchRepository, Error, Result};

pub type DeletePublicFaqItemForSearchUseCaseInput = queue::entities::FaqItemContent;

pub trait DeletePublicFaqItemForSearchUseCase: Send + Sync + 'static {
    type PublicFaqSearchRepository: PublicFaqSearchRepository<Err = Error>;
    fn execute(&self, params: DeletePublicFaqItemForSearchUseCaseInput) -> Result<()>;
}

pub struct DeletePublicFaqItemForSearchUseCaseImpl<PFSR: PublicFaqSearchRepository<Err = Error>> {
    public_faq_search_repository: PFSR,
}

impl<PFSR: PublicFaqSearchRepository<Err = Error>> DeletePublicFaqItemForSearchUseCaseImpl<PFSR> {
    pub fn new(public_faq_search_repository: PFSR) -> Self {
        Self {
            public_faq_search_repository,
        }
    }
}

impl<PFSR: PublicFaqSearchRepository<Err = Error>> DeletePublicFaqItemForSearchUseCase
    for DeletePublicFaqItemForSearchUseCaseImpl<PFSR>
{
    type PublicFaqSearchRepository = PFSR;

    fn execute(&self, params: DeletePublicFaqItemForSearchUseCaseInput) -> Result<()> {
        let faq_item_content = FaqItemContent::from(params);
        self.public_faq_search_repository
            .delete_faq_item_content(faq_item_content)?;
        Ok(())
    }
}
