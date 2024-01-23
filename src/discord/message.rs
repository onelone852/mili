use std::fmt::Debug;

use crate::{bot::client::DiscordClient, prelude::*};

use super::{Channel, OwnedID, RawUser, User, ID};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct RawMessage {
    id: OwnedID,
    channel_id: OwnedID,
    author: RawUser,
    content: String,
    tts: bool,
}

impl RawMessage {
    #[inline]
    pub(crate) fn to_mature(self, client: DiscordClient) -> Message {
        Message {
            id: self.id,
            tts: self.tts,
            author: User::from_raw(self.author, client.clone()),
            content: if self.content.is_empty() {
                None
            } else {
                Some(self.content)
            },
            channel_id: self.channel_id,
            client,
        }
    }
}

#[derive(Debug)]
pub struct Message {
    id: OwnedID,
    channel_id: OwnedID,
    author: User,
    content: Option<String>,
    tts: bool,
    client: DiscordClient,
}

impl Message {
    #[inline]
    pub fn id(&self) -> &ID {
        &self.id
    }

    #[inline]
    pub fn channel_id(&self) -> &ID {
        &self.channel_id
    }

    #[inline]
    pub fn author(&self) -> &User {
        &self.author
    }

    #[inline]
    pub fn content(&self) -> Option<&str> {
        self.content.as_ref().map(String::as_str)
    }

    #[inline]
    pub fn is_tts(&self) -> bool {
        self.tts
    }

    pub async fn channel(&self) -> Channel {
        let route = format!("/channels/{}", self.channel_id());
        let raw_channel = serde_json::from_str(
            &self
                .client
                .get(&route)
                .send()
                .await
                .expect("should succeed")
                .text()
                .await
                .expect("should succeed"),
        )
        .expect("should be valid channel");
        Channel::from_raw(raw_channel, self.client.clone())
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct SendedMessage {
    content: String,
    tts: bool,
}

impl SendedMessage {
    pub fn plain(text: impl Into<String>) -> Self {
        Self {
            content: text.into(),
            tts: false,
        }
    }

    pub fn tts(mut self, tts: bool) -> Self {
        self.tts = tts;
        self
    }
}
