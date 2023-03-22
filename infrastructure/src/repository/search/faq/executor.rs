use anyhow::anyhow;
use domain::model::FaqItemWithContentsAndCategories;
use domain::repository::FaqSearchRepository;
use futures::executor;
use search::SearchClient;

/// This struct implements FaqSearchRepository.
/// This repository manipulates search engine document objects.
pub struct FaqSearchRepositoryExecutor {
    search_client: SearchClient,
}

impl FaqSearchRepositoryExecutor {
    pub fn new(search_client: SearchClient) -> Self {
        Self { search_client }
    }
}

impl FaqSearchRepository for FaqSearchRepositoryExecutor {
    type Err = domain::Error;

    fn upsert_faq_item(
        &self,
        faq_item: &FaqItemWithContentsAndCategories,
    ) -> Result<(), Self::Err> {
        match executor::block_on(search::adapters::faq_item::add_or_replace(
            &self.search_client,
            faq_item.into(),
        ))? {
            search::entities::Task::Succeeded { content: _ } => Ok(()),
            _ => Err(domain::Error::SystemError {
                cause: anyhow!("failed to complete upsert faq_item meilisearch document."),
            }),
        }
    }

    fn delete_faq_item(&self, faq_item: FaqItemWithContentsAndCategories) -> Result<(), Self::Err> {
        match executor::block_on(search::adapters::faq_item::delete_by_id(
            &self.search_client,
            &faq_item.id,
        ))? {
            search::entities::Task::Succeeded { content: _ } => Ok(()),
            _ => Err(domain::Error::SystemError {
                cause: anyhow!("failed to complete delete faq_item meilisearch document."),
            }),
        }
    }
}
