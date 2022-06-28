use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct User {
    pub(crate) id: String,
    pub(crate) username: String,
    pub(crate) discriminator: String,
    pub(crate) avatar: String,
    pub(crate) bot: bool,
    pub(crate) banner: Option<String>,
    pub(crate) accent_color: Option<u32>,
    pub(crate) locale: Option<String>,
    pub(crate) verified: bool,
    pub(crate) email: Option<String>,
    pub(crate) flags: Option<u32>,
    pub(crate) premium_type: Option<PremiumType>,
}
macro_rules! getter {
    ($field:ident -> $typ:ty) => {
        pub fn $field(&self) -> $typ {
            self.$field.clone()
        }
    };
    ($getter_name:ident -> $field:ident -> $typ:ty) => {
        pub fn $getter_name(&self) -> $typ {
            self.$field.clone()
        }
    };
}
impl User {
    getter!(id -> String);
    getter!(username -> String);
    getter!(discriminator -> String);
    getter!(avatar -> String);
    getter!(is_bot -> bot -> bool);
    getter!(banner -> Option<String>);
    getter!(accent_color -> Option<u32>);
    getter!(locale -> Option<String>);
    getter!(is_verified -> verified -> bool);
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
