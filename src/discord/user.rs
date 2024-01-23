use crate::{bot::client::DiscordClient, prelude::*};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct RawUser {
    id: OwnedID,
    username: String,
}

#[derive(Debug, Clone)]
pub struct User(RawUser, DiscordClient);

impl User {
    pub(crate) fn from_raw(user: RawUser, client: DiscordClient) -> Self {
        User(user, client)
    }

    pub fn id(&self) -> &ID {
        &self.0.id
    }

    pub fn username(&self) -> &str {
        &self.0.username
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Eq for User {}
