use crate::SearchClient;

pub fn new_client(host: impl Into<String>, api_key: impl Into<String>) -> SearchClient {
    SearchClient::new(host, api_key)
}
