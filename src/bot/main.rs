use std::{error::Error, sync::Arc};

use super::{client::DiscordClient, connection::Connection};
use crate::{
    discord::{
        gateway::{
            recover_data::RecoverData, ConnectionProperties, DispatchedEvent, Event, IdentifyData,
            RawEvent, ReadyEvent,
        },
        User,
    },
    prelude::*,
};
use serde_json::{from_str, to_string, to_value};
use tokio::{sync::Mutex, task::JoinHandle};

pub(crate) struct RawBot<Impl> {
    pub(super) token: Box<str>,
    pub(super) state: Impl,
    pub(super) bot: User,
    pub(super) intents: Intents,
    pub(super) client: DiscordClient,
    pub(super) connection: Connection,
    pub(super) last_sequence_number: Mutex<Option<usize>>,
}

pub struct Bot<Impl>(Arc<RawBot<Impl>>);

impl<Impl> Clone for Bot<Impl> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<Impl> Bot<Impl>
where
    Impl: BotImpl,
{
    pub(crate) fn from_raw(bot: RawBot<Impl>) -> Self {
        Self(Arc::new(bot))
    }

    pub fn token(&self) -> &str {
        &self.0.token
    }

    pub fn api_version(&self) -> u8 {
        self.0.client.api_version()
    }

    #[inline]
    pub fn implementation(&self) -> &Impl {
        &self.0.state
    }

    #[inline]
    pub fn intents(&self) -> Intents {
        self.0.intents
    }

    #[inline]
    pub(crate) fn client(&self) -> &DiscordClient {
        &self.0.client
    }

    #[inline]
    pub fn me(&self) -> &User {
        &self.0.bot
    }

    #[inline]
    async fn update_sequence_number(&self, sequence_number: usize) {
        *self.0.last_sequence_number.lock().await = Some(sequence_number);
    }

    #[inline]
    async fn get_seqenuce_number(&self) -> Option<usize> {
        *self.0.last_sequence_number.lock().await
    }

    fn identify_data(&self) -> RawEvent {
        const LIB: &str = "mili";
        let prop = ConnectionProperties {
            os: std::env::consts::OS,
            browser: LIB,
            device: LIB,
        };
        let identify_data = IdentifyData {
            token: self.token(),
            intents: self.intents().as_u64(),
            properties: prop,
        };
        let val = to_value(identify_data).expect("Should succeed");
        RawEvent::new(2, val)
    }
}

impl<Impl> Bot<Impl>
where
    Impl: BotImpl + Send + Sync,
{
    async fn ready(&self) -> Result<RecoverData, Box<dyn Error>> {
        let data = self.identify_data();
        let identify_data = serde_json::to_string(&data).expect("should succeed");
        self.0
            .connection
            .send(identify_data)
            .await
            .expect("Should succeed");

        let ready: Event = from_str::<RawEvent>(
            &self
                .0
                .connection
                .recv()
                .await
                .expect("Should not have closed")?,
        )?
        .try_into_mature(self.client().clone())
        .expect("Should be valid event");
        if let Event::Dispatch {
            sequence_number,
            event,
        } = ready
        {
            if let DispatchedEvent::Ready(ReadyEvent {
                session_id,
                resume_gateway_url,
            }) = event
            {
                tokio::spawn(Impl::on_ready(self.clone()));
                self.update_sequence_number(sequence_number).await;
                Ok(RecoverData {
                    session_id,
                    resume_url: resume_gateway_url,
                })
            } else {
                unimplemented!()
            }
        } else {
            unimplemented!()
        }
    }

    async fn hello(&self) -> Result<JoinHandle<()>, Box<dyn Error>> {
        let hello_event = {
            let event = self
                .0
                .connection
                .recv()
                .await
                .expect("Should not have closed")?;
            from_str::<RawEvent>(&event)
                .or(Err(()))
                .and_then(|event| event.try_into_mature(self.client().clone()))
                .expect("Should be valid event")
        };
        if let Event::Hello { mut heartbeat } = hello_event {
            let bot = self.clone();
            let heartbeater = tokio::spawn(async move {
                heartbeat.tick().await;
                loop {
                    heartbeat.tick().await;
                    let seq_num = *bot.0.last_sequence_number.lock().await;
                    let data = to_value(seq_num).expect("Should be valid");
                    let heartbeat_event = RawEvent::new(1, data);
                    bot.0
                        .connection
                        .send(to_string(&heartbeat_event).expect("Should succeed"))
                        .await
                        .expect("should not closed");
                }
            });
            Ok(heartbeater)
        } else {
            unimplemented!(); // TODO: Return error
        }
    }

    async fn resume_data(&self, resume_data: &RecoverData) -> String {
        let seq_num = self
            .get_seqenuce_number()
            .await
            .map(|num| num.to_string())
            .unwrap_or_else(|| "null".to_string());
        let mut data = String::with_capacity(10);
        data.push_str("{\"op\":6,\"d\":{\"token\":\"");
        data.push_str(self.token());
        data.push_str("\",\"session_id\":\"");
        data.push_str(&resume_data.session_id);
        data.push_str("\",\"seq\":");
        data.push_str(&seq_num);
        data
    }

    async fn resume(
        &self,
        heartbeater: &mut JoinHandle<()>,
        recover_data: &RecoverData,
    ) -> Result<(), Box<dyn Error>> {
        heartbeater.abort();
        self.0
            .connection
            .change_socket(&recover_data.resume_url)
            .await?;
        *heartbeater = self.hello().await?;
        let resume_data = self.resume_data(recover_data).await;
        self.0.connection.send(resume_data).await?;
        Ok(())
    }

    pub(crate) async fn run(self) -> Result<(), Box<dyn Error>> {
        let mut heartbeater = self.hello().await?;
        let recover_data = self.ready().await?;
        loop {
            let event_str = self
                .0
                .connection
                .recv()
                .await
                .expect("Should succeed")
                .expect("Should succeed");

            let event = serde_json::from_str::<RawEvent>(&event_str)
                .expect("Should succeed")
                .try_into_mature(self.client().clone())
                .expect("should be valid event");
            dbg!(&event);
            if let Event::Dispatch {
                sequence_number,
                event,
            } = event
            {
                let bot = self.clone();
                self.update_sequence_number(sequence_number).await;
                tokio::spawn(match event {
                    DispatchedEvent::MessageCreated(msg) => Impl::on_message_created(bot, msg),
                    _ => continue,
                });
            } else if let Event::Reconnect = event {
                self.resume(&mut heartbeater, &recover_data).await?;
            }
        }
    }
}
