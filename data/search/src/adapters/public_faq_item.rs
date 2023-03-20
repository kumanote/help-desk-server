use crate::entities::{PublicFaqItem, SearchResults, TaskInfo};
use crate::{Result, SearchClient};
use meilisearch_sdk::search::SearchQuery;

/// primary key for localized public_faq_item index
const PRIMARY_KEY: &'static str = "faq_item_id";

pub async fn add_or_replace(client: &SearchClient, entity: PublicFaqItem) -> Result<TaskInfo> {
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
) -> Result<SearchResults<PublicFaqItem>> {
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
    format!("public_faq_item_{}", locale)
}
