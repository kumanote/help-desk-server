use anyhow::anyhow;
use domain::model::FaqItemContent;
use domain::repository::PublicFaqSearchRepository;
use futures::executor;
use search::SearchClient;

/// This struct implements PublicFaqSearchRepository.
/// This repository manipulates search engine document objects.
pub struct PublicFaqSearchRepositoryExecutor {
    search_client: SearchClient,
}

impl PublicFaqSearchRepositoryExecutor {
    pub fn new(search_client: SearchClient) -> Self {
        Self { search_client }
    }
}

impl PublicFaqSearchRepository for PublicFaqSearchRepositoryExecutor {
    type Err = domain::Error;

    fn upsert_faq_item_content(&self, faq_item_content: &FaqItemContent) -> Result<(), Self::Err> {
        match executor::block_on(search::adapters::public_faq_item::add_or_replace(
            &self.search_client,
            faq_item_content.into(),
        ))? {
            search::entities::Task::Succeeded { content: _ } => Ok(()),
            _ => Err(domain::Error::SystemError {
                cause: anyhow!("failed to complete upsert public_faq_item meilisearch document."),
            }),
        }
    }

    fn delete_faq_item_content(&self, faq_item_content: FaqItemContent) -> Result<(), Self::Err> {
        match executor::block_on(
            search::adapters::public_faq_item::delete_by_faq_item_id_and_locale(
                &self.search_client,
                &faq_item_content.faq_item_id,
                &faq_item_content.locale,
            ),
        )? {
            search::entities::Task::Succeeded { content: _ } => Ok(()),
            _ => Err(domain::Error::SystemError {
                cause: anyhow!("failed to complete delete public_faq_item meilisearch document."),
            }),
        }
    }
}
