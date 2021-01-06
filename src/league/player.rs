use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub(super) name: String,
}