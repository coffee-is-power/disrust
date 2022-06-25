use serde::{Deserialize, Serialize};

pub enum PremiumType {
    None = 0,
    NitroClassic = 1,
    Nitro = 2,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub discriminator: String,
    pub bot: bool,
    pub system: bool,
    pub mfa_enabled: bool,
    pub accent_color: u32,
    pub locale: String,
    pub verified: bool,
    pub premium_type: u8
}