use std::time::Duration;

use serde_json::Value;
use tokio::time::Interval;

use crate::{bot::client::DiscordClient, discord::gateway::DispatchedEvent, prelude::*};

#[derive(Debug, Serialize, Deserialize)]
pub struct RawEvent {
    #[serde(rename = "op")]
    opcode: u8,
    #[serde(rename = "t")]
    event_name: Option<Box<str>>,
    #[serde(rename = "s")]
    sequence_number: Option<usize>,
    #[serde(rename = "d")]
    data: Value,
}

impl RawEvent {
    pub fn new(opcode: u8, data: Value) -> Self {
        Self {
            opcode,
            data,
            event_name: None,
            sequence_number: None,
        }
    }

    pub fn opcode(&self) -> u8 {
        self.opcode
    }

    pub fn data(&self) -> &Value {
        &self.data
    }

    #[inline]
    pub fn try_into_mature(self, client: DiscordClient) -> Result<Event, ()> {
        Event::from_raw(self, client)
    }
}

#[derive(Debug)]
pub enum Event {
    Dispatch {
        sequence_number: usize,
        event: DispatchedEvent,
    },
    Hello {
        heartbeat: Interval,
    },
    Reconnect,
    HeartbeatACK,
}

impl Event {
    #[inline]
    pub fn from_raw(
        event: RawEvent,
        client: DiscordClient,
    ) -> Result<Self, <Self as TryFrom<(RawEvent, DiscordClient)>>::Error> {
        Self::try_from((event, client))
    }
}

impl TryFrom<(RawEvent, DiscordClient)> for Event {
    type Error = ();

    fn try_from(value: (RawEvent, DiscordClient)) -> Result<Self, Self::Error> {
        use Event::*;

        match value.0.opcode() {
            0 => {
                let RawEvent {
                    sequence_number,
                    data,
                    event_name,
                    ..
                } = value.0;
                let seq_num = sequence_number.ok_or(())?; // TODO: Change to error
                let name = event_name.ok_or(())?; // TODO: Change to error
                let dispatch = DispatchedEvent::from_raw(name, data, value.1).or(Err(()))?; // TODO: Change to error
                Ok(Dispatch {
                    sequence_number: seq_num,
                    event: dispatch,
                })
            }
            7 => Ok(Reconnect),
            10 => {
                let heartbeat = value
                    .0
                    .data()
                    .as_object()
                    .ok_or(())? // TODO: Change to error
                    .get("heartbeat_interval")
                    .ok_or(())? // TODO: Change to error
                    .as_u64()
                    .ok_or(())?; // TODO: Change to error
                let interval = tokio::time::interval(Duration::from_millis(heartbeat));
                Ok(Hello {
                    heartbeat: interval,
                })
            }
            11 => Ok(HeartbeatACK),
            _ => unimplemented!(), // TODO
        }
    }
}
