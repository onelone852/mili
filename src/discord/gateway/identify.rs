use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionProperties<'a> {
    pub os: &'a str,
    pub browser: &'a str,
    pub device: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifyData<'a> {
    pub token: &'a str,
    pub intents: u64,
    pub properties: ConnectionProperties<'a>,
}
