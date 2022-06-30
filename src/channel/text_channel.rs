use reqwest::{Client, header::{AUTHORIZATION, HeaderValue}, Method};
use serde_json::{Map, Value};

use super::{message::Message, ChannelCommon};
use crate::snowflake::Snowflake;

pub struct TextChannel {
    channel_common: ChannelCommon,
    guild_id: Snowflake,
    client: Client,
}
impl TextChannel {
    pub(crate) fn from_json(json: &Map<String, Value>, client: Client) -> TextChannel {
        TextChannel::new(
            ChannelCommon::from_json(json),
            json["guild_id"].as_str().unwrap().parse().unwrap(),
            client.clone(),
        )
    }
    pub(crate) fn new(
        channel_common: ChannelCommon,
        guild_id: Snowflake,
        client: Client,
    ) -> Self {
        Self {
            channel_common,
            guild_id,
            client: client.clone(),
        }
    }
    pub async fn send_message(&self, content: String) -> reqwest::Result<Message> {
        let response = self
            .client
            .post(format!(
                "https://discord.com/api/v10/channels/{channel_id}/messages",
                channel_id = self.channel_common.id
            ))
            .body(format!(r#"{{"content":"{content}"}}"#, content = content))
            .header("Content-Type", HeaderValue::from_str("application/json").unwrap())
            .send().await?;
        Ok(Message::from_json(
            &serde_json::from_str(&response.text().await?.as_str()).unwrap(),
            self.client.clone(),
        ))
    }
}
