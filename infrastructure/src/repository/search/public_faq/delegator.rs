use domain::model::FaqItemContent;
use domain::repository::PublicFaqSearchRepository;
use queue::QueueConnectionPool;

/// This struct implements PublicFaqSearchRepository.
/// This repository only enqueue the background tasks and dose **not** provide functions
/// that manipulate search engine document objects.
pub struct PublicFaqSearchRepositoryDelegator {
    queue_connection_pool: QueueConnectionPool,
}

impl PublicFaqSearchRepositoryDelegator {
    pub fn new(queue_connection_pool: QueueConnectionPool) -> Self {
        Self {
            queue_connection_pool,
        }
    }
}

impl PublicFaqSearchRepository for PublicFaqSearchRepositoryDelegator {
    type Err = domain::Error;

    fn upsert_faq_item_content(&self, faq_item_content: &FaqItemContent) -> Result<(), Self::Err> {
        let msg = queue::entities::Search::UpsertPublicFaqItem {
            id: faq_item_content.faq_item_id.to_string(),
            locale: faq_item_content.locale.to_string(),
            title: faq_item_content.title.to_string(),
            body: faq_item_content.body.text(),
        };
        let mut queue_connection = self.queue_connection_pool.get()?;
        queue::publishers::search::publish_search_engine_task(&mut queue_connection, msg)?;
        Ok(())
    }

    fn delete_faq_item_content(&self, faq_item_content: &FaqItemContent) -> Result<(), Self::Err> {
        let msg = queue::entities::Search::DeletePublicFaqItem {
            id: faq_item_content.faq_item_id.to_string(),
            locale: faq_item_content.locale.to_string(),
        };
        let mut queue_connection = self.queue_connection_pool.get()?;
        queue::publishers::search::publish_search_engine_task(&mut queue_connection, msg)?;
        Ok(())
    }
}
