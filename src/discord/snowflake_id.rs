use std::{borrow::Borrow, fmt::Display, ops::Deref};

use crate::prelude::*;

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq)]
pub struct ID(str);

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnedID(Box<ID>);

impl ID {
    #[inline]
    pub(crate) fn from_raw(id: &str) -> &Self {
        unsafe { &*(id as *const str as *const ID) }
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for ID {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for ID {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl OwnedID {
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }

    #[inline]
    pub fn as_id(&self) -> &ID {
        ID::from_raw(self.as_str())
    }

    #[inline]
    pub fn into_boxed_str(self) -> Box<str> {
        self.0
    }

    #[inline]
    pub fn into_string(self) -> String {
        self.0.into()
    }
}

impl From<OwnedID> for Box<str> {
    #[inline]
    fn from(value: OwnedID) -> Self {
        value.into_boxed_str()
    }
}

impl Display for OwnedID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Borrow<ID> for OwnedID {
    fn borrow(&self) -> &ID {
        self.as_id()
    }
}

impl ToOwned for ID {
    type Owned = OwnedID;

    fn to_owned(&self) -> Self::Owned {
        OwnedID(self.as_str().into())
    }
}

impl Deref for OwnedID {
    type Target = ID;

    fn deref(&self) -> &Self::Target {
        self.as_id()
    }
}
