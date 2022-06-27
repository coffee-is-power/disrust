pub struct User {
    pub(crate) id: u128,
    pub(crate) username: String,
    pub(crate) discriminator: u16,
    pub(crate) avatar_hash: String,
    pub(crate) bot: bool,
    pub(crate) system: bool,
    pub(crate) has_2fa: bool,
    pub(crate) banner_hash: Option<String>,
    pub(crate) accent_color: Option<u32>,
    pub(crate) locale: Option<String>,
    pub(crate) verified: bool,
    pub(crate) email: Option<String>,
    pub(crate) flags: Option<u32>,
    pub(crate) premium_type: Option<PremiumType>,
}
macro_rules! getter {
    ($field:ident -> $typ:ty) => {
        pub fn $field(&self) -> $typ { self.$field.clone() }
    };
    ($getter_name:ident -> $field:ident -> $typ:ty) => {
        pub fn $getter_name(&self) -> $typ { self.$field.clone() }
    };
}
impl User {
    getter!(id -> u128);
    getter!(username -> String);
    getter!(discriminator -> u16);
    getter!(avatar_hash -> String);
    getter!(is_bot -> bot -> bool);
    getter!(is_system_bot -> system -> bool);
    getter!(has_2fa -> bool);
    getter!(banner_hash -> Option<String>);
    getter!(accent_color -> Option<u32>);
    getter!(locale -> Option<String>);
    getter!(is_verified -> verified -> bool);
    getter!(email -> Option<String>);
    getter!(flags -> Option<u32>);
    getter!(premium_type -> Option<PremiumType>);
}
#[derive(Clone, Copy)]
pub enum PremiumType {
    None,
    NitroClassic,
    Nitro
}