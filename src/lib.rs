pub mod channel;
pub mod emoji;
mod getter;
pub mod guild;
pub mod permissions;
pub mod role;
pub mod snowflake;
pub use gateway::Event;
use gateway::Gateway;
pub use gateway::Intent;
pub use guild::Guild;
use reqwest::{
    blocking::{Client, ClientBuilder},
    header::{HeaderMap, HeaderValue},
};
use snowflake::Snowflake;
mod gateway;
pub mod user;
pub struct Bot {
    pub(crate) partial_guilds: Vec<Snowflake>,
    guilds: Vec<Guild>,
    pub(crate) client: Client,
    token: String,
}
impl Bot {
    /// Creates a new bot
    pub fn new(token: &str) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bot {}", token)).unwrap(),
        );
        Self {
            partial_guilds: vec![],
            guilds: vec![],
            client: ClientBuilder::default()
                .default_headers(headers)
                .build()
                .unwrap(),
            token: token.to_string(),
        }
    }
    /// Logs into a bot account using a token and starts an event loop to handle all events coming from the discord gateway
    pub async fn login(&mut self, intents: Vec<Intent>, eh: fn(Event)) -> ! {
        let mut gateway = Gateway::connect().await;
        gateway.authenticate(&self.token, intents);
        gateway.start_event_loop(self, eh).await
    }
}
