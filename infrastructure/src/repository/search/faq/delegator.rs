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
        let msg = translate_faq_item_into_upsert_entity(faq_item);
        let mut queue_connection = self.queue_connection_pool.get()?;
        queue::publishers::search::publish_search_engine_task(&mut queue_connection, msg)?;
        Ok(())
    }

    fn delete_faq_item(
        &self,
        faq_item: &FaqItemWithContentsAndCategories,
    ) -> Result<(), Self::Err> {
        let msg = queue::entities::Search::DeleteFaqItem {
            id: faq_item.id.to_string(),
        };
        let mut queue_connection = self.queue_connection_pool.get()?;
        queue::publishers::search::publish_search_engine_task(&mut queue_connection, msg)?;
        Ok(())
    }
}

fn translate_faq_item_into_upsert_entity(
    faq_item: &FaqItemWithContentsAndCategories,
) -> queue::entities::Search {
    let mut categories = vec![];
    for category_with_contents in &faq_item.categories {
        for content in &category_with_contents.category.contents {
            categories.push(queue::entities::FaqItemCategory {
                locale: content.locale.to_string(),
                title: content.title.to_string(),
            })
        }
    }
    queue::entities::Search::UpsertFaqItem {
        id: faq_item.id.to_string(),
        contents: faq_item
            .contents
            .iter()
            .map(|content| queue::entities::FaqItemContent {
                locale: content.locale.to_string(),
                title: content.title.to_string(),
                body: content.body.text(),
            })
            .collect(),
        categories,
    }
}
