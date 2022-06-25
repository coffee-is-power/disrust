use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct UnavailableGuild {
    id: String,
}
