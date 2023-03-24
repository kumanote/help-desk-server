use anyhow::anyhow;
use domain::model::{FaqItemWithContentsAndCategories, PagingResult, SearchedFaqItem};
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

    fn search_faq_items_by_text(
        &self,
        text: Option<&str>,
        limit: u64,
        offset: u64,
    ) -> Result<PagingResult<SearchedFaqItem>, Self::Err> {
        let search_results = executor::block_on(search::adapters::faq_item::search(
            &self.search_client,
            text.unwrap_or_default(),
            limit as usize,
            offset as usize,
        ))?;
        // println!("search_results: {:?}", search_results);
        Ok(PagingResult {
            total: search_results.estimated_total_hits.unwrap_or_default() as u64,
            list: search_results
                .hits
                .into_iter()
                .map(|hit| hit.result)
                .collect(),
        })
    }
}
