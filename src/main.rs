mod league;
mod parser;
mod serialization;
mod websocket;

use crate::league::league::League;
use crate::parser::command::{Command, LeagueCommand, ControlCommand};
use crate::parser::command::GameArgs;

use clap::Clap;
use std::path::Path;
use std::sync::mpsc::Sender;

#[derive(Clap)]
#[clap(version = "0.1", author = "Hodor")]
struct Opts{
    #[clap()]
    config: String,
    #[clap(long)]
    players: Option<Vec<String>>,
}

fn keyboard_input() {

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

    let (sender, receiver) = std::sync::mpsc::channel();

    let client_send_channel = sender.clone();
    std::thread::spawn(move || websocket::ws::wait_for_clients(client_send_channel));

    let mut client_channels : Vec<Sender<String>> = Vec::new();

    std::thread::spawn(move || {
        loop {
            let mut guess = String::new();
            std::io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            // Parse it
            let command = parser::parse_input(&mut guess);
            match command {
                Ok(cmd) => {sender.send(cmd);},
                Err(e) => {println!("{}", e);},
            }
        }
    });

    loop {
        let msg = match receiver.recv() {
            Ok(m) => m,
            Err(e) => {println!("Got an error: {}", e); continue;}
        };

        match msg {
            Command::Modify(args) => match args {
                LeagueCommand::AddGame(game) => {
                    println!("Got a game");
                    league.add_game(&game);
                    for channel in &client_channels {
                        channel.send(serde_json::to_string(&league).unwrap());
                    }
                }
            },
            Command::Control(ctrl) => match ctrl {
                ControlCommand::Serialize => {
                    std::fs::write(&opts.config,serde_json::to_string_pretty(&league).unwrap());
                }
                ControlCommand::NewClient(client) => {
                    client.send(serde_json::to_string(&league).unwrap());
                    client_channels.push(client);
                }
            }
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
