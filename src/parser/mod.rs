pub mod command;

use command::Command;
use crate::parser::command::{GameArgs, LeagueCommand, ControlCommand};

pub fn parse_input_for_game(literals : &Vec<&str>) -> Result<Command,String> {
    if literals.len() < 5 {
        return Err(String::from("Usage: Game [Player] [Player] [race_char] [race_char]"));
    }

    let player1 = String::from(literals[1]);
    let player2 = String::from(literals[2]);

    // TODO beautify
    let c1 = match literals[3].chars().next() {
        Some(e) => match e {
            'z' | 't'| 'p' | 'Z' | 'T' | 'P' => e,
            _ => {return Err(String::from("z t p needed as race"));}

        },
        None => {return Err(String::from("Char needed for race"));}
    };

    let c2 = match literals[4].chars().next() {
        Some(e) => match e {
            'z' | 't'| 'p' | 'Z' | 'T' | 'P' => e,
            _ => {return Err(String::from("z t p needed as race"));}

        },
        None => {return Err(String::from("Char needed for race"));}
    };

    if c1.is_uppercase() == c2.is_uppercase() {
        return Err(String::from("Exactly one winner is needed"));

    }

    Ok(Command::Modify(LeagueCommand::AddGame(GameArgs{
        player1: (player1, c1.to_ascii_lowercase()),
        player2: (player2, c2.to_ascii_lowercase()),
        first_player_win: c1.is_uppercase(),
        duration_min: 2,
        duration_sec: 37
    })))
}

pub fn parse_input(guess : &mut String) -> Result<Command, String> {
    guess.retain(|c| c != '\n');
    let literals= guess.split(" ").collect::<Vec<&str>>();

    if literals.len() < 1 {
        return Err(String::from("Need at least a command..."));
    }

    match literals[0] {
        "Game" => parse_input_for_game(&literals),
        "Save" => Ok(Command::Control(ControlCommand::Serialize)),
        _ => Err(String::from("Command not known"))
    }

}