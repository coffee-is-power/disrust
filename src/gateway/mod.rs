mod events;
mod intents;
mod packet;
pub use intents::*;
use std::env;
use websockets::WebSocket;

use crate::gateway::packet::{HelloPacket, Payload};

use self::{
    events::ReadyEvent,
    packet::{IdentifyConnectionProperties, IdentityPacket},
};
const HELLO_OPCODE: i32 = 10;
const ACK_OPCODE: i32 = 11;
pub struct Gateway {
    pub socket: WebSocket,
    pub heartbeat_interval: i32,
}

impl Gateway {
    pub async fn connect() -> Self {
        let mut socket = WebSocket::connect("wss://gateway.discord.gg/?v=10&encoding=json")
            .await
            .expect("Couldn't connect to discord gateway");
        let hello = socket.receive().await.unwrap();
        let hello: Payload<HelloPacket> = serde_json::from_str(&hello.as_text().unwrap().0)
            .expect("The received packet is not valid json.");
        assert_eq!(hello.op, HELLO_OPCODE);
        Self {
            socket,
            heartbeat_interval: hello.d.unwrap().heartbeat_interval,
        }
    }
    pub async fn authenticate(&mut self, token: &'static str, intents: i32) -> ReadyEvent {
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
        self.socket
            .send_text(serde_json::to_string(&id_packet).unwrap())
            .await
            .expect("Couldn't send identity packet!");
        serde_json::from_str::<Payload<ReadyEvent>>(
            self.socket.receive().await.unwrap().as_text().unwrap().0,
        )
        .unwrap()
        .d
        .unwrap()
    }
    pub async fn send_heartbeat(&mut self) {
        let hb = serde_json::to_string(&Payload::<i32> { op: 1, d: None }).unwrap();
        self.socket
            .send_text(hb)
            .await
            .expect("Couldn't send heartbeat");
        let ack = self
            .socket
            .receive()
            .await
            .expect("Couldn't read ack packet");
        let ack: Payload<()> = serde_json::from_str(&ack.as_text().unwrap().0)
            .expect("The received packet is not valid json.");
        assert_eq!(ack.op, ACK_OPCODE);
    }
}
