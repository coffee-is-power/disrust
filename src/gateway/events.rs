use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ReadyEvent {
    pub v: i32,
    pub session_id: String,
    pub(crate) guilds: Option<Vec<crate::guild::UnavailableGuild>>,
}
