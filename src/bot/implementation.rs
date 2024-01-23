use futures::Future;

use crate::discord::gateway::MessageCreatedEvent;

use super::{Bot, CommandRegister};

pub trait BotImpl: 'static + Sized {
    fn on_ready(_: Bot<Self>) -> impl Future<Output = ()> + Send {
        async { println!("[MILI] The bot is ready!") }
    }

    fn on_message_created(_: Bot<Self>, _: MessageCreatedEvent) -> impl Future<Output = ()> + Send {
        async {}
    }

    fn command_register(_: &mut CommandRegister) {}
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct BlanketImpl;

impl BotImpl for BlanketImpl {}
