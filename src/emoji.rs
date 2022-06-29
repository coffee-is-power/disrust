use serde_json::{Map, Value};

use crate::{getter, role::Role, snowflake::Snowflake, user::User};

#[derive(Debug, Clone)]
pub struct Emoji {
    /// If None, probably it's a builtin emoji, and name will be a unicode emoji
    id: Option<Snowflake>,
    name: Option<String>,
    roles: Vec<Role>,
    creator: Option<User>,
    animated: bool,
    available: bool,
}
impl Emoji {
    getter!(id -> Option<Snowflake>);
    getter!(name -> Option<String>);
    getter!(&roles -> Vec<Role>);
    getter!(creator -> Option<User>);
    getter!(animated -> bool);
    getter!(available -> bool);
    pub(crate) fn from_json(json: &Map<String, Value>) -> Self {
        Self {
            id: json["id"].as_str().map(|s| s.parse::<Snowflake>().unwrap()),
            animated: json
                .get("animated")
                .unwrap_or(&Value::Bool(false))
                .as_bool()
                .unwrap(),
            available: json
                .get("available")
                .unwrap_or(&Value::Bool(false))
                .as_bool()
                .unwrap(),
            creator: if let Some(user) = json.get("user") {
                Some(serde_json::from_value(user.clone()).unwrap())
            } else {
                None
            },
            name: if let Some(name) = json["name"].as_str() {
                Some(name.to_string())
            } else {
                None
            },
            roles: json["roles"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| Role::from_json(v.as_object().unwrap()))
                .collect(),
        }
    }
}
