use crate::entities::{FaqItem, SearchResults, TaskInfo};
use crate::{Result, SearchClient};
use meilisearch_sdk::search::SearchQuery;

/// primary key for faq_item index
const PRIMARY_KEY: &'static str = "id";
const INDEX_NAME: &'static str = "faq_items";

pub async fn add_or_replace(client: &SearchClient, entity: FaqItem) -> Result<TaskInfo> {
    client
        .index(INDEX_NAME)
        .add_or_replace(&[entity], Some(PRIMARY_KEY))
        .await
        .map_err(Into::into)
}

pub async fn search(
    client: &SearchClient,
    text: &str,
    limit: usize,
    offset: usize,
) -> Result<SearchResults<FaqItem>> {
    let index = client.index(INDEX_NAME);
    let query = SearchQuery::new(&index)
        .with_query(text)
        .with_limit(limit)
        .with_offset(offset)
        .build();
    client
        .index(INDEX_NAME)
        .execute_query(&query)
        .await
        .map_err(Into::into)
}

pub async fn delete_by_id(client: &SearchClient, id: &str) -> Result<TaskInfo> {
    client
        .index(INDEX_NAME)
        .delete_document(id)
        .await
        .map_err(Into::into)
}
