use gateway::Gateway;
pub mod guild;
pub mod snowflake;
pub mod permissions;
pub mod role;
pub mod emoji;
pub use gateway::Event;
pub use gateway::Intent;
pub use guild::Guild;
mod gateway;
pub mod user;
pub struct Bot {
    gateway: Gateway,
    intents: Vec<Intent>,
    event_handler: Option<fn(Event)>,
}
impl Bot {
    pub async fn new(intents: Vec<Intent>) -> Self {
        Self {
            gateway: Gateway::connect().await,
            intents,
            event_handler: None,
        }
    }
    pub fn set_event_handler(&mut self, e: fn(Event)) {
        self.event_handler = Some(e);
    }
    pub async fn login(mut self, token: &str) {
        self.gateway.authenticate(token, self.intents);
        self.gateway
            .start_event_loop(self.event_handler.unwrap_or(|_| {}))
            .await;
    }
}
