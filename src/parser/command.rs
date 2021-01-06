use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct GameArgs {
    pub first_player_win : bool,
    pub player1: (String, char),
    pub player2: (String, char),
    pub duration_min : usize,
    pub duration_sec : usize,

}

#[derive(Serialize, Deserialize)]
pub enum Command {
    AddGame(GameArgs),
    Serialize,
    // TODO Do Statistics
    // TODO Do Debug
}