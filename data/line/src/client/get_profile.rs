use super::{client::Client, url::LineBotApiUrl};
use crate::{objects::Profile, LineClient, Result};

impl LineClient {
    pub async fn get_profile(&self, user_id: &str) -> Result<Profile> {
        let url = LineBotApiUrl::GetProfile {
            user_id: user_id.to_owned(),
        }
        .build_url();
        let client = Client::new(&url)?;
        client.get(&self.channel_access_token).await
    }
}
