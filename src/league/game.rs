use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Race {
    Terran,
    Zerg,
    Protoss,
}

impl Race {
    #[allow(dead_code)]
    pub fn char_to_race(r : char) -> Option<Race>{
        match r {
            'z' => Some(Race::Zerg),
            'Z' => Some(Race::Zerg),
            'p' => Some(Race::Protoss),
            'P' => Some(Race::Protoss),
            't' => Some(Race::Terran),
            'T' => Some(Race::Terran),
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Duration {
    pub min: usize,
    pub sec: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Color {
    pub(super) hex: String,
    pub(super) name: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Players {
    pub color: Color,
    pub name: String,
    pub race: Race,
    pub random: bool,
    pub win: bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayedDate {
    pub year: usize,
    pub month: usize,
    pub day: usize,
    pub hour: usize,
    pub minute: usize,
    pub second: usize
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Game {
    pub duration: Duration,
    pub map_name: String,
    pub observers: Vec<String>,
    pub players: Vec<Players>,
    pub date: PlayedDate
}
