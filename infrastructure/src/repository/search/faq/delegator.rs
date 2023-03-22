use domain::model::FaqItemWithContentsAndCategories;
use domain::repository::FaqSearchRepository;
use queue::QueueConnectionPool;

/// This struct implements FaqSearchRepository.
/// This repository only enqueue the background tasks and dose **not** provide functions
/// that manipulate search engine document objects.
pub struct FaqSearchRepositoryDelegator {
    queue_connection_pool: QueueConnectionPool,
}

impl FaqSearchRepositoryDelegator {
    pub fn new(queue_connection_pool: QueueConnectionPool) -> Self {
        Self {
            queue_connection_pool,
        }
    }
}

impl FaqSearchRepository for FaqSearchRepositoryDelegator {
    type Err = domain::Error;

    fn upsert_faq_item(
        &self,
        faq_item: &FaqItemWithContentsAndCategories,
    ) -> Result<(), Self::Err> {
        let msg = queue::entities::Search::UpsertFaqItem(faq_item.clone().into());
        let mut queue_connection = self.queue_connection_pool.get()?;
        queue::publishers::search::publish_search_engine_task(&mut queue_connection, msg)?;
        Ok(())
    }

    fn delete_faq_item(&self, faq_item: FaqItemWithContentsAndCategories) -> Result<(), Self::Err> {
        let msg = queue::entities::Search::DeleteFaqItem(faq_item.into());
        let mut queue_connection = self.queue_connection_pool.get()?;
        queue::publishers::search::publish_search_engine_task(&mut queue_connection, msg)?;
        Ok(())
    }
}
