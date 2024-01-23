use std::{
    error::Error,
    fmt::{Debug, Display},
    marker::PhantomData,
};

use bitflags::bitflags;

use crate::prelude::*;

use super::localization::Localization;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(into = "u8")]
pub enum CommandOptionsType {
    String,
    Integer,
    Boolean,
    User,
    Channel,
    Role,
    Mentionable,
    Number,
    Attachment,
}

impl CommandOptionsType {
    pub fn as_u8(&self) -> u8 {
        use CommandOptionsType::*;
        match self {
            String => 3,
            Integer => 4,
            Boolean => 5,
            User => 6,
            Channel => 7,
            Role => 8,
            Mentionable => 9,
            Number => 10,
            Attachment => 11,
        }
    }
}

impl From<CommandOptionsType> for u8 {
    fn from(value: CommandOptionsType) -> Self {
        value.as_u8()
    }
}

#[derive(Debug)]
pub struct CommandOptionMetadata {
    name: Localization,
    description: Localization,
    option_type: CommandOptionsType,
    required: bool,
}

pub enum CommandOption {
    Number(f64),
    Integer(i64),
    Boolean(bool),
    String(String),
}

pub struct ConvertError<'a> {
    target: &'a str,
}

impl<'a> Debug for ConvertError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ConvertError: Cannot convert to ")?;
        f.write_str(self.target)
    }
}

impl<'a> Display for ConvertError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<'a> Error for ConvertError<'a> {}

pub trait FromCommandOption {
    const OPTION_TYPE: CommandOptionsType;
    const REQUIRED: bool;
}

impl TryFrom<Option<CommandOption>> for bool {
    type Error = ConvertError<'static>;
    fn try_from(value: Option<CommandOption>) -> Result<Self, Self::Error> {
        if let Ok(CommandOption::Boolean(boolean)) = value {
            Ok(boolean)
        } else {
            Err(ConvertError { target: "boolean" })
        }
    }
}

impl FromCommandOption for bool {
    const OPTION_TYPE: CommandOptionsType = CommandOptionsType::Boolean;
    const REQUIRED: bool = true;
}

impl TryFrom<CommandOption> for Box<str> {
    type Error = ConvertError<'static>;
    fn try_from(value: CommandOption) -> Result<Self, Self::Error> {
        if let CommandOption::String(string) = value {
            Ok(string.into_boxed_str())
        } else {
            Err(ConvertError { target: "string" })
        }
    }
}

impl TryFrom<CommandOption> for String {
    type Error = ConvertError<'static>;
    fn try_from(value: CommandOption) -> Result<Self, Self::Error> {
        if let CommandOption::String(string) = value {
            Ok(string)
        } else {
            Err(ConvertError { target: "string" })
        }
    }
}

impl TryFrom<CommandOption> for f64 {
    type Error = ConvertError<'static>;
    fn try_from(value: CommandOption) -> Result<Self, Self::Error> {
        if let CommandOption::Number(num) = value {
            Ok(num)
        } else {
            Err(ConvertError { target: "f64" })
        }
    }
}

impl TryFrom<CommandOption> for f32 {
    type Error = ConvertError<'static>;
    fn try_from(value: CommandOption) -> Result<Self, Self::Error> {
        if let CommandOption::Number(num) = value {
            Ok(num as f32)
        } else {
            Err(ConvertError { target: "f32" })
        }
    }
}

impl TryFrom<CommandOption> for i64 {
    type Error = ConvertError<'static>;
    fn try_from(value: CommandOption) -> Result<Self, Self::Error> {
        if let CommandOption::Integer(int) = value {
            Ok(int)
        } else {
            Err(ConvertError { target: "i64" })
        }
    }
}
