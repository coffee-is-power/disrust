mod events;
use std::{time::Duration, env::consts::OS, collections::HashSet};
use const_format::formatcp;
use reqwest::Url;
use serde_json::{Value, Number, Map};
use tokio::{net::TcpStream, sync::mpsc::channel};
use tokio_tungstenite::{MaybeTlsStream, connect_async, WebSocketStream, tungstenite::Message};
use self::events::{Event, Command};
use futures_util::{StreamExt, SinkExt, TryStreamExt};

pub use events::Intent;

const API_VERSION: u32 = 10;
const DISCORD_WEBSOCKET_URL: &str = formatcp!("wss://gateway.discord.gg/?v={}&encoding=json", API_VERSION);

pub struct Gateway {
    socket: WebSocketStream<MaybeTlsStream<TcpStream>>,
    event_queue: Vec<Event>,
    command_queue: Vec<Command>,
    heartbeat_interval: u32
}
impl Gateway {
    pub async fn connect() -> Self {
        let (mut socket, _) = connect_async(Url::parse(DISCORD_WEBSOCKET_URL).unwrap()).await.unwrap();
        let hello_packet_text = socket.next().await.unwrap().unwrap().into_text().unwrap();
        let hello = serde_json::from_str::<serde_json::Map<_, _>>(&hello_packet_text).unwrap();
        
        Self {
            socket: socket,
            command_queue: vec![],
            event_queue: vec![],
            heartbeat_interval: hello["d"]["heartbeat_interval"].as_u64().unwrap() as u32
        }
    }
    
    pub async fn start_event_loop(mut self) {
        let (sender, mut receiver) = channel::<Command>(5);
        let sender2 = sender.clone();
        let heartbeat_interval = self.heartbeat_interval;
        let mut s = 0u64;
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_millis(10000));
            loop {
                interval.tick().await;
                sender2.send(Command::HeartBeat).await.unwrap();
                println!("Queueing hb");
            }
        });
        loop {
            match receiver.try_recv() {
                Ok(cmd) => {
                    self.command_queue.push(cmd);
                    println!("Received HB");
                }
                Err(e) => {
                },
            }
            while let Some(cmd) = self.command_queue.pop() {
                match cmd {
                    Command::HeartBeat => {
                        let mut packet = serde_json::Map::<_, _>::new();
                        packet.insert("op".to_owned(), Value::Number(Number::from(1u32)));
                        packet.insert("d".to_owned(), Value::Number(Number::from(s)));
                        println!("HB");
                        self.socket.send(Message::Text(serde_json::to_string(&packet).unwrap())).await.unwrap();
                    },
                    Command::Identity {intents, token} => {
                        let intents = Intent::calculate_intent_bitfield(Box::new(intents.into_iter()));
                        let mut d = serde_json::Map::<_, _>::new();
                        d.insert("token".to_string(), Value::String(token));
                        d.insert("intents".to_string(), Value::Number(Number::from(intents)));
                        d.insert("properties".to_string(), Value::Object(serde_json::Map::<_, _>::new()));
                        d["properties"].as_object_mut().unwrap().insert("os".to_string(), Value::String(OS.to_string()));
                        d["properties"].as_object_mut().unwrap().insert("browser".to_string(), Value::String("disrust".to_owned()));
                        d["properties"].as_object_mut().unwrap().insert("device".to_string(), Value::String("disrust".to_owned()));
                        let mut packet = serde_json::Map::<_, _>::new();
                        packet.insert("op".to_owned(), Value::Number(Number::from(2u32)));
                        packet.insert("d".to_owned(), serde_json::Value::Object(d));
                        self.socket.send(Message::Text(serde_json::to_string(&packet).unwrap())).await.unwrap();
                    }
                }
            }
            if let Some(packet) = self.try_read().await {
                let seq = serde_json::from_str::<Map<_, _>>(&packet).unwrap()["s"].as_u64();
                if let Some(seq) = seq{
                    s = seq;
                }
                dbg!(s);
            }
        }
    }
    async fn try_read(&mut self) -> Option<String> {
        self.socket.try_next().await.ok()??.into_text().ok()
    }
    pub fn queue_command(&mut self, cmd: Command) {
        self.command_queue.push(cmd);
    }
    pub fn authenticate(&mut self, token: &str, intents: HashSet<Intent>){
        self.queue_command(Command::Identity {
            token: token.to_string(),
            intents
        })
    }
}