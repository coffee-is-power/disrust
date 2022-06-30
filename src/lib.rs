pub mod channel;
pub mod emoji;
mod getter;
pub mod guild;
pub mod permissions;
pub mod role;
pub mod snowflake;
use futures_util::Future;
pub use gateway::Event;
use gateway::Gateway;
pub use gateway::Intent;
pub use guild::Guild;
use reqwest::{
    Client, ClientBuilder,
    header::{HeaderMap, HeaderValue},
};
use snowflake::Snowflake;
mod gateway;
pub mod user;
pub struct Bot {
    pub(crate) partial_guilds: Vec<Snowflake>,
    guilds: Vec<Guild>,
    token: String,
}
impl Bot {
    /// Creates a new bot
    pub fn new(token: &str) -> Self {
        
        Self {
            partial_guilds: vec![],
            guilds: vec![],
            token: token.to_string(),
        }
    }
    
    /// Logs into a bot account using a token and starts an event loop to handle all events coming from the discord gateway
    pub async fn login<F: Future>(&mut self, intents: Vec<Intent>, eh: fn(Event) -> F) -> ! {
        let mut gateway = Gateway::connect().await;
        gateway.authenticate(&self.token, intents);
        gateway.start_event_loop(self, eh, create_client(&self.token)).await
    }
}
fn create_client(token: &str) -> Client {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bot {}", token)).unwrap(),
    );
    ClientBuilder::default()
            .default_headers(headers)
            .build()
            .unwrap()
}