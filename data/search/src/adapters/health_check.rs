use crate::SearchClient;

pub async fn health_check(client: &SearchClient) -> bool {
    if let Ok(health) = client.health().await {
        "available" == health.status
    } else {
        false
    }
}
