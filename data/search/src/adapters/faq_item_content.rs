use crate::entities::{FaqItemContent, SearchResults, TaskInfo};
use crate::{Result, SearchClient};
use meilisearch_sdk::search::SearchQuery;

/// primary key for localized faq_item_content index
const PRIMARY_KEY: &'static str = "faq_item_id";

pub async fn add_or_replace(client: &SearchClient, entity: FaqItemContent) -> Result<TaskInfo> {
    let index = build_index_name(&entity.locale);
    client
        .index(index)
        .add_or_replace(&[entity], Some(PRIMARY_KEY))
        .await
        .map_err(Into::into)
}

pub async fn search(
    client: &SearchClient,
    text: &str,
    locale: &str,
    limit: usize,
    offset: usize,
) -> Result<SearchResults<FaqItemContent>> {
    let index_name = build_index_name(locale);
    let index = client.index(&index_name);
    let query = SearchQuery::new(&index)
        .with_query(text)
        .with_limit(limit)
        .with_offset(offset)
        .build();
    client
        .index(&index_name)
        .execute_query(&query)
        .await
        .map_err(Into::into)
}

pub async fn delete_by_faq_item_id_and_locale(
    client: &SearchClient,
    faq_item_id: &str,
    locale: &str,
) -> Result<TaskInfo> {
    let index = build_index_name(&locale);
    client
        .index(index)
        .delete_document(faq_item_id)
        .await
        .map_err(Into::into)
}

fn build_index_name(locale: &str) -> String {
    format!("faq_item_content_{}", locale)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_faq_item_content_crud() {
        dotenv::dotenv().ok();
        let host = std::env::var("MEILISEARCH_HOST").unwrap_or("http://localhost:7700".to_owned());
        let api_key = std::env::var("MEILISEARCH_API_KEY").unwrap_or("MASTER_KEY".to_owned());
        let client = new_client(host, api_key);

        let test_id = "test_faq_item_content_id";
        let locale = "en";

        // add
        let entity = FaqItemContent {
            faq_item_id: test_id.to_owned(),
            locale: locale.to_owned(),
            title: "Test".to_owned(),
            body: json!("This is test..."),
        };
        let task = add_or_replace(&client, entity).await.unwrap();
        assert_eq!(task.status, "enqueued");
        loop {
            let task = client.get_task(task.clone()).await.unwrap();
            if !task.is_pending() {
                assert!(task.is_success());
                break;
            }
        }

        // query
        let results = search(&client, "this", locale, 30, 0).await.unwrap();
        let found = results
            .hits
            .into_iter()
            .find(|hit| hit.result.faq_item_id == test_id);
        assert!(found.is_some());

        // delete
        let task = delete_by_faq_item_id_and_locale(&client, test_id, locale)
            .await
            .unwrap();
        assert_eq!(task.status, "enqueued");
        loop {
            let task = client.get_task(task.clone()).await.unwrap();
            if !task.is_pending() {
                assert!(task.is_success());
                break;
            }
        }

        // query
        let results = search(&client, "this", locale, 30, 0).await.unwrap();
        let found = results
            .hits
            .into_iter()
            .find(|hit| hit.result.faq_item_id == test_id);
        assert!(found.is_none());
    }
}
