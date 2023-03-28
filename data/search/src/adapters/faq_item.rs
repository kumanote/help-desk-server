use super::create_empty_results;
use crate::entities::{FaqItem, SearchResults, Task};
use crate::{Result, SearchClient};
use meilisearch_sdk::search::SearchQuery;

/// primary key for faq_item index
const PRIMARY_KEY: &'static str = "id";
const INDEX_NAME: &'static str = "faq_items";

pub async fn add_or_replace(client: &SearchClient, entity: FaqItem) -> Result<Task> {
    client
        .index(INDEX_NAME)
        .add_or_replace(&[entity], Some(PRIMARY_KEY))
        .await?
        .wait_for_completion(client, None, None)
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
    let query_result = client.index(INDEX_NAME).execute_query(&query).await;
    match query_result {
        Ok(search_results) => Ok(search_results),
        Err(err) => match &err {
            meilisearch_sdk::errors::Error::Meilisearch(meilisearch_error) => {
                if meilisearch_error.error_code == meilisearch_sdk::errors::ErrorCode::IndexNotFound
                {
                    Ok(create_empty_results())
                } else {
                    Err(err.into())
                }
            },
            _ => Err(err.into()),
        },
    }
}

pub async fn delete_by_id(client: &SearchClient, id: &str) -> Result<Task> {
    client
        .index(INDEX_NAME)
        .delete_document(id)
        .await?
        .wait_for_completion(client, None, None)
        .await
        .map_err(Into::into)
}
