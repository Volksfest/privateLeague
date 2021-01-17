mod league;
mod parser;
mod serialization;
mod ws;

use crate::league::league::League;
use crate::parser::command::{Command, LeagueCommand};
use crate::parser::command::AddGameArgs;

use clap::Clap;
use std::path::Path;
use std::sync::mpsc::Sender;
use std::net::TcpListener;
use std::time::Duration;
use websocket::websocket_base::stream::sync::TcpStream;


#[derive(Clap)]
#[clap(version = "0.1", author = "Hodor")]
struct Opts{
    #[clap()]
    config: String,
    #[clap(long)]
    players: Option<Vec<String>>,
    #[clap(long, default_value = "127.0.0.1:8080")]
    host:String,
}

fn save(file : &String, league : &League) {
    std::fs::write(file, serde_json::to_string_pretty(league).unwrap());
}

fn keyboard_input(sender : Sender<Command>) {
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

    let listener = TcpListener::bind(opts.host.clone()).unwrap();
    listener.set_nonblocking(true).expect("Cannot set non-blocking");

    let mut clients =Vec::new();

    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(|| keyboard_input(sender));

    loop {

        let mut some_msg : Option<Command>;
        let mut idx : usize = 0;

        // TODO maybe a bit better structure... this is a bit weird as it handles http quite arbitrarily
        // handle new requests
        match ws::ws::handle_request( &listener, &opts.host) {
            // new client
            Ok(mut client) => {
                client.set_nonblocking(true);
                // TODO remove by building the initial dom before?!?
                client.send_message(&websocket::Message::text(serde_json::to_string(&league).unwrap()));
                clients.push(client);},
            // unimportant error or handled http request
            Err(()) => (),
        }

        // handle receive from keyboard
        some_msg = match receiver.try_recv() {
            Ok(m) => Some(m),
            Err(e) => None // no data or disconnect; latter should not be able to happen...
        };

        // handle receive from clients
        // TODO [actually will be never done] make a _fair_ scheduling to prevent DOS
        if some_msg.is_none() {
            for client in &mut clients {
                match ws::ws::handle_client(client) {
                    Some(cmd) => {
                        some_msg = Some(cmd);
                        break; }
                    _ => (),
                }
                idx += 1;
            }
        }

        let msg = match some_msg {
            Some(m) => m,
            None => {
                std::thread::sleep(Duration::from_millis(10));
                continue;
            }
        };

        match msg {
            Command::Modify(args) => match args {
                LeagueCommand::AddGame(game) => {
                    league.add_game(&game);
                    save(&opts.config, &league);
                    for client in &mut clients {
                        client.send_message(&websocket::Message::text(serde_json::to_string(&league).unwrap()));
                    }
                },
                LeagueCommand::RemoveGames(game) => {
                    league.remove_game(&game);
                    save(&opts.config, &league);
                    for client in &mut clients {
                        client.send_message(&websocket::Message::text(serde_json::to_string(&league).unwrap()));
                    }
                },
            },
            Command::Serialize => {
                save(&opts.config, &league);
            },
            Command::Quit => break,
            Command::CloseClient => {clients.remove(idx); println!("Disconnected")},
        };
    }

    /*
     TODO Better Web
       TODO gen TODOS...

     TODO Statistics

     TODO Telegram-Bot
     TODO Registration
     TODO Date with remembering
     */
}
