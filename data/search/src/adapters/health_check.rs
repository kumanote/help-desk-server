use crate::SearchClient;

pub async fn health_check(client: &SearchClient) -> bool {
    if let Ok(health) = client.health().await {
        "available" == health.status
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[tokio::test]
    async fn test_health_check() {
        dotenv::dotenv().ok();
        let host = std::env::var("MEILISEARCH_HOST").unwrap_or("http://localhost:7700".to_owned());
        let api_key = std::env::var("MEILISEARCH_API_KEY").unwrap_or("MASTER_KEY".to_owned());
        let client = new_client(host, api_key);
        assert!(health_check(&client).await);
    }
}
