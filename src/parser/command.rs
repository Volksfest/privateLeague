use serde::{Serialize, Deserialize};
use std::sync::mpsc::Sender;

#[derive(Serialize, Deserialize, Debug)]
pub struct GameArgs {
    pub first_player_win : bool,
    pub player1: (String, char),
    pub player2: (String, char),
    pub duration_min : usize,
    pub duration_sec : usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum LeagueCommand {
    AddGame(GameArgs),
    // TODO Do Statistics
    // TODO Do Debug
}

pub enum Command{
    Modify(LeagueCommand),
    Serialize,
    NewClient(Sender<String>),
    Quit,
}