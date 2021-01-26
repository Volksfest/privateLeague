use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AddGameArgs {
    pub first_player_win : bool,
    pub player1: (String, char),
    pub player2: (String, char),
    pub duration_min : usize,
    pub duration_sec : usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveGameArgs{
    pub player1: String,
    pub player2: String
}

#[derive(Serialize, Deserialize, Debug)]
pub enum LeagueCommand {
    AddGame(AddGameArgs),
    RemoveGames(RemoveGameArgs)
    // TODO Do Statistics
    // TODO Do Debug
}

pub enum Command{
    Modify(LeagueCommand),
    Serialize,
    CloseClient,
    Quit,
}