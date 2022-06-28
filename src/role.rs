use serde_json::{Map, Value};

use crate::{permissions::Permission, snowflake::Snowflake};

#[derive(Debug, Clone)]
pub struct Color(u8, u8, u8);
impl Color {
    pub fn from_hex(hex: u32) -> Self {
        Self(
            (hex & 0xFF) as u8,
            ((hex & 0xFF00) >> 8) as u8,
            ((hex & 0xFF0000) >> 16) as u8,
        )
    }
}

macro_rules! getter {
    ($field:ident -> $typ:ty) => {
        pub fn $field(&self) -> $typ {
            self.$field.clone()
        }
    };
    (&$field:ident -> $typ:ty) => {
        pub fn $field<'a>(&'a self) -> &'a $typ {
            &self.$field
        }
    };
}
#[derive(Debug, Clone)]
pub struct Role {
    id: Snowflake,
    name: String,
    color: Color,
    /// true if the role appears as a category in the members list
    hoist: bool,
    icon_url: Option<String>,
    /// This is the place/level of importance of the role
    ///
    /// Example: If we had 3 roles, Owner, Moderator and Member, Owner would be 2, Moderator would be 1 and Member would be 0
    position: u64,
    mentionable: bool,
    permissions: Vec<Permission>,
}
impl Role {
    pub(crate) fn from_json(json: &Map<String, Value>) -> Self {
        Self {
            id: json["id"].as_str().unwrap().parse().unwrap(),
            color: Color::from_hex(json["color"].as_u64().unwrap() as u32),
            hoist: json["hoist"].as_bool().unwrap(),
            icon_url: if let Some(icon_hash) = json["icon"].as_str() {
                Some(format!(
                    "https://cdn.discordapp.com/role-icons/{role_id}/{role_icon}.png",
                    role_id = json["id"].as_str().unwrap(),
                    role_icon = icon_hash
                ))
            } else {
                None
            },
            name: json["name"].as_str().unwrap().to_string(),
            position: json["position"].as_u64().unwrap(),
            mentionable: json["mentionable"].as_bool().unwrap(),
            permissions: Permission::get_permissions(
                json["permissions"].as_str().unwrap().parse().unwrap(),
            ),
        }
    }
    getter!(&id -> Snowflake);
    getter!(&name -> String);
    getter!(&color -> Color);
    getter!(hoist -> bool);
    getter!(&icon_url -> Option<String>);
    getter!(position -> u64);
    getter!(mentionable -> bool);
    getter!(&permissions -> Vec<Permission>);
}
