use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddGameArgs {
    pub first_player_win : bool,
    pub player1: (String, char),
    pub player2: (String, char),
    pub duration_min : usize,
    pub duration_sec : usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoveGameArgs{
    pub player1: String,
    pub player2: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub cmd : LeagueCommand,
    pub token : usize
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LeagueCommand {
    AddGame(AddGameArgs),
    RemoveGames(RemoveGameArgs)
    // TODO Do Statistics
    // TODO Do Debug
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateMatchArgs {
    pub idx : usize,
    pub dom : String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateArgs {
    pub matches : Vec<UpdateMatchArgs>,
    pub table_dom : String,
    pub processed : bool
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Respond {
    Update(UpdateArgs),
    Error(String),
    Token(usize),
    Nothing
}