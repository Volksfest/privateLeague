use std::fs;

use std::io::prelude::*;
use std::net::TcpListener;

use std::thread::{spawn, JoinHandle};

use websocket::sync::server::upgrade::IntoWs;

use crate::league::league::League;

/*
fn do_websocket_stuff(mut client : websocket::sync::Client<std::net::TcpStream>, league : &League ) -> JoinHandle<()>{
    spawn(move || {
        loop {
            match client.recv_message().unwrap() {

                // getting (text) message
                websocket::message::OwnedMessage::Text(t) => {
                    println!("{}", t);
                    let str = serde_json::to_string_pretty(league).unwrap();
                    client.send_message(&websocket::Message::text(str)).unwrap();
                },

                // getting close
                websocket::message::OwnedMessage::Close(_) => {
                    break;
                }

                // ignore rest
                _ => continue
            };
        }
        println!("Tschüü");
        client.shutdown().unwrap();
    })
}*/

pub fn do_web_stuff(league : &mut League) {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    let filename = "asset/test.html";




    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let content = fs::read_to_string(filename)
            .expect("Could not read bitch");

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

        //let _handle = do_websocket_stuff(client, league);

        loop {
            match client.recv_message().unwrap() {

                // getting (text) message
                websocket::message::OwnedMessage::Text(t) => {
                    println!("{}", t);
                    let str = serde_json::to_string_pretty(league).unwrap();
                    client.send_message(&websocket::Message::text(str)).unwrap();
                },

                // getting close
                websocket::message::OwnedMessage::Close(_) => {
                    break;
                }

                // ignore rest
                _ => continue
            };
        }
        println!("Tschüü");
        client.shutdown().unwrap();

    }
}