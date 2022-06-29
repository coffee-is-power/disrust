use reqwest::blocking::Client;
use serde_json::{Map, Value};

use crate::{getter, role::Role, snowflake::Snowflake, user::User};

use super::TextChannel;

pub struct Message<'a> {
    id: Snowflake,
    content: String,
    author: User,
    channel_id: Snowflake,
    mentions: Vec<User>,
    mentions_roles: Vec<Role>,
    client: &'a Client,
}
impl<'b> Message<'b> {
    getter!(id -> Snowflake);
    getter!(channel_id -> Snowflake);
    getter!(content -> String);
    getter!(&mentions_roles -> Vec<Role>);
    getter!(&mentions -> Vec<User>);
    getter!(&author -> User);
    pub fn channel(&self) -> TextChannel {
        TextChannel::from_json(
            &serde_json::from_str::<Map<_, _>>(
                &self
                    .client
                    .get(format!("https://discord.com/api/v10/channels/{}", self.id()))
                    .send()
                    .unwrap()
                    .text()
                    .unwrap(),
            )
            .unwrap(),
            self.client,
        )
    }
    pub(crate) fn from_json(json: Map<String, Value>, client: &'b Client) -> Self {
        Self {
            id: json["id"].as_str().unwrap().parse().unwrap(),
            channel_id: json["channel_id"].as_str().unwrap().parse().unwrap(),
            author: serde_json::from_value(json["author"].clone()).unwrap(),
            content: json["content"].as_str().unwrap().to_string(),
            mentions: json["mentions"]
                .as_array()
                .unwrap()
                .iter()
                .map(|u| serde_json::from_value(u.clone()).unwrap())
                .collect(),
            mentions_roles: json["mentions_roles"]
                .as_array()
                .unwrap()
                .iter()
                .map(|r| Role::from_json(r.as_object().unwrap()))
                .collect(),
            client,
        }
    }
}
