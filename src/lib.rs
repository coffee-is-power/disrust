use gateway::Gateway;
pub mod emoji;
pub mod guild;
pub mod permissions;
pub mod role;
pub mod snowflake;
pub use gateway::Event;
pub use gateway::Intent;
pub use guild::Guild;
mod gateway;
pub mod user;
pub struct Bot {}
impl Bot {
    /// Creates a new bot
    pub fn new() -> Self {
        Self {}
    }
    /// Logs into a bot account using a token and starts an event loop to handle all events coming from the discord gateway
    pub async fn login(&mut self, token: &str, intents: Vec<Intent>, e: fn(Event)) -> ! {
        let mut gateway = Gateway::connect().await;
        gateway.authenticate(token, intents);
        gateway.start_event_loop(self, e).await
    }
}
