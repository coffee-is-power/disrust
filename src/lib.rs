use gateway::{Gateway, Event};
pub use gateway::Intent;
mod gateway;
mod user;
pub struct Bot {
    gateway: Gateway,
    intents: Vec<Intent>,
    event_handler: Option<fn(Event)>
}
impl Bot {
    pub async fn new(intents: Vec<Intent>) -> Self {
        Self {
            gateway: Gateway::connect().await,
            intents,
            event_handler: None
        }
    }
    pub async fn set_event_handler(&mut self, e: fn(Event)) {
        self.event_handler = Some(e);
    }
    pub async fn login(mut self, token: &str) {
        self.gateway.authenticate(token, self.intents);
        self.gateway.start_event_loop(self.event_handler.unwrap_or(|_|{})).await;
    }
}
