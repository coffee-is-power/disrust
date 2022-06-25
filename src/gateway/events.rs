use serde::Deserialize;

use crate::{user::User, guild::UnavailableGuild};

#[derive(Deserialize, Debug, Clone)]
pub struct ReadyEvent {
    pub v: i32,
    pub session_id: String,
    pub guilds: Vec<UnavailableGuild>,
    pub user: User
}