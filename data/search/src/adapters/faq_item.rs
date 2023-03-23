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
    client
        .index(INDEX_NAME)
        .execute_query(&query)
        .await
        .map_err(Into::into)
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

/*
#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::FaqItemContent;
    use crate::*;

    #[tokio::test]
    async fn test_faq_item_crud() {
        dotenv::dotenv().ok();
        let host = std::env::var("MEILISEARCH_HOST").unwrap_or("http://localhost:7700".to_owned());
        let api_key = std::env::var("MEILISEARCH_API_KEY").unwrap_or(
            "MASTER_KEY".to_owned(),
        );
        let client = new_client(host, api_key);

        let test_id = "01GW6HQX6JT2CTTYBBZKSK0RJ4";
        let locale = "en";

        // add
        let entity = FaqItem {
            id: test_id.to_owned(),
            contents: vec![FaqItemContent {
                locale: locale.to_owned(),
                title: "test title".to_owned(),
                body: "heading.\nthis is the test body.".to_owned(),
            }],
            categories: vec![],
        };
        let _task = add_or_replace(&client, entity).await.unwrap();

        // query
        let results = search(&client, "this", 30, 0).await.unwrap();
        let found = results
            .hits
            .into_iter()
            .find(|hit| hit.result.id.as_str() == test_id);
        assert!(found.is_some());

        // delete
        let _task = delete_by_id(&client, test_id).await.unwrap();

        // query
        let results = search(&client, "this", 30, 0).await.unwrap();
        let found = results
            .hits
            .into_iter()
            .find(|hit| hit.result.id.as_str() == test_id);
        assert!(found.is_none());
    }
}
*/
