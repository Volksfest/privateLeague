use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub enum Race {
    Terran,
    Zerg,
    Protoss,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct Duration {
    pub(super) min: usize,
    pub(super) sec: usize,
}

#[derive(Serialize, Deserialize)]
pub struct Game {
    pub(super) first_player_won: bool,
    pub(super) races: (Race, Race),
    #[allow(dead_code)]
    pub(super) duration: Duration,
}