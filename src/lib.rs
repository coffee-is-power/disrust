use std::{thread, time::Duration};

use gateway::Gateway;

mod gateway;
pub use gateway::GatewayIntents;
pub struct Bot {
    token: &'static str,
    gateway: Gateway,
    intents: i32,
}
impl Bot {
    pub async fn new(token: &'static str, intents: i32) -> Self {
        Self {
            token,
            gateway: Gateway::connect().await,
            intents,
        }
    }
    pub async fn login(&mut self) {
        self.gateway.authenticate(self.token, self.intents).await;
        loop {
            tokio::time::sleep(Duration::from_millis(self.gateway.heartbeat_interval as u64)).await;
            self.gateway.send_heartbeat().await;
        }
    }
}
