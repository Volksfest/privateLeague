mod league;
mod parser;
mod serialization;
mod websocket;

use crate::league::league::League;
use crate::parser::command::Command;
use crate::parser::command::GameArgs;

use clap::Clap;
use std::path::Path;

#[derive(Clap)]
#[clap(version = "0.1", author = "Hodor")]
struct Opts{
    #[clap()]
    config: String,
    #[clap(long)]
    players: Option<Vec<String>>,
}

fn main() {

    let opts: Opts = Opts::parse();

    if !Path::new(&opts.config).exists() && opts.players.is_none() {
        println!("The config must either exist or you have to give players!");
        std::process::exit(1);
    }

    let mut league:League = if opts.players.is_none() {
        if let Ok(content) = std::fs::read_to_string(&opts.config) {
            if let Ok(val) = serde_json::from_str(content.as_str()) {
                val
            } else {
                println!("Config is not a valid json file!");
                std::process::exit(1);
            }
        } else {
            println!("Could not read config!");
            std::process::exit(1);
        }
    } else {
        League::new(&opts.players.unwrap())
    };

    if !league.is_consistent() {
        println!("Config file has a broken state!");
        std::process::exit(1);
    }

    websocket::ws::do_web_stuff(&mut league);

    loop {
        //println!("{}", serde_json::to_string_pretty(&league).unwrap());
        println!("{}", league);

        // TODO exchange with WebService
        // Read input
        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Parse it
        let command = parser::parse_input(&mut guess);

        match command {
            Ok(cmd) =>
                match cmd {
                    Command::AddGame(args) =>
                        {league.add_game(&args);},
                    Command::Serialize =>
                        {
                            std::fs::write(
                                &opts.config,
                                serde_json::to_string_pretty(&league).unwrap());}
                    _ => ()

                }

            Err(str) =>
                println!("{}",str)
        }


    }

    /*
     TODO Command Object

     TODO Deserialization


     TODO WebService
     TODO Session-Liga Bind

     TODO Statistics

     TODO Telegram-Bot
     TODO Registration

     TODO Date with remembering
     */
}
