use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub(crate) struct UnavailableGuild {
    id: String,
}
