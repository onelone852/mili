use std::fmt::Display;

use reqwest::RequestBuilder;

#[derive(Debug, Clone)]
pub struct DiscordClient {
    client: reqwest::Client,
    api_version: u8,
}

impl DiscordClient {
    pub fn from_raw(client: reqwest::Client, api_version: u8) -> Self {
        Self {
            client,
            api_version,
        }
    }

    fn api(&self, route: impl Display) -> String {
        format!("https://discord.com/api/v{}/{route}", self.api_version)
    }

    pub fn api_version(&self) -> u8 {
        self.api_version
    }

    pub fn get(&self, route: impl Display) -> RequestBuilder {
        self.client.get(self.api(route))
    }

    pub fn post(&self, route: impl Display) -> RequestBuilder {
        self.client.post(self.api(route))
    }
}
