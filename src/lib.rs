pub mod channel;
pub mod emoji;
mod getter;
pub mod guild;
pub mod permissions;
pub mod role;
pub mod snowflake;
use channel::Channel;
use futures_util::Future;
pub use gateway::Event;
use gateway::Gateway;
pub use gateway::Intent;
pub use guild::Guild;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, ClientBuilder,
};
use serde_json::{Map, Value};
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
        gateway
            .start_event_loop(self, eh, create_client(&self.token))
            .await
    }
    pub async fn fetch_channel(&self, id: Snowflake) -> reqwest::Result<Channel> {
        let client = create_client(&self.token);
        Self::fetch_channel_with_client(client, id).await
    }
    pub async fn fetch_channel_with_client(
        client: Client,
        id: Snowflake,
    ) -> reqwest::Result<Channel> {
        let channel_obj = serde_json::from_str::<Map<String, Value>>(
            &client
                .get(format!("https://discord.com/api/v10/channels/{}", id))
                .send()
                .await?
                .text()
                .await?,
        )
        .unwrap();
        Ok(Channel::from_json(&channel_obj, client))
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

