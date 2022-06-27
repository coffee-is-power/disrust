
use gateway::Gateway;
pub use gateway::Intent;
mod gateway;
mod user;
pub struct Bot {
    gateway: Gateway,
    intents: Vec<Intent>,
}
impl Bot {
    pub async fn new(intents: Vec<Intent>) -> Self {
        Self {
            gateway: Gateway::connect().await,
            intents,
        }
    }
    pub async fn login(mut self, token: &str) {
        self.gateway.authenticate(token, self.intents);
        self.gateway.start_event_loop().await;
    }
}
