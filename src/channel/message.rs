use reqwest::Client;
use serde_json::{Map, Value};

use crate::{getter, role::Role, snowflake::Snowflake, user::User};

use super::TextChannel;
#[derive(Debug)]
pub struct Message {
    id: Snowflake,
    content: String,
    author: User,
    channel_id: Snowflake,
    mentions: Vec<User>,
    mention_roles: Vec<Role>,
    client: Client,
}
impl Message {
    getter!(id -> Snowflake);
    getter!(channel_id -> Snowflake);
    getter!(content -> String);
    getter!(&mention_roles -> Vec<Role>);
    getter!(&mentions -> Vec<User>);
    getter!(&author -> User);
    pub async fn delete(self) {
        let _ = self
            .client
            .delete(format!(
                "https://discord.com/api/v10/channels/{}/messages/{}",
                self.channel_id(),
                self.id()
            ))
            .send()
            .await;
    }
    pub async fn channel(&self) -> TextChannel {
        TextChannel::from_json(
            &serde_json::from_str::<Map<_, _>>(
                &self
                    .client
                    .get(format!(
                        "https://discord.com/api/v10/channels/{}",
                        self.channel_id()
                    ))
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap(),
            )
            .unwrap(),
            self.client.clone(),
        )
    }
    pub(crate) fn from_json(json: &Map<String, Value>, client: Client) -> Self {
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
            mention_roles: json["mention_roles"]
                .as_array()
                .unwrap()
                .iter()
                .map(|r| Role::from_json(r.as_object().unwrap()))
                .collect(),
            client,
        }
    }
}
