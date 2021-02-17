use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub(super) name: String,
}

impl Player {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}