use crate::{bot::client::DiscordClient, prelude::*};

use super::SendedMessage;

#[derive(Debug, Deserialize)]
pub(crate) struct RawChannel {
    id: OwnedID,
    name: Option<Box<str>>,
    #[serde(rename = "nsfw")]
    is_nsfw: Option<bool>,
}

pub struct Channel(RawChannel, DiscordClient);

impl Channel {
    pub(crate) fn from_raw(channel: RawChannel, client: DiscordClient) -> Self {
        Self(channel, client)
    }

    pub fn id(&self) -> &ID {
        &self.0.id
    }

    pub fn name(&self) -> Option<&str> {
        self.0.name.as_ref().map(Box::as_ref)
    }

    pub fn is_nsfw(&self) -> bool {
        self.0.is_nsfw.unwrap_or(false)
    }

    pub async fn send(&self, message: SendedMessage) {
        let route = format!("/channels/{}/messages", self.id().as_str());
        let client = self.1.clone();

        client
            .post(&route)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&message).expect("should succeed"))
            .send()
            .await
            .expect("should succeed");
    }
}
