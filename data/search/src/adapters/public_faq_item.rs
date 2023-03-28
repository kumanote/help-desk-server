use super::create_empty_results;
use crate::entities::{PublicFaqItem, SearchResults, Task};
use crate::{Result, SearchClient};
use meilisearch_sdk::search::SearchQuery;

/// primary key for localized public_faq_item index
const PRIMARY_KEY: &'static str = "faq_item_id";

pub async fn add_or_replace(client: &SearchClient, entity: PublicFaqItem) -> Result<Task> {
    let index = build_index_name(&entity.locale);
    client
        .index(index)
        .add_or_replace(&[entity], Some(PRIMARY_KEY))
        .await?
        .wait_for_completion(client, None, None)
        .await
        .map_err(Into::into)
}

pub async fn search(
    client: &SearchClient,
    text: &str,
    locale: &str,
    limit: usize,
    offset: usize,
) -> Result<SearchResults<PublicFaqItem>> {
    let index_name = build_index_name(locale);
    let index = client.index(&index_name);
    let query = SearchQuery::new(&index)
        .with_query(text)
        .with_limit(limit)
        .with_offset(offset)
        .build();
    let query_result = client.index(&index_name).execute_query(&query).await;
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

pub async fn delete_by_faq_item_id_and_locale(
    client: &SearchClient,
    faq_item_id: &str,
    locale: &str,
) -> Result<Task> {
    let index = build_index_name(&locale);
    client
        .index(index)
        .delete_document(faq_item_id)
        .await?
        .wait_for_completion(client, None, None)
        .await
        .map_err(Into::into)
}

fn build_index_name(locale: &str) -> String {
    format!("public_faq_item_{}", locale)
}
