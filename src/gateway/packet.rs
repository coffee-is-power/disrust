use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload<T> {
    pub op: i32,
    pub d: Option<T>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct IdentifyConnectionProperties {
    pub os: String,
    pub browser: String,
    pub device: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IdentityPacket {
    pub token: String,
    pub properties: IdentifyConnectionProperties,
    pub compress: Option<bool>,
    pub large_threshold: Option<i32>,
    pub intents: i32,
}

#[derive(Deserialize)]
pub struct HelloPacket {
    pub heartbeat_interval: i32,
}
