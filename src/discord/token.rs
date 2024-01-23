use std::{env, ffi::OsStr};

pub struct Token(RawToken);

enum RawToken {
    Lazy(fn() -> Box<str>),
    Direct(Box<str>),
}

impl Token {
    pub const ENV: Token = Token(RawToken::Lazy(|| {
        env::var("TOKEN")
            .expect("Cannot access environment variable: TOKEN")
            .into()
    }));

    #[inline]
    pub fn env_custom<K: AsRef<OsStr> + 'static>(key: &K) -> Token {
        Token(RawToken::Direct(
            env::var(key)
                .expect("Cannot access environment variable: TOKEN")
                .into(),
        ))
    }

    pub fn insecure<T: Into<Box<str>>>(token: T) -> Token {
        Token(RawToken::Direct(token.into()))
    }

    pub(crate) fn into_inner(self) -> Box<str> {
        match self.0 {
            RawToken::Lazy(func) => func(),
            RawToken::Direct(s) => s,
        }
    }
}

impl Default for Token {
    fn default() -> Self {
        Self::ENV
    }
}
