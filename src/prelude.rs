pub use crate::{
    bot::{self, Bot, BotImpl, BotTemplate},
    discord::{self, intents::Intents, token::Token, OwnedID, ID},
};
pub use futures::{Future, FutureExt};

pub use serde::{self, Deserialize, Serialize};
pub use tokio;

pub use tokio::{main, test};

pub type BoxedFuture<T> = std::pin::Pin<Box<dyn Future<Output = T> + Send>>;
