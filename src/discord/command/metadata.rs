pub use crate::prelude::*;

use std::{
    error::Error,
    fmt::{Debug, Display},
};

use super::localization::Localization;

#[derive(Debug)]
pub enum CommandError<E> {
    ParseError(Box<dyn Error>),
    Internal(E),
}

impl<E> Display for CommandError<E>
where
    E: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseError(err) => write!(f, "ParseError {:?}", err),
            Self::Internal(err) => write!(f, "InternalError {:?}", err),
        }
    }
}

impl<E> std::error::Error for CommandError<E> where E: std::error::Error {}

pub struct CommandMetadata {
    pub name: Localization,
    pub description: Localization,
    pub nsfw: bool,
}

impl CommandMetadata {
    pub fn new(name: impl Into<Localization>, description: impl Into<Localization>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            nsfw: false,
        }
    }
}
