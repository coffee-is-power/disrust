use std::hash::Hash;

use crate::{snowflake::Snowflake, user::User, Guild, channel::message::Message};
#[derive(Debug)]
pub enum Event {
    /// This event is sent when the bot successfully logs in
    /// It contains some information about the bot, and all the guilds (IDs) the bot is in
    Ready {
        api_version: u64,
        session_id: String,
        application_id: u64,
        guild_ids: Vec<Snowflake>,
        bot_user: User,
    },
    /// This event is sent when the bot joins a server
    ///
    /// It will also be sent after Ready to complete the Guild objects sent in the ready event
    ///
    /// See also: https://discord.com/developers/docs/topics/gateway#guilds
    GuildCreate(Guild),
    MessageCreate(Message),
    HeartBeatAcknowledge,
    InvalidSession,
}
#[derive(Debug)]
pub enum Command {
    HeartBeat,
    Identity { token: String, intents: Vec<Intent> },
}
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[allow(unused)]
pub enum Intent {
    Guild = (1 << 0),
    GuildMembers = (1 << 1),
    GuildBans = (1 << 2),
    GuildEmojisAndStickers = (1 << 3),
    GuildIntegrations = (1 << 4),
    GuildWebhooks = (1 << 5),
    GuildInvited = (1 << 6),
    GuildVoiceStates = (1 << 7),
    GuildPresences = (1 << 8),
    GuildMessages = (1 << 9),
    GuildMessageReactions = (1 << 10),
    GuildMessageTyping = (1 << 11),
    DirectMessages = (1 << 12),
    DirectMessageReactions = (1 << 13),
    DirectMessageTyping = (1 << 14),
    MessageContent = (1 << 15),
    GuildScheduledEvents = (1 << 16),
    AutoModerationConfiguration = (1 << 20),
    AutoModerationExecution = (1 << 21),
}
impl Hash for Intent {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u32(self.get_bit());
    }
}
impl Intent {
    pub fn calculate_intent_bitfield(intents: Box<dyn Iterator<Item = Self>>) -> u32 {
        let mut result = 0u32;
        for intent in intents {
            result |= intent.get_bit();
        }
        result
    }
    pub fn get_bit(self) -> u32 {
        self as u32
    }
}
