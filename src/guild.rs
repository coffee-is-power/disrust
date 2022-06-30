use std::time::Duration;

use crate::{
    channel::{Channel, TextChannel},
    emoji::Emoji,
    getter,
    role::Role,
    snowflake::Snowflake,
    Bot,
};
use reqwest::Client;
use serde_json::{Map, Value};
use strum::FromRepr;

#[derive(Debug, Clone)]
pub struct Guild {
    id: Snowflake,
    name: String,
    icon_url: String,
    splash_url: Option<String>,
    discovery_splash_url: Option<String>,
    owner_user_id: String,
    afk_channel_id: Option<Snowflake>,
    afk_timeout: Duration,
    widget_enabled: bool,
    widget_channel_id: Option<Snowflake>,
    verification_level: VerificationLevel,
    default_message_notifications: MessageNotificationLevel,
    explicit_content_filter: ContentFilterLevel,
    roles: Vec<Role>,
    emojis: Vec<Emoji>,
    features: Vec<String>,
    mfa_level: bool,
    client: Client,
}

impl Guild {
    getter!(id -> Snowflake);
    getter!(name -> String);
    getter!(icon_url -> String);
    getter!(splash_url -> Option<String>);
    getter!(discovery_splash_url -> Option<String>);
    getter!(owner_user_id -> String);
    getter!(&afk_channel_id -> Option<Snowflake>);
    getter!(afk_timeout -> Duration);
    getter!(widget_enabled -> bool);
    getter!(widget_channel_id -> Option<Snowflake>);
    getter!(verification_level -> VerificationLevel);
    getter!(default_message_notifications -> MessageNotificationLevel);
    getter!(explicit_content_filter -> ContentFilterLevel);
    getter!(&roles -> Vec<Role>);
    getter!(&emojis -> Vec<Emoji>);
    getter!(&features -> Vec<String>);
    getter!(mfa_level -> bool);
    pub async fn channels<'a>(&'a self) -> Vec<Channel> {
        let res = self
            .client
            .get("https://discord.com/api/v10/guilds/{guild.id}/channels")
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        let array = serde_json::from_str::<Vec<Map<String, Value>>>(&res).unwrap();
        array
            .iter()
            .map(|x| Channel::from_json(x, self.client.clone()))
            .collect()
    }
    pub(crate) fn from_json(json: &Map<String, Value>, client: Client) -> Self {
        Self {
            afk_channel_id: if let Some(afk_channel_id) = json["afk_channel_id"].as_str() {
                Some(afk_channel_id.parse().unwrap())
            } else {
                None
            },
            id: json["id"].as_str().unwrap().parse().unwrap(),
            name: json["name"].as_str().unwrap().to_string(),
            icon_url: format!(
                "https://cdn.discordapp.com/icons/{guild_id}/{guild_icon}.png",
                guild_id = json["id"].as_str().unwrap(),
                guild_icon = json["icon"].as_str().unwrap()
            ),
            splash_url: if let Some(splash) = json["splash"].as_str() {
                Some(format!(
                    "https://cdn.discordapp.com/splashes/{guild_id}/{splash}.png",
                    guild_id = json["id"].as_str().unwrap(),
                    splash = splash
                ))
            } else {
                None
            },
            discovery_splash_url: if let Some(splash) = json["discovery_splash"].as_str() {
                Some(format!(
                    "https://cdn.discordapp.com/discovery-splashes/{guild_id}/{splash}.png",
                    guild_id = json["id"].as_str().unwrap(),
                    splash = splash
                ))
            } else {
                None
            },
            owner_user_id: json["owner_id"].as_str().unwrap().parse().unwrap(),
            afk_timeout: Duration::from_millis(json["afk_timeout"].as_u64().unwrap()),
            widget_enabled: if let Some(widget_enabled) = json.get("widget_enabled") {
                widget_enabled.as_bool().unwrap()
            } else {
                false
            },
            widget_channel_id: if let Some(widget_channel_id) = json.get("widget_channel_id") {
                widget_channel_id.as_str().unwrap().parse().ok()
            } else {
                None
            },
            verification_level: VerificationLevel::from_repr(
                json["verification_level"].as_u64().unwrap(),
            )
            .unwrap(),
            default_message_notifications: MessageNotificationLevel::from_repr(
                json["default_message_notifications"].as_u64().unwrap(),
            )
            .unwrap(),
            explicit_content_filter: ContentFilterLevel::from_repr(
                json["explicit_content_filter"].as_u64().unwrap(),
            )
            .unwrap(),
            roles: json["roles"]
                .as_array()
                .unwrap()
                .iter()
                .map(|r| Role::from_json(&r.as_object().unwrap()))
                .collect(),
            emojis: json["emojis"]
                .as_array()
                .unwrap()
                .iter()
                .map(|e| Emoji::from_json(e.as_object().unwrap()))
                .collect(),
            features: json["features"]
                .as_array()
                .unwrap()
                .iter()
                .map(|f| f.as_str().unwrap().to_string())
                .collect(),
            mfa_level: if json["mfa_level"].as_u64().unwrap() > 0 {
                true
            } else {
                false
            },
            client: client.clone(),
        }
    }
}

#[derive(Debug, FromRepr, Clone, Copy)]
#[repr(u64)]
pub enum VerificationLevel {
    None,
    Low,
    Medium,
    High,
    VeryHigh,
}
#[derive(Debug, FromRepr, Clone, Copy)]
#[repr(u64)]
pub enum MessageNotificationLevel {
    AllMessages,
    OnlyMentions,
}
#[derive(Debug, FromRepr, Clone, Copy)]
#[repr(u64)]
pub enum ContentFilterLevel {
    None,
    MembersWithoutRoles,
    AllMembers,
}
