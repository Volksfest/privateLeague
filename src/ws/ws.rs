use std::fs;

use std::io::prelude::*;
use std::net::TcpListener;

use std::thread::{spawn, JoinHandle};

use websocket::sync::server::upgrade::IntoWs;
use websocket::result::WebSocketResult;
use websocket::result::WebSocketError;

use crate::league::league::League;
use crate::Command;
use std::sync::mpsc::{Sender, channel};
use crate::parser::command::{AddGameArgs, LeagueCommand};

pub fn handle_client(client: &mut websocket::sync::Client<std::net::TcpStream>) -> Option<Command> {
    match client.recv_message() {
        WebSocketResult::Ok(ws_msg) => match ws_msg {

            // getting (text) message
            websocket::message::OwnedMessage::Text(t) => {
                let game_message: Result<AddGameArgs, serde_json::Error> = serde_json::from_str(t.as_str());
                match game_message {
                    Ok(args) => {
                        Some(Command::Modify(LeagueCommand::AddGame(args)))
                    },
                    Err(e) => None
                }
            },

            // getting close
            websocket::message::OwnedMessage::Close(_) => {
                Some(Command::CloseClient)
            }

            // ignore rest
            _ => None
        },
        WebSocketResult::Err(e) => match e {
            WebSocketError::NoDataAvailable => None,
            WebSocketError::IoError(s) => None,
            _ => {
                println!("Got client error: {}", e); None
            }
        }
    }
/*

            let channel_msg = rx.recv_timeout(std::time::Duration::from_millis(10));

            match channel_msg {
                Ok(league_msg) => {
                    client.send_message(&ws::Message::text(league_msg));
                },
                Err(e) => match e {
                    _ => ()
                }

            }
        }
        println!("Tschüü");
        client.shutdown().unwrap();*/
}

// Error means dont care anymore and no new ws client
pub fn handle_request(listener: &std::net::TcpListener, host : &String) -> Result<websocket::sync::Client<std::net::TcpStream>,()> {
    let stream = match listener.accept() {
        Ok((stream, addr)) => stream,
        Err(e) => return Err(()),
    };

    let filename = "asset/test.html";

    let content = fs::read_to_string(filename)
        .expect("Could not read file")
        .replace("[[ADDR]]", host.as_str());

    // Check if WS Upgrade
    let client: websocket::sync::Client<std::net::TcpStream> = match stream.into_ws() {
        // Do upgrade
        Ok(upgrade) => {
            match upgrade.accept() {
                Ok(client) => client,
                Err(_) => return Err(()),
            }
        },
        // Send HTTP Response if not upgrade
        Err(mut s) => {
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                content.len(),
                content
            );

            s.0.write(response.as_bytes()).unwrap();
            s.0.flush().unwrap();
            return Err(());
        },
    };

    // Do WS Stuff
    println!("Connected");
    return Ok(client);
}
