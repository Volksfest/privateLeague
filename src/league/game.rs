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


#[derive(Serialize, Deserialize)]
pub struct Game {
    pub(super) first_player_won: bool,
    pub(super) races: (Race, Race),
    #[allow(dead_code)]
    pub(super) duration: Duration,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerGame {
    pub duration: Duration,
    pub map_name: String,
    pub observers: Vec<String>,
    pub players: Vec<Players>
}

impl SerGame {
    pub fn first_player_won(&self) -> bool {
        self.players[0].win
    }

    pub fn is_consistent(&self) -> bool {
        self.players.len() == 2 &&
            self.players[0].win != self.players[1].win
    }
}
