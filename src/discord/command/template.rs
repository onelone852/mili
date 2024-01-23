use std::{convert::Infallible, error::Error};

use futures::TryFutureExt;

use crate::prelude::*;

use self::discord::{
    metadata::{CommandError, CommandMetadata},
    option::{CommandOption, FromCommandOption},
};

use super::localization::Localization;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize)]
#[serde(into = "u8")]
pub enum CommandType {
    SlashCommand,
    UserCommand,
    MessageCommand,
}

impl CommandType {
    pub fn as_u8(&self) -> u8 {
        match self {
            Self::SlashCommand => 1,
            Self::UserCommand => 2,
            Self::MessageCommand => 3,
        }
    }
}

impl From<CommandType> for u8 {
    fn from(value: CommandType) -> Self {
        value.as_u8()
    }
}

#[derive(Debug, Clone, Copy)]
struct NameLocalization<'a>(&'a Localization);

impl<'a> Serialize for NameLocalization<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.localization_with("name").serialize(serializer)
    }
}

#[derive(Debug, Clone, Copy)]
struct DescriptionLocalization<'a>(&'a Localization);

impl<'a> Serialize for DescriptionLocalization<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0
            .localization_with("description")
            .serialize(serializer)
    }
}

#[derive(Debug, Serialize)]
pub struct RawCommand<'a> {
    #[serde(rename = "type")]
    command_type: CommandType,
    name: NameLocalization<'a>,
    description: DescriptionLocalization<'a>,
    nsfw: bool,
}

impl<'a> RawCommand<'a> {
    pub fn slash_command(
        name: &'a Localization,
        description: &'a Localization,
        nsfw: bool,
    ) -> Self {
        Self {
            command_type: CommandType::SlashCommand,
            name: NameLocalization(name),
            description: DescriptionLocalization(description),
            nsfw,
        }
    }
}

pub struct Context;

pub struct Command<Impl: BotImpl, Err: Error> {
    func: Box<
        dyn FnMut(
            Bot<Impl>,
            Context,
            Box<[CommandOption]>,
        ) -> BoxedFuture<Result<(), CommandError<Err>>>,
    >,
    metadata: CommandMetadata,
    command_type: CommandType,
}

macro_rules! to_commandext_creator {
    ($trait_name:ident) => {
        pub trait $trait_name<Impl: BotImpl, Err: Error, CompiledRubbish>: Sized {
            fn to_command(&'static mut self, metadata: CommandMetadata) -> Command<Impl, Err>;
        }
    };
    ($trait_name:ident, $($names:ident), +) => {
        to_commandext_creator!($trait_name);
        to_commandext_creator!($($names), +);
    };
}

to_commandext_creator!(
    ToSlashCommandExt0,
    ToSlashCommandExtFailable0,
    ToSlashCommandExt1
);

impl<F, Fut, Impl> ToSlashCommandExt0<Impl, Infallible, ()> for F
where
    F: FnMut(Bot<Impl>, Context) -> Fut,
    Fut: Future<Output = ()> + Send + 'static,
    Impl: BotImpl,
{
    fn to_command(&'static mut self, metadata: CommandMetadata) -> Command<Impl, Infallible> {
        let func = Box::new(|bot, ctx, options: Box<[CommandOption]>| {
            if options.is_empty() {
                let func = &mut *self;
                func(bot, ctx).map(|_| Ok(())).boxed()
            } else {
                async {
                    Err(CommandError::ParseError(
                        "There should be no arguemnt in this command".into(),
                    ))
                }
                .boxed()
            }
        });
        Command {
            func,
            metadata,
            command_type: CommandType::SlashCommand,
        }
    }
}

impl<F, Fut, Impl, Err> ToSlashCommandExtFailable0<Impl, Err, ()> for F
where
    F: FnMut(Bot<Impl>, Context) -> Fut,
    Fut: Future<Output = Result<(), Err>> + Send + 'static,
    Impl: BotImpl,
    Err: std::error::Error,
{
    fn to_command(&'static mut self, metadata: CommandMetadata) -> Command<Impl, Err> {
        let func = Box::new(|bot, ctx, options: Box<[CommandOption]>| {
            if options.is_empty() {
                let func = &mut *self;

                func(bot, ctx)
                    .map_err(|err| CommandError::Internal(err))
                    .boxed()
            } else {
                async {
                    Err(CommandError::ParseError(
                        "There should be no arguemnt in this command".into(),
                    ))
                }
                .boxed()
            }
        });
        Command {
            func,
            metadata,
            command_type: CommandType::SlashCommand,
        }
    }
}

impl<F, Fut, Impl, Arg> ToSlashCommandExt1<Impl, Infallible, Arg> for F
where
    F: FnMut(Bot<Impl>, Context, Arg) -> Fut,
    Fut: Future<Output = ()> + Send + 'static,
    Impl: BotImpl,
    Arg: FromCommandOption,
    Arg::Error: std::error::Error + Send + 'static,
{
    fn to_command(&'static mut self, metadata: CommandMetadata) -> Command<Impl, Infallible> {
        let func = Box::new(|bot, ctx, options: Box<[CommandOption]>| {
            if options.len() == 1 {
                let mut options_iter = options.into_vec().into_iter();
                let func = &mut *self;
                let arg1 = match Arg::try_from(options_iter.next().expect("should succeed")) {
                    Ok(arg) => arg,
                    Err(err) => {
                        return async { Err(CommandError::ParseError(Box::new(err))) }.boxed()
                    }
                };

                func(bot, ctx, arg1).map(|_| Ok(())).boxed()
            } else {
                async {
                    Err(CommandError::ParseError(
                        "There should be no arguemnt in this command".into(),
                    ))
                }
                .boxed()
            }
        });
        Command {
            func,
            metadata,
            command_type: CommandType::SlashCommand,
        }
    }
}

mod compile_test {
    use super::*;
    use crate::bot::BlanketImpl;

    async fn test(bot: Bot<BlanketImpl>, ctx: Context, is_tts: String) {}

    fn compile_test() {
        test.to_command();
    }
}
