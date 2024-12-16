pub mod events;

use anyhow::Result;

pub struct Netbird {
    base_url: &'static str,
    client: reqwest::Client,
    api_token: String, // "Token <api-token>"
}

impl Netbird {
    pub fn new(base_url: &'static str, client: reqwest::Client, api_token: &String) -> Self {
        Self {
            base_url,
            client,
            api_token: format!("Token {}", api_token),
        }
    }
    pub async fn get_events(&self) -> Result<Vec<events::Event>> {
        let url = format!("{}/api/events", self.base_url);
        let response = self
            .client
            .get(&url)
            .header("accept", "application/json")
            .header("authorization", &self.api_token)
            .send()
            .await?;
        let events = response.json::<Vec<events::Event>>().await?;
        Ok(events)
    }
}
