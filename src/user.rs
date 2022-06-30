use serde::Deserialize;

use crate::getter;

#[derive(Deserialize, Debug, Clone)]
pub struct User {
    pub(crate) id: String,
    pub(crate) username: String,
    pub(crate) discriminator: String,
    pub(crate) avatar: String,
    pub(crate) bot: Option<bool>,
    pub(crate) banner: Option<String>,
    pub(crate) accent_color: Option<u32>,
    pub(crate) locale: Option<String>,
    pub(crate) verified: Option<bool>,
    pub(crate) email: Option<String>,
    pub(crate) flags: Option<u32>,
    pub(crate) premium_type: Option<PremiumType>,
}
impl User {
    getter!(id -> String);
    getter!(username -> String);
    getter!(discriminator -> String);
    getter!(avatar -> String);
    getter!(is_bot -> unwrap_or bot false => bool);
    getter!(banner -> Option<String>);
    getter!(accent_color -> Option<u32>);
    getter!(locale -> Option<String>);
    getter!(is_verified -> unwrap_or verified false => bool);
    getter!(email -> Option<String>);
    getter!(flags -> Option<u32>);
    getter!(premium_type -> Option<PremiumType>);
}
#[derive(Clone, Copy, Deserialize, Debug)]
pub enum PremiumType {
    None,
    NitroClassic,
    Nitro,
}
