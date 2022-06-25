use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ReadyEvent {
    pub v: i32,
    pub session_id: String,
    pub guilds: Vec<crate::guild::UnavailableGuild>,
}
impl Clone for ReadyEvent {
    fn clone_from(&mut self, source: &Self)
    {
        self.v = self.v.clone();
        self.session_id = self.session_id.clone();
        self.guilds = self.guilds.clone();
    }

    fn clone(&self) -> Self {
        Self { v: self.v.clone(), session_id: self.session_id.clone(), guilds: self.guilds.clone() }
    }
}