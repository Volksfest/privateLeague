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

// TODO too lazy now -> summerize all client into one thread (actually then only one channel to the clients is needed!)
fn handle_client(mut client : websocket::sync::Client<std::net::TcpStream>, sender : Sender<Command> ) {

        let (tx, rx) = channel();
        sender.send(Command::NewClient(tx));

        client.set_nonblocking(true);

        loop {

            let client_msg = client.recv_message();

            match client_msg {

                WebSocketResult::Ok(ws_msg) => match ws_msg {

                        // getting (text) message
                        websocket::message::OwnedMessage::Text(t) => {
                            println!("{}", t);
                            //let str = serde_json::to_string_pretty(league).unwrap();
                            //client.send_message(&websocket::Message::text(str)).unwrap();
                        },

                        // getting close
                        websocket::message::OwnedMessage::Close(_) => {
                            break;
                        }

                        // ignore rest
                        _ => continue
                    },
                WebSocketResult::Err(e) => match e {
                    WebSocketError::NoDataAvailable => (),
                    WebSocketError::IoError(s) => (),
                    _ => {println!("Got client error: {}", e); continue;}

                }
            }

            let channel_msg = rx.recv_timeout(std::time::Duration::from_millis(10));

            match channel_msg {
                Ok(league_msg) => {
                    client.send_message(&websocket::Message::text(league_msg));
                },
                Err(e) => match e {
                    _ => ()
                }

            }
        }
        println!("Tschüü");
        client.shutdown().unwrap();
}

pub fn wait_for_clients(sender : Sender<Command>) {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    let filename = "asset/test.html";

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let content = fs::read_to_string(filename)
            .expect("Could not read file");

        // Check if WS Upgrade
        let mut client: websocket::sync::Client<std::net::TcpStream> = match stream.into_ws() {
            // Do upgrade
            Ok(upgrade) => {
                match upgrade.accept() {
                    Ok(client) => client,
                    Err(_) => panic!(),
                }
            },
            // Send HTTP Response if not upgrade
            Err(mut s) => {
                //s.1.unwrap().
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                    content.len(),
                    content
                );

                s.0.write(response.as_bytes()).unwrap();
                s.0.flush().unwrap();
                continue;
            },
        };

        // Do WS Stuff
        println!("Connected");
        let clone = sender.clone();
        spawn(move || handle_client(client, clone));
    }
}