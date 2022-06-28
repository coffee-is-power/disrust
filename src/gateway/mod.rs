mod events;
use crate::{Bot, Guild};

pub use self::events::{Command, Event};
use const_format::formatcp;
use futures_util::{SinkExt, StreamExt, TryStreamExt};
use reqwest::Url;
use serde_json::{Map, Number, Value};
use std::{env::consts::OS, sync::Arc, time::Duration};
use tokio::{
    net::TcpStream,
    sync::{
        mpsc::{channel, Sender},
        Mutex,
    },
};
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

pub use events::Intent;

const API_VERSION: u32 = 10;
const DISCORD_WEBSOCKET_URL: &str =
    formatcp!("wss://gateway.discord.gg/?v={}&encoding=json", API_VERSION);

pub struct Gateway {
    socket: WebSocketStream<MaybeTlsStream<TcpStream>>,
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
            heartbeat_interval: hello["d"]["heartbeat_interval"].as_u64().unwrap() as u32,
        }
    }

    pub async fn start_event_loop(mut self, bot: &mut Bot, event_handler: fn(Event)) -> ! {
        let (event_sender, mut event_receiver) = channel::<Event>(5);
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
                    let packet = serde_json::from_str::<Map<_, _>>(&packet).unwrap();
                    let seq = packet["s"].as_u64();
                    if let Some(seq) = seq {
                        *s_mutex2.lock().await = seq;
                    }
                    Self::handle_packet(event_sender.clone(), packet).await;
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
            while let Ok(event) = event_receiver.try_recv() {
                event_handler(event);
            }
        }
    }
    async fn handle_packet(sender: Sender<Event>, packet: Map<String, Value>) {
        match &packet["t"] {
            Value::String(typ) => match typ.as_str() {
                "READY" => {
                    sender
                        .send(Event::Ready {
                            api_version: packet["d"]["v"].as_u64().unwrap(),
                            application_id: packet["d"]["application"]["id"]
                                .as_str()
                                .unwrap()
                                .parse()
                                .unwrap(),
                            guild_ids: packet["d"]["guilds"]
                                .as_array()
                                .unwrap()
                                .iter()
                                .map(|v| v["id"].as_str().unwrap().parse().unwrap())
                                .collect(),
                            session_id: packet["d"]["session_id"].as_str().unwrap().to_owned(),
                            bot_user: serde_json::from_value(packet["d"]["user"].clone()).unwrap(),
                        })
                        .await
                        .unwrap();
                }
                "GUILD_CREATE" => sender
                    .send(Event::GuildCreate(Guild::from_json(
                        packet["d"].as_object().unwrap(),
                    )))
                    .await
                    .unwrap(),
                _ => todo!("Event {} not implemented yet!", typ),
            },
            Value::Null => {}
            _ => println!("Received strange event type from websocket: {:#?}", packet),
        }
    }
    pub fn authenticate(&mut self, token: &str, intents: Vec<Intent>) {
        self.command_queue.push(Command::Identity {
            token: token.to_string(),
            intents,
        })
    }
}
