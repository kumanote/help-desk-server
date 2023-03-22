use crate::{model::FaqItemContent, repository::PublicFaqSearchRepository, Error, Result};

pub type UpsertPublicFaqItemForSearchUseCaseInput = queue::entities::FaqItemContent;

pub trait UpsertPublicFaqItemForSearchUseCase: Send + Sync + 'static {
    type PublicFaqSearchRepository: PublicFaqSearchRepository<Err = Error>;
    fn execute(&self, params: UpsertPublicFaqItemForSearchUseCaseInput) -> Result<()>;
}

pub struct UpsertPublicFaqItemForSearchUseCaseImpl<PFSR: PublicFaqSearchRepository<Err = Error>> {
    public_faq_search_repository: PFSR,
}

impl<PFSR: PublicFaqSearchRepository<Err = Error>> UpsertPublicFaqItemForSearchUseCaseImpl<PFSR> {
    pub fn new(public_faq_search_repository: PFSR) -> Self {
        Self {
            public_faq_search_repository,
        }
    }
}

impl<PFSR: PublicFaqSearchRepository<Err = Error>> UpsertPublicFaqItemForSearchUseCase
    for UpsertPublicFaqItemForSearchUseCaseImpl<PFSR>
{
    type PublicFaqSearchRepository = PFSR;

    fn execute(&self, params: UpsertPublicFaqItemForSearchUseCaseInput) -> Result<()> {
        let faq_item_content = FaqItemContent::from(params);
        self.public_faq_search_repository
            .upsert_faq_item_content(&faq_item_content)?;
        Ok(())
    }
}
