use std::error::Error;

use serde_json::{from_value, Value};

use crate::{
    bot::client::DiscordClient,
    discord::{Message, RawMessage},
    prelude::*,
};

macro_rules! event_from_raw {
    ($data:ident, $event_type:ident) => {{
        $event_type(from_value($data)?)
    }};
    ($data:ident, $event_type:ident, $client:ident, $raw_type:ty) => {{
        $event_type(from_value::<$raw_type>($data)?.to_mature($client).into())
    }};
}

#[derive(Debug)]
pub enum DispatchedEvent {
    Ready(ReadyEvent),
    MessageCreated(MessageCreatedEvent),
    Unknown { event_name: Box<str>, data: Value },
}

impl DispatchedEvent {
    pub fn from_raw(
        event_name: Box<str>,
        data: Value,
        client: DiscordClient,
    ) -> Result<Self, Box<dyn Error>> {
        use DispatchedEvent::*;
        Ok(match event_name.as_ref() {
            "READY" => event_from_raw!(data, Ready),
            "MESSAGE_CREATE" => event_from_raw!(data, MessageCreated, client, RawMessage),
            _ => Unknown { event_name, data },
        })
    }

    pub fn name(&self) -> &str {
        use DispatchedEvent::*;
        match self {
            Ready(_) => "READY",
            MessageCreated(_) => "MESSAGE_CREATE",
            Unknown { event_name, .. } => &event_name,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReadyEvent {
    pub session_id: Box<str>,
    pub resume_gateway_url: Box<str>,
}

impl From<Message> for MessageCreatedEvent {
    fn from(value: Message) -> Self {
        Self { message: value }
    }
}

#[derive(Debug)]
pub struct MessageCreatedEvent {
    pub message: Message,
}
