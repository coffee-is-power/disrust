use serde::{Deserialize, Serialize};

pub enum PremiumType {
    None = 0,
    NitroClassic = 1,
    Nitro = 2,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub bot: bool,
    pub system: Option<bool>,
    pub mfa_enabled: bool,
    pub accent_color: Option<u32>,
    pub locale: Option<String>,
    pub verified: bool,
    pub premium_type: Option<u8>
}