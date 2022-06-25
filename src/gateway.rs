use serde::{Deserialize, Serialize};
use websockets::WebSocket;
use std::{env, time::Duration};
const HELLO_OPCODE: i32 = 10;
const ACK_OPCODE: i32 = 11;
pub enum GatewayIntents {
    GUILDS = (1 << 0),
    GUILD_MEMBERS = (1 << 1),
    GUILD_BANS = (1 << 2),
    GUILD_EMOJIS_AND_STICKERS = (1 << 3),
    GUILD_INTEGRATIONS = (1 << 4),
    GUILD_WEBHOOKS = (1 << 5),
    GUILD_INVITES = (1 << 6),
    GUILD_VOICE_STATES = (1 << 7),
    GUILD_PRESENCES = (1 << 8),
    GUILD_MESSAGES = (1 << 9),
    GUILD_MESSAGE_REACTIONS = (1 << 10),
    GUILD_MESSAGE_TYPING = (1 << 11),
    DIRECT_MESSAGES = (1 << 12),
    DIRECT_MESSAGE_REACTIONS = (1 << 13),
    DIRECT_MESSAGE_TYPING = (1 << 14),
    MESSAGE_CONTENT = (1 << 15),
    GUILD_SCHEDULED_EVENTS = (1 << 16),
    AUTO_MODERATION_CONFIGURATION = (1 << 20),
    AUTO_MODERATION_EXECUTION = (1 << 21),
}
pub struct Gateway {
    socket: WebSocket,
    pub heartbeat_interval: i32
}
#[derive(Serialize, Deserialize, Debug)]
struct Payload<T> {
    op: i32,
    d: Option<T>,
}
#[derive(Deserialize)]
struct HelloPacket {
    pub heartbeat_interval: i32,
}
#[derive(Serialize, Deserialize, Debug)]
struct IdentifyConnectionProperties {
    os: String,
    browser: String,
    device: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct IdentityPacket {
    token: String,
    properties: IdentifyConnectionProperties,
    compress: Option<bool>,
    large_threshold: Option<i32>,
    intents: i32,
}
#[derive(Deserialize, Debug)] 
pub struct ReadyEvent {
    v: i32,
    session_id: String
}
impl Gateway {
    pub async fn connect() -> Self {
        let mut socket = WebSocket::connect("wss://gateway.discord.gg/?v=10&encoding=json").await.expect("Couldn't connect to discord gateway");
        let hello = socket.receive().await.unwrap();
        let hello: Payload<HelloPacket> =
            serde_json::from_str(&hello.as_text().unwrap().0).expect("The received packet is not valid json.");
        assert_eq!(hello.op, HELLO_OPCODE);
        Self { socket , heartbeat_interval: hello.d.unwrap().heartbeat_interval}
    }
    pub async fn authenticate(&mut self, token: &'static str, intents: i32) {
        let id_packet: Payload<IdentityPacket> = Payload {
            op: 2,
            d: Some(IdentityPacket {
                compress: Some(false),
                intents,
                large_threshold: None,
                properties: IdentifyConnectionProperties {
                    browser: "disrust".to_string(),
                    device: "disrust".to_string(),
                    os: env::consts::OS.to_string(),
                },
                token: token.to_string(),
            }),
        };
        self.socket.send_text(serde_json::to_string(&id_packet).unwrap()).await.expect("Couldn't send identity packet!");
        println!("Ready Event: {:#?}", serde_json::from_str::<Payload<ReadyEvent>>(self.socket.receive().await.unwrap().as_text().unwrap().0));
    }
    pub async fn send_heartbeat(&mut self) {
        self.socket
            .send_text(stringify!({"op": 1}).to_string()).await
            .expect("Couldn't send heartbeat");
        let ack = self
            .socket
            .receive().await
            .expect("Couldn't read ack packet");
        tokio::time::sleep(Duration::from_millis(10)).await;
        let ack: Payload<()> =
            serde_json::from_str(&ack.as_text().unwrap().0).expect("The received packet is not valid json.");
        assert_eq!(ack.op, ACK_OPCODE);
    }
}
