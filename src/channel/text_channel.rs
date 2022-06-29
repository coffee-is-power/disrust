use reqwest::{blocking::Client, header::AUTHORIZATION, Method};
use serde_json::{Map, Value};

use super::{message::Message, ChannelCommon};
use crate::snowflake::Snowflake;

pub struct TextChannel<'a> {
    channel_common: ChannelCommon,
    guild_id: Snowflake,
    client: &'a Client,
}
impl<'b> TextChannel<'b> {
    pub(crate) fn from_json(json: &Map<String, Value>, client: &'b Client) -> TextChannel<'b> {
        TextChannel::new(
            ChannelCommon::from_json(json),
            json["guild_id"].as_str().unwrap().parse().unwrap(),
            client,
        )
    }
    pub(crate) fn new(
        channel_common: ChannelCommon,
        guild_id: Snowflake,
        client: &'b Client,
    ) -> Self {
        Self {
            channel_common,
            guild_id,
            client,
        }
    }
    pub fn send_message(&self, content: String) -> reqwest::Result<Message> {
        let response = self
            .client
            .post(format!(
                "https://discord.com/api/v10/channels/{channel_id}/messages",
                channel_id = self.channel_common.id
            ))
            .body(format!(r#"{{"content":"{content}"}}"#, content = content))
            .send()?;
        Ok(Message::from_json(
            serde_json::from_str(response.text()?.as_str()).unwrap(),
            self.client,
        ))
    }
}
