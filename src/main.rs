mod league;
mod com;

use crate::league::league::League;
use crate::com::command::{LeagueCommand, Respond, UpdateArgs, UpdateMatchArgs};

use actix_files as fs;

use clap::Clap;
use std::path::Path;
use chrono::prelude::*;

use std::sync::Mutex;
use std::sync::Arc;

use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};
use crate::com::generator::{create_single_match, create_table};
use crate::com::command::Respond::{Update, Error};
use serde::Serialize;

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

#[get("/get_token")]
async fn get_token(ctx : web::Data<Arc<Mutex<Context>>>) -> impl Responder {
    let g = ctx.lock().unwrap();
    HttpResponse::Ok()
        .body(serde_json::to_string(&Respond::Token(
            g.stack.len()
        )).unwrap())
}

fn create_response<T: Serialize>(payload : T) -> HttpResponse {
    HttpResponse::Ok().json(payload)
}

fn create_diff_update(ctx : &Context, token: usize, updated : bool) -> Option<Respond> {
    if token >= ctx.stack.len() || ctx.stack.len() == 0 {
        return None;
    }

    let mut update = UpdateArgs{
        matches: Vec::new(),
        table_dom: create_table(&ctx.league).print(),
        processed: updated
    };

    let mut idx_list = Vec::new();

    for cmd in ctx.stack.split_at(token).1 {
        match cmd.get_match_idx(&ctx.league) {
            None => continue,
            Some(idx) => if !idx_list.contains(&idx) {
                update.matches.push(UpdateMatchArgs {
                    idx,
                    dom: create_single_match(&ctx.league, idx).print(),
                });
                idx_list.push(idx);
            }
        }
    }

    Some(Update(update))
}

#[post("/api")]
async fn api(ctx : web::Data<Arc<Mutex<Context>>>, payload : web::Json<com::command::Request>) -> impl Responder {
    println!("Got API call ({:?})", payload);

    let mut g = match ctx.lock() {
        Ok(g) => g,
        _ => {return HttpResponse::BadGateway().finish();}
    };

    if payload.token != g.stack.len() {
        return create_response(create_diff_update(&*g, payload.token,false));
    }

    let path = g.path.clone();

    let league = &mut g.league;

    let update = match &payload.cmd
    {
        LeagueCommand::AddGame(game) => {
            league.add_game(&game)
        }
        LeagueCommand::RemoveGames(game) =>{
            league.remove_game(&game)
        }
    };

    match update {
        Ok(_) => {
            save(&path, &g.league);
            g.stack.push(payload.cmd.clone());
            create_response(create_diff_update(&*g, payload.token, true))
        },
        Err(e) => {
            println!("{}", e);
            create_response(Error(e))
        }
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
        let new_league = League::new(&opts.players.unwrap(),
                    Local::today().naive_local().iso_week().week());
        save(&path, &new_league);
        new_league
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
            .service(get_token)
            .service(fs::Files::new("/resource", "./asset"))
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
