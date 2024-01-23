use serde_json::{from_str, Map, Value};
use tokio::sync::Mutex;

use crate::{
    bot::{connection::Connection, RawBot},
    discord::{token::Token, User},
    prelude::*,
};
use std::error::Error;

#[derive(Debug)]
pub struct BotTemplate {
    pub(crate) intents: Intents,
    pub(crate) api_version: u8,
}

impl Default for BotTemplate {
    fn default() -> Self {
        Self {
            intents: Intents::all(),
            api_version: 10,
        }
    }
}

impl BotTemplate {
    #[inline]
    pub fn intents(mut self, intents: Intents) -> Self {
        self.intents = intents;
        self
    }

    #[inline]
    pub fn api_version(mut self, api_version: u8) -> Self {
        self.api_version = api_version;
        self
    }

    #[inline]
    pub async fn implement_default<Impl>(self, token: Token) -> Result<(), Box<dyn Error>>
    where
        Impl: BotImpl + Send + Sync + Default,
    {
        self.implement(Impl::default(), token).await
    }

    pub async fn implement<Impl>(
        self,
        implementation: Impl,
        token: Token,
    ) -> Result<(), Box<dyn Error>>
    where
        Impl: BotImpl + Send + Sync,
    {
        use reqwest::header;
        let new_token = token.into_inner();
        let auth_token = {
            let mut token = "Bot ".to_string();
            token.push_str(new_token.as_ref());
            header::HeaderValue::try_from(token)
        }?;
        let mut auth = header::HeaderValue::try_from(auth_token)?;
        auth.set_sensitive(true);
        let map = {
            let mut map = header::HeaderMap::new();
            map.insert(header::AUTHORIZATION, auth);
            map.insert(
                header::CONNECTION,
                header::HeaderValue::from_str("Keep-Alive").expect("Should be valid header value"),
            );
            map
        };

        let client = bot::client::DiscordClient::from_raw(
            reqwest::ClientBuilder::default()
                .default_headers(map)
                .user_agent("")
                .build()?,
            self.api_version,
        );

        let gateway = client.get("/gateway/bot").send().await?.text().await?;
        let map = from_str::<Map<String, Value>>(&gateway)?;
        let url = {
            let mut url = map
                .get("url")
                .expect("should have url")
                .as_str()
                .expect("should be str")
                .to_string();
            url.push_str(&format!("/?v={}&encoding=json", self.api_version));
            url
        };
        let raw_user_str = client
            .get("/users/@me")
            .send()
            .await
            .expect("should succeed")
            .text()
            .await
            .expect("should succeed");
        let raw_user = serde_json::from_str(&raw_user_str).expect("should be valid user");
        let me = User::from_raw(raw_user, client.clone());

        let connection = Connection::new(&url).await?;
        let bot = Bot::from_raw(RawBot::<Impl> {
            client,
            token: new_token,
            state: implementation,
            intents: self.intents,
            connection,
            bot: me,
            last_sequence_number: Mutex::new(None),
        });
        bot.run().await?;
        Ok(())
    }
}
