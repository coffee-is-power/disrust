use gateway::Gateway;
use std::time::Duration;
mod gateway;
mod guild;
pub use gateway::GatewayIntents;
pub struct Bot {
    token: &'static str,
    gateway: Gateway,
    intents: i32
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
        let ready_event = self.gateway.authenticate(self.token, self.intents).await;
        for _ in 0..ready_event.guilds.unwrap().len() {
            self.gateway.socket.receive().await.unwrap();
        }
        loop {
            tokio::time::sleep(Duration::from_millis(
                (self.gateway.heartbeat_interval/4) as u64,
            ))
            .await;
            self.gateway.send_heartbeat().await;
        }
    }
}
