use std::{collections::HashSet, hash::Hash};

use crate::user::User;

pub enum Event {
    Ready {
        api_version: u128,
        session_id: String,
        application_id: u128,
        guild_ids: Vec<u128>,
        bot_user: User
    },
    HeartBeatAcknowledge,
    InvalidSession,
}
#[derive(Debug)]
pub enum Command {
    HeartBeat,
    Identity {
        token: String,
        intents: HashSet<Intent>
    },
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
    pub fn calculate_intent_bitfield(intents: Box<dyn Iterator<Item=Self>>) -> u32 {
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