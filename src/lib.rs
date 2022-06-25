use gateway::Gateway;
use std::{time::{SystemTime, UNIX_EPOCH}, collections::HashMap, any::Any};
mod gateway;
pub mod guild;
pub mod user;
pub use gateway::GatewayIntents;
pub use gateway::events;
pub struct Bot {
    token: &'static str,
    gateway: Gateway,
    intents: i32,
    last_heartbeat_time: u128,
    listeners: HashMap<&'static str, Vec<fn(Box<dyn Any>)>>
}
impl Bot {
    pub async fn new(token: &'static str, intents: i32) -> Self {
        Self {
            token,
            gateway: Gateway::connect().await,
            intents,
            last_heartbeat_time: 0,
            listeners: HashMap::new()
        }
    }
    pub fn on(&mut self, event_name: &'static str, listener: fn(Box<dyn Any>)) {
        if !self.listeners.contains_key(event_name) {
            self.listeners.insert(event_name, Vec::new());
        }
        self.listeners.get_mut(event_name).unwrap().push(listener);
    }
    pub fn emit<E>(&mut self, event_name: &'static str, data: Box<E>)
        where E: Clone + Any + 'static {
        if !self.listeners.contains_key(event_name) {
            self.listeners.insert(event_name, Vec::new());
        }
        for listener in &self.listeners[event_name] {
            listener(Box::new(*data.clone()));
        }
    }
    pub async fn login(&mut self) {
        let ready_event = self.gateway.authenticate(self.token, self.intents).await;
        for _ in 0..ready_event.guilds.len() {
            self.gateway.socket.receive().await.unwrap();
        }
        self.emit("ready", Box::from(ready_event));
        loop {
            let time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis();
            if time - self.last_heartbeat_time > (self.gateway.heartbeat_interval / 2) as u128 {
                self.last_heartbeat_time = time;
                self.gateway.send_heartbeat().await;
            }
        }
    }
}