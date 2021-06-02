use serde::{Serialize, Deserialize};
use crate::league::league::League;
use crate::league::game::Game;

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
    AddGame(Game),
    RemoveGames(RemoveGameArgs)
    // TODO Do Statistics
    // TODO Do Debug
}

impl LeagueCommand {
    pub fn get_match_idx(&self, league: &League) -> Option<usize> {
        match match self {
            LeagueCommand::AddGame(game) => league.get_match_idx(&game.players[0].name, &game.players[1].name),
            LeagueCommand::RemoveGames(game) => league.get_match_idx(&game.player1, &game.player2)
        } {
            None => None,
            Some((idx, _)) => Some(idx)
        }
    }
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
    Token(usize)
}