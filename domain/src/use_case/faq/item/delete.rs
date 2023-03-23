use crate::{
    model::FaqItemId,
    repository::{FaqRepository, FaqSearchRepository, PublicFaqSearchRepository},
    Error, Result,
};
use std::str::FromStr;

pub struct DeleteFaqItemUseCaseInput {
    pub id: String,
}

pub trait DeleteFaqItemUseCase: Send + Sync + 'static {
    type Transaction;
    type FaqRepository: FaqRepository<Err = Error, Transaction = Self::Transaction>;
    type FaqSearchRepository: FaqSearchRepository<Err = Error>;
    type PublicFaqSearchRepository: PublicFaqSearchRepository<Err = Error>;
    fn execute(&self, tx: &mut Self::Transaction, params: DeleteFaqItemUseCaseInput) -> Result<()>;
}

pub struct DeleteFaqItemUseCaseImpl<
    FR: FaqRepository<Err = Error>,
    FSR: FaqSearchRepository<Err = Error>,
    PFSR: PublicFaqSearchRepository<Err = Error>,
> {
    faq_repository: FR,
    faq_search_repository: FSR,
    public_faq_search_repository: PFSR,
}

impl<
        FR: FaqRepository<Err = Error>,
        FSR: FaqSearchRepository<Err = Error>,
        PFSR: PublicFaqSearchRepository<Err = Error>,
    > DeleteFaqItemUseCaseImpl<FR, FSR, PFSR>
{
    pub fn new(
        faq_repository: FR,
        faq_search_repository: FSR,
        public_faq_search_repository: PFSR,
    ) -> Self {
        Self {
            faq_repository,
            faq_search_repository,
            public_faq_search_repository,
        }
    }
}

impl<
        TX,
        FR: FaqRepository<Err = Error, Transaction = TX>,
        FSR: FaqSearchRepository<Err = Error>,
        PFSR: PublicFaqSearchRepository<Err = Error>,
    > DeleteFaqItemUseCase for DeleteFaqItemUseCaseImpl<FR, FSR, PFSR>
{
    type Transaction = TX;
    type FaqRepository = FR;
    type FaqSearchRepository = FSR;
    type PublicFaqSearchRepository = PFSR;

    fn execute(&self, tx: &mut Self::Transaction, params: DeleteFaqItemUseCaseInput) -> Result<()> {
        // validate
        let id = FaqItemId::from_str(&params.id).map_err(|_| Error::InvalidRequest)?;
        let faq_item = self
            .faq_repository
            .get_item_with_contents_and_categories_by_id(tx, &id)?;
        if faq_item.is_none() {
            return Err(Error::InvalidRequest);
        }
        let faq_item = faq_item.unwrap();
        let is_published = faq_item.is_published;
        // delete
        self.faq_repository
            .delete_item_with_contents_and_categories(tx, faq_item.clone())?;
        self.faq_search_repository
            .delete_faq_item(faq_item.clone())?;
        if is_published {
            // if published, remove public document from search engine.
            for content in faq_item.contents {
                self.public_faq_search_repository
                    .delete_faq_item_content(content)?;
            }
        }
        Ok(())
    }
}
