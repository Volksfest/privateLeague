use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Race {
    Terran,
    Zerg,
    Protoss,
}

impl Race {
    pub fn char_to_race(r : char) -> Option<Race>{
        match r {
            'z' => Some(Race::Zerg),
            'p' => Some(Race::Protoss),
            't' => Some(Race::Terran),
            _ => None
        }
    }

    pub fn race_to_char(&self) -> char {
        match self {
            Race::Terran => 'T',
            Race::Protoss => 'P',
            Race::Zerg => 'Z'
        }
    }
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