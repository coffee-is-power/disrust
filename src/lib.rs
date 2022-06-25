use gateway::Gateway;
use std::time::{Duration, self, SystemTime, UNIX_EPOCH};
mod gateway;
mod guild;
pub use gateway::GatewayIntents;
pub struct Bot {
    token: &'static str,
    gateway: Gateway,
    intents: i32,
    last_heartbeat_time: u128
}
impl Bot {
    pub async fn new(token: &'static str, intents: i32) -> Self {
        Self {
            token,
            gateway: Gateway::connect().await,
            intents,
            last_heartbeat_time: 0
        }
    }
    pub async fn login(&mut self) {
        let ready_event = self.gateway.authenticate(self.token, self.intents).await;
        for _ in 0..ready_event.guilds.unwrap().len() {
            self.gateway.socket.receive().await.unwrap();
        }
        loop {
            let time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards").as_millis();
            if time - self.last_heartbeat_time > (self.gateway.heartbeat_interval/2) as u128 {
                self.last_heartbeat_time = time;
                self.gateway.send_heartbeat().await;
            }
            
        }
    }
}
