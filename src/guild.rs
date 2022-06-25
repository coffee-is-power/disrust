use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct UnavailableGuild {
    id: String,
}

impl Clone for UnavailableGuild {
    fn clone_from(&mut self, source: &Self)
    {
        self.id = source.id.clone();
    }

    fn clone(&self) -> Self {
        Self { id: self.id.clone() }
    }
}