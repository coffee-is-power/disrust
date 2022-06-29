use serde_json::{Map, Value};

pub use self::text_channel::TextChannel;
use crate::snowflake::Snowflake;
pub mod message;
pub mod text_channel;
pub struct ChannelCommon {
    id: Snowflake,
    position: u64,
    name: String,
}
impl ChannelCommon {
    pub(crate) fn from_json(json: &Map<String, Value>) -> Self {
        ChannelCommon {
            id: json["id"].as_str().unwrap().parse().unwrap(),
            position: json["position"].as_u64().unwrap(),
            name: json["name"].as_str().unwrap().to_string(),
        }
    }
}
pub enum Channel<'a> {
    GuildText(TextChannel<'a>),
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
