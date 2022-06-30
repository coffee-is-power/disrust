pub use self::text_channel::TextChannel;
use crate::getter;
use crate::snowflake::Snowflake;
use reqwest::Client;
use serde_json::{Map, Value};
pub mod message;
pub mod text_channel;
#[derive(Debug)]
pub struct ChannelCommon {
    id: Snowflake,
    position: u64,
    name: String,
}
impl ChannelCommon {
    getter!(id -> Snowflake);
    getter!(position -> u64);
    getter!(name -> String);
    pub(crate) fn from_json(json: &Map<String, Value>) -> Self {
        ChannelCommon {
            id: json["id"].as_str().unwrap().parse().unwrap(),
            position: json["position"].as_u64().unwrap(),
            name: json["name"].as_str().unwrap().to_string(),
        }
    }
}
#[derive(Debug)]
pub enum Channel {
    GuildText(TextChannel),
    DM,
    GuildVoice,
    GroupDM,
    GuildCategory,
    GuildNews,
    GuildNewsThread,
    GuildPublicThread,
    GuildPrivateThread,
    GuildStageVoice,
}
impl Channel {
    pub(crate) fn from_json(json: &Map<String, Value>, client: Client) -> Self {
        match json["type"].as_u64().unwrap() {
            0 => Channel::GuildText(TextChannel::from_json(&json, client.clone())),
            1 => Channel::DM,
            2 => Channel::GuildVoice,
            3 => Channel::GroupDM,
            4 => Channel::GuildCategory,
            5 => Channel::GuildNews,
            10 => Channel::GuildNewsThread,
            11 => Channel::GuildPublicThread,
            12 => Channel::GuildPrivateThread,
            13 => Channel::GuildStageVoice,
            _ => unimplemented!("{:#?}", json),
        }
    }
}
