pub mod bot;
pub mod discord;
pub mod prelude;

#[cfg(test)]
mod test {

    use std::error::Error;

    use crate::discord::SendedMessage;

    use super::prelude::{self, *};

    #[derive(Clone, Default)]
    struct MyBot;

    impl BotImpl for MyBot {
        async fn on_message_created(bot: Bot<Self>, msg: discord::gateway::MessageCreatedEvent) {
            println!("Message created");
            if msg.message.author() != bot.me() {
                msg.message
                    .channel()
                    .await
                    .send(SendedMessage::plain("hello!").tts(true))
                    .await;
            }
        }
    }

    #[prelude::test(flavor = "multi_thread", worker_threads = 5)]
    async fn ok_test() -> Result<(), Box<dyn Error>> {
        BotTemplate::default()
            .api_version(10)
            .intents(
                Intents::GUILDS
                    | Intents::MESSAGE_CONTENT
                    | Intents::DIRECT_MESSAGES
                    | Intents::GUILD_MESSAGES,
            )
            .implement_default::<MyBot>(Token::insecure(
                "MTE5NjAyMDM0MzQ4NDE5MDc5MA.GM8za1.XpcGIbmmvwzCNfl4JiVdEPlBLSE-YciTSDvb8A",
            ))
            .await
    }
}
