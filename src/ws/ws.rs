use std::fs;

use std::io::prelude::*;

use websocket::sync::server::upgrade::IntoWs;
use websocket::result::WebSocketResult;
use websocket::result::WebSocketError;

use crate::Command;
use crate::parser::command::LeagueCommand;

pub fn handle_client(client: &mut websocket::sync::Client<std::net::TcpStream>) -> Result<Option<Command>, String> {
    match client.recv_message() {
        WebSocketResult::Ok(ws_msg) => match ws_msg {

            // getting (text) message
            websocket::message::OwnedMessage::Text(t) => {
                //let game_message: Result<AddGameArgs, serde_json::Error> = serde_json::from_str(t.as_str());
                let game_message : Result<LeagueCommand, serde_json::Error> = serde_json::from_str(t.as_str());
                match game_message {
                    Ok(args) => {
                        Ok(Some(Command::Modify(args)))
                    },
                    Err(e) => Err(e.to_string())
                }
            },

            // getting close
            websocket::message::OwnedMessage::Close(_) => {
                Ok(Some(Command::CloseClient))
            }

            // ignore rest
            _ => Ok(None)
        },
        WebSocketResult::Err(e) => match e {
            WebSocketError::NoDataAvailable => Ok(None),
            WebSocketError::IoError(_) => Ok(None),
            e => Err(e.to_string())
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
pub fn handle_request(listener: &std::net::TcpListener, host : &String) -> Result<websocket::sync::Client<std::net::TcpStream>, Option<String>> {
    let stream = match listener.accept() {
        Ok((stream, _)) => stream,
        Err(e) => return match e.raw_os_error() {
            Some(11) => Err(None),
            _ => Err(Some(e.to_string())),
        },
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
                Err((_,e)) => return Err(Some(e.to_string())),
            }
        },
        // Send HTTP Response if not upgrade
        Err(mut s) => {
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                content.len(),
                content
            );

            if s.0.write(response.as_bytes()).is_err() {
                return Err(Some(String::from("Error on write")));
            }
            if s.0.flush().is_err() {
                return Err(Some(String::from("Error on flush")));
            }
            return Err(None);
        },
    };

    // Do WS Stuff
    println!("Connected");
    return Ok(client);
}
