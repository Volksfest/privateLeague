mod league;
mod com;

use crate::league::league::League;
use crate::com::command::{LeagueCommand, Respond, UpdateArgs};

use actix_files as fs;

use clap::Clap;
use std::path::Path;
use chrono::prelude::*;

use std::sync::Mutex;
use std::sync::Arc;

use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};
use crate::com::generator::create_single_match;

struct Context {
    path : String,
    league : League,
    stack : Vec<LeagueCommand>,
}

#[get("/")]
async fn single(data: web::Data<Arc<Mutex<Context>>>) -> impl Responder {
    let g = data.lock().unwrap();
    HttpResponse::Ok()
        .body(com::generator::create_html(&g.league))
}

fn process<A, F>(league: &mut League, path : &String, args: A, f: F) -> HttpResponse
where F: Fn(&mut League, A) -> Result<Option<usize>, String> {

    let resp = match f(league, args) {
        Ok(Some(idx)) =>
            HttpResponse::Ok().body(
                serde_json::to_string(&Respond::Update(UpdateArgs{idx, dom: create_single_match(league, idx).print()})).unwrap()
            ),
        Ok(None) =>
            HttpResponse::Ok().body(
                serde_json::to_string(&Respond::Message("Ok".to_string())).unwrap()
            ),
        Err(e) => {
            println!("{}", e);
            return HttpResponse::BadRequest().body(
                serde_json::to_string(&Respond::Error(e)).unwrap()
            )
        }
    };
    save(path, league);

    resp

}

#[post("/api")]
async fn api(ctx : web::Data<Arc<Mutex<Context>>>, payload : web::Json<LeagueCommand>) -> impl Responder {
    println!("Got API call ({:?})", payload);

    let mut g = match ctx.lock() {
        Ok(g) => g,
        _ => return HttpResponse::BadGateway().body("{\"Message\":\"Failed\"}")
    };
    let path = g.path.clone();

    let league = &mut g.league;

    /*
    let idx = league.get_match_idx()
    let resp = serde_json::to_string(
        Respond::Update(
            UpdateArgs{idx, dom: create_single_match(&league, 1).print()}
        )
    ).unwrap();
    */

    match payload.0
    {
        LeagueCommand::AddGame(game) =>

            process(league, &path, game,
                    |league : &mut League, game| -> Result<Option<usize>, String> {
                        let idx = league.get_match_idx(&game.player1.0, &game.player2.0);
                        if idx.is_none() {
                            return Err("Match does not exist".to_string());
                        }

                        league.add_game(&game)?;
                        Ok(Some(idx.unwrap().0))
                    }),
        LeagueCommand::RemoveGames(game) =>
            process(league, &path, game,
                |league : &mut League, game| -> Result<Option<usize>, String> {
                    let idx = league.get_match_idx(&game.player1, &game.player2);
                    if idx.is_none() {
                        return Err("Match does not exist".to_string());
                    }

                    league.remove_game(&game)?;
                    Ok(Some(idx.unwrap().0))
                })
    }
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

    let context = Context{
        path,
        league,
        stack:Vec::new()
    };

    // Create league context
    let shared_context = Arc::new(Mutex::new(context));

    HttpServer::new(move ||
        App::new()
            .data(shared_context.clone())
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
