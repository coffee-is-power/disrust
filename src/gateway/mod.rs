mod events;
use self::events::{Command, Event};
use const_format::formatcp;
use futures_util::{SinkExt, StreamExt, TryStreamExt};
use reqwest::Url;
use serde_json::{Map, Number, Value};
use std::{env::consts::OS, sync::Arc, time::Duration};
use tokio::{
    net::TcpStream,
    sync::{mpsc::channel, Mutex},
};
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

pub use events::Intent;

const API_VERSION: u32 = 10;
const DISCORD_WEBSOCKET_URL: &str =
    formatcp!("wss://gateway.discord.gg/?v={}&encoding=json", API_VERSION);

pub struct Gateway {
    socket: WebSocketStream<MaybeTlsStream<TcpStream>>,
    event_queue: Vec<Event>,
    command_queue: Vec<Command>,
    heartbeat_interval: u32,
}
impl Gateway {
    pub async fn connect() -> Self {
        let (mut socket, _) = connect_async(Url::parse(DISCORD_WEBSOCKET_URL).unwrap())
            .await
            .unwrap();
        let hello_packet_text = socket.next().await.unwrap().unwrap().into_text().unwrap();
        let hello = serde_json::from_str::<serde_json::Map<_, _>>(&hello_packet_text).unwrap();

        Self {
            socket: socket,
            command_queue: vec![],
            event_queue: vec![],
            heartbeat_interval: hello["d"]["heartbeat_interval"].as_u64().unwrap() as u32,
        }
    }

    pub async fn start_event_loop(mut self) {
        let (sender, mut receiver) = channel::<Command>(5);
        let sender2 = sender.clone();
        let heartbeat_interval = self.heartbeat_interval;
        let s_mutex = Arc::from(Mutex::new(0));
        tokio::spawn(async move {
            let mut interval =
                tokio::time::interval(Duration::from_millis(heartbeat_interval.into()));
            loop {
                interval.tick().await;
                sender2.send(Command::HeartBeat).await.unwrap();
            }
        });

        let s_mutex2 = s_mutex.clone();
        let (mut writer, mut reader) = self.socket.split();

        tokio::spawn(async move {
            loop {
                let next = reader
                    .try_next()
                    .await
                    .ok()
                    .flatten()
                    .map(|x| x.into_text().ok())
                    .flatten();
                if let Some(packet) = next {
                    let seq = serde_json::from_str::<Map<_, _>>(&packet).unwrap()["s"].as_u64();
                    if let Some(seq) = seq {
                        *s_mutex2.lock().await = seq;
                    }
                }
            }
        });
        loop {
            if let Ok(cmd) = receiver.try_recv() {
                self.command_queue.push(cmd);
            }
            while let Some(cmd) = self.command_queue.pop() {
                match cmd {
                    Command::HeartBeat => {
                        let mut packet = serde_json::Map::<_, _>::new();
                        packet.insert("op".to_owned(), Value::Number(Number::from(1u32)));
                        let s = s_mutex.lock().await;
                        println!("{}", *s);
                        packet.insert("d".to_owned(), Value::Number(Number::from(*s)));
                        writer
                            .send(Message::Text(serde_json::to_string(&packet).unwrap()))
                            .await
                            .unwrap();
                    }
                    Command::Identity { intents, token } => {
                        let intents =
                            Intent::calculate_intent_bitfield(Box::new(intents.into_iter()));
                        let mut d = serde_json::Map::<_, _>::new();
                        d.insert("token".to_string(), Value::String(token));
                        d.insert("intents".to_string(), Value::Number(Number::from(intents)));
                        d.insert(
                            "properties".to_string(),
                            Value::Object(serde_json::Map::<_, _>::new()),
                        );
                        d["properties"]
                            .as_object_mut()
                            .unwrap()
                            .insert("os".to_string(), Value::String(OS.to_string()));
                        d["properties"]
                            .as_object_mut()
                            .unwrap()
                            .insert("browser".to_string(), Value::String("disrust".to_owned()));
                        d["properties"]
                            .as_object_mut()
                            .unwrap()
                            .insert("device".to_string(), Value::String("disrust".to_owned()));
                        let mut packet = serde_json::Map::<_, _>::new();
                        packet.insert("op".to_owned(), Value::Number(Number::from(2u32)));
                        packet.insert("d".to_owned(), serde_json::Value::Object(d));
                        writer
                            .send(Message::Text(serde_json::to_string(&packet).unwrap()))
                            .await
                            .unwrap();
                    }
                }
            }
        }
    }
    pub fn authenticate(&mut self, token: &str, intents: Vec<Intent>) {
        self.command_queue.push(Command::Identity {
            token: token.to_string(),
            intents,
        })
    }
}
