mod league;
mod com;

use crate::league::league::League;
use crate::com::command::LeagueCommand;

use actix_files as fs;

use clap::Clap;
use std::path::Path;
use chrono::prelude::*;

use std::sync::Mutex;
use std::sync::Arc;

use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};

#[get("/")]
async fn single(data: web::Data<Arc<Mutex<(String,League)>>>) -> impl Responder {
    let g = data.lock().unwrap();
    HttpResponse::Ok()
        .body(com::generator::create_html(&g.1))
}

#[post("/api")]
async fn api(ctx : web::Data<Arc<Mutex<(String,League)>>>, payload : web::Json<LeagueCommand>) -> impl Responder {
    println!("Got API call ({:?})", payload);

    let mut g = match ctx.lock() {
        Ok(g) => g,
        _ => return HttpResponse::BadGateway().body("{\"Message\":\"Failed\"}")
    };
    let path = g.0.clone();

    let league = &mut g.1;

    // TODO beautify

    let resp = serde_json::to_string(&payload.0).unwrap();

    match payload.0
    {
        LeagueCommand::AddGame(game) => {
            // TODO Move to a function
            if let Err(err) = league.add_game(&game) {
                println!("{}", err);
            }
            save(&path, &league);
        }
        LeagueCommand::RemoveGames(game) => {
            if let Err(err) = league.remove_game(&game) {
                println!("{}", err);
            }
            save(&path, &league);
        }
    }

    HttpResponse::Ok()
        .body(resp)
}

#[derive(Clap)]
#[clap(version = "0.1", author = "Volksfest")]
struct Opts{
    #[clap()]
    config: String,
    #[clap(long)]
    players: Option<Vec<String>>,
    #[clap(long, default_value = "127.0.0.1:8080")]
    host:String,
}

fn save(file : &String, league : &League) {
    if let Ok(msg) = serde_json::to_string_pretty(league) {
        if let Err(_) = std::fs::write(file, msg) {
            println!("Could not write the save file");
        }
    } else {
        println!("Could not serialize");
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let opts: Opts = Opts::parse();

    let path = opts.config.clone();

    if !Path::new(&opts.config).exists() && opts.players.is_none() {
        println!("The config must either exist or you have to give players!");
        std::process::exit(1);
    }

    let league:League = if opts.players.is_none() {
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
        League::new(&opts.players.unwrap(),
                    Local::today().naive_local().iso_week().week())
    };

    // Check for correct input file
    if !league.is_consistent() {
        println!("Config file has a broken state!");
        std::process::exit(1);
    }

    // Create league context
    let shared_league = Arc::new(Mutex::new((path,league)));

    HttpServer::new(move ||
        App::new()
            .data(shared_league.clone())
            .service(single)
            .service(api)
            .service(fs::Files::new("/resource", "./asset").show_files_listing())
    )
        .bind(opts.host)?
        .run()
        .await

    /*
     TODO Better Web
       TODO gen TODOS...

     TODO Statistics

     TODO Telegram-Bot
     TODO Registration
     TODO Date with remembering
    */
}
