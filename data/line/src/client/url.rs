use std::collections::HashMap;
use url::Url;

const BASE_URL: &'static str = "https://api.line.me/v2/bot/";
// const BASE_DATA_URL: &'static str = "https://api-data.line.me/v2/bot/";

pub enum LineBotApiUrl {
    /// Get webhook endpoint information. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#get-webhook-endpoint-information)
    GetWebhookEndpoint,
    /// Test webhook endpoint. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#test-webhook-endpoint)
    TestWebhookEndpoint,
    /// Send push message. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#send-push-message)
    SendPushMessage,
    /// Get profile. [\[detail\]](https://developers.line.biz/en/reference/messaging-api/#get-profile)
    GetProfile { user_id: String },
}

impl LineBotApiUrl {
    pub fn build_url(&self) -> String {
        let url = match self {
            Self::GetWebhookEndpoint => build_url(BASE_URL, Some("channel/webhook/endpoint"), None),
            Self::TestWebhookEndpoint => build_url(BASE_URL, Some("channel/webhook/test"), None),
            Self::SendPushMessage => build_url(BASE_URL, Some("message/push"), None),
            Self::GetProfile { user_id } => {
                let path = format!("profile/{}", user_id);
                build_url(BASE_URL, Some(&path), None)
            },
        };
        url.to_string()
    }
}

fn build_url(base_url: &str, path: Option<&str>, queries: Option<HashMap<&str, &str>>) -> Url {
    let url = Url::parse(base_url).expect("base url must be parsed...");
    let mut url = match path {
        Some(path) => url.join(path).unwrap(),
        None => url,
    };
    if let Some(queries) = queries {
        for (k, v) in queries.into_iter() {
            url.query_pairs_mut().append_pair(k, v);
        }
    }
    url
}
