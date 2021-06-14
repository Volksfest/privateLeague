mod league;
mod com;

use crate::league::league::League;
use crate::league::matches::Winner;
use crate::league::game::Game;
use crate::com::command::{LeagueCommand, Respond, UpdateArgs, UpdateMatchArgs, RemoveGameArgs};
use crate::com::generator::{create_single_match, create_table};

use clap::Clap;
use std::path::Path;
use chrono::prelude::*;

use uuid::Uuid;

use std::sync::Mutex;
use std::sync::Arc;

use actix_web::{get, post, web, App, HttpServer, Error, Responder, HttpResponse};
use actix_files;
use actix_multipart::Multipart;

use futures::{StreamExt, TryStreamExt};
use std::io::Write;
use std::process::Command;

use lettre;
use lettre_email;
use lettre::Transport;
use serde::Deserialize;
use horrorshow::Template;

struct Context {
    secret : String,
    path : String,
    league : League,
    stack : Vec<LeagueCommand>,
}

#[get("/")]
async fn index(data: web::Data<Arc<Mutex<Context>>>) -> impl Responder {
    let g = data.lock().unwrap();
    HttpResponse::Ok()
        .body(com::generator::create_html(&g.league).into_string().unwrap())
}

#[get("/get_token")]
async fn get_token(ctx : web::Data<Arc<Mutex<Context>>>) -> impl Responder {
    let g = ctx.lock().unwrap();
    HttpResponse::Ok()
        .body(serde_json::to_string(&Respond::Token(
            g.stack.len()
        )).unwrap())
}


#[post("/update")]
async fn update(ctx : web::Data<Arc<Mutex<Context>>>, payload : web::Json<com::command::Request>) -> impl Responder {
    let g = match ctx.lock() {
        Ok(g) => g,
        _ => {return HttpResponse::BadGateway().finish();}
    };

    if payload.token == g.stack.len() {
        return HttpResponse::Ok().finish();
    }

    let mut update_list = UpdateArgs{
        matches: Vec::new(),
        table_dom: create_table(&g.league).into_string().unwrap(),
        processed: false,
        token: g.stack.len()
    };

    let mut idx_list = Vec::new();

    for cmd in g.stack.split_at(payload.token).1 {
        match cmd.get_match_idx(&g.league) {
            None => continue,
            Some(idx) => if !idx_list.contains(&idx) {
                update_list.matches.push(UpdateMatchArgs {
                    idx,
                    dom: create_single_match(&g.league, idx).into_string().unwrap(),
                });
                idx_list.push(idx);
            }
        }
    }

    return HttpResponse::Ok().json(Respond::Update(update_list));
}

#[post("/upload/{secret}")]
async fn upload(path: web::Path<String>, mut payload: Multipart, ctx: web::Data<Arc<Mutex<Context>>>) -> Result<HttpResponse, Error> {

    let mut g = ctx.lock().unwrap();
    if g.secret != path.into_inner() {
        return Ok(HttpResponse::Forbidden().into());
    }

    // iterate over multipart stream
    // should actually be only the file
    while let Ok(Some(mut field)) = payload.try_next().await {
        // create random tmp file
        let filepath = format!("./tmp/{}", Uuid::new_v4().to_simple().to_string());
        let cloned_path = filepath.clone();
        let cloned_path_2 = filepath.clone();

        // create file
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap(); // TODO handle unwrap

        // write file
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap(); // TODO handle unwrap
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }

        // call replay parser and retrieve output
        let output =  web::block(
                || Command::new("./parser.py")
                    .arg(cloned_path)
                    .output()
        )
            .await
            .unwrap(); // TODO handle unwrap

        // seperate stdout which contains the JSON result of the replay (see the parser.py)
        let output = String::from_utf8(output.stdout)
            .unwrap(); // TODO handle unwrap

        // Deserialize JSON
        let game_res = serde_json::from_str(output.as_str());
        let game : Game = match game_res {
            Ok(game) => game,
            Err(_) => {
                return Ok(HttpResponse::BadRequest().into());
            }
        };

        // Check if game is valid
        if !game.is_valid() {
            return Ok(HttpResponse::BadRequest().into());
        }

        let m = &g.league.matches[
            g.league.get_match_idx(&game.players[0].name, &game.players[1].name)
                .unwrap().0 //TODO handle unwrao
            ];

        // Check if match still needs a game
        match m.winner() {
            Winner::None => return Ok(HttpResponse::BadRequest().into()),
            _ => ()
        };

        // Check if game already exists
        for old_game in m.get_games() {
            if *old_game == game {
                return Ok(HttpResponse::BadRequest().into());
            }
        }

        // create a useful file name for success
        let new_filepath = format!("replays/{}_{}_{}.SC2Replay", game.players[0].name, game.players[1].name, m.get_games().len());

        // push Game to the league
        match g.league.add_game(&game) {
            Ok(_) => {
                // Save league
                save(&g.path, &g.league);
                let cmd = LeagueCommand::AddGame(game);
                // add copy of game to the stack
                // TODO stack should be updated in general
                g.stack.push(cmd);
            }
            Err(_) => {
                return Ok(HttpResponse::BadRequest().into());
            }
        }

        // move file
        web::block(move || std::fs::rename(cloned_path_2,new_filepath)).await?;
    }
    Ok(HttpResponse::Ok().into())
}

#[get("/replay/{player_1}/{player_2}/{idx}")]
async fn replay(path: web::Path<(String,String,String)>) -> Result<actix_files::NamedFile, Error> {
    let path = path.into_inner();

    println!("replays/{}_{}_{}.SC2Replay", path.1, path.0, path.2);

    match actix_files::NamedFile::open(format!("replays/{}_{}_{}.SC2Replay", path.1, path.0, path.2)) {
        Ok(f) => Ok(f),
        Err(_) => Ok(actix_files::NamedFile::open(format!("replays/{}_{}_{}.SC2Replay", path.0, path.1, path.2))?)
    }
}

#[get("/remove/{secret}/{player_1}/{player_2}")]
async fn remove(path: web::Path<(String,String,String)>, ctx: web::Data<Arc<Mutex<Context>>>) -> impl Responder {
    let mut g = ctx.lock().unwrap();

    let path = path.into_inner();

    if g.secret != path.0 {
        return HttpResponse::Forbidden().finish();
    }

    let args = RemoveGameArgs{player1:path.1, player2:path.2};

    match g.league.remove_game(&args) {
        Ok(_) => {
            save(&g.path, &g.league);
            let cmd = LeagueCommand::RemoveGames(args);
            g.stack.push(cmd);
        }
        Err(e) => {
            return HttpResponse::Ok().json(Respond::Error(e));
        }
    }

    HttpResponse::Ok().finish()
}

#[derive(Clap)]
#[clap(version = "0.1", author = "Volksfest")]
struct Opts{
    #[clap()]
    config: String,
    #[clap(long)]
    players: Option<Vec<String>>,
    #[clap(long)]
    kick: Option<Vec<String>>,
    #[clap(long, default_value = "127.0.0.1:8080")]
    host:String,
    #[clap(long)]
    mail: Option<String>
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

#[derive(Deserialize)]
struct MailConfig{
    from : String,
    to : String,
    pw : String,
    smtp : String
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

    let league = match opts.kick {
        None => league,
        Some(banned) => {
            let mut new_path =opts.config.clone();
            new_path.push_str(".before_purge");
            save(&new_path, &league);
            league.kick(banned)
        }
    };

    let context = Context{
        secret:Uuid::new_v4().to_simple().to_string(),
        path,
        league,
        stack:Vec::new()
    };

    println!("{}", context.secret);

    if opts.mail.is_some() {
        let mail_opt: MailConfig = serde_json::from_str(&std::fs::read_to_string(opts.mail.unwrap()).unwrap()).unwrap();

        let mut mailer = lettre::SmtpClient
        ::new_simple(mail_opt.smtp.as_str()).unwrap()
            .credentials(
                lettre::smtp::authentication::Credentials::new(
                    mail_opt.from.clone(),
                    mail_opt.pw,
                )
            )
            .transport();

        let email = lettre_email::EmailBuilder::new()
            .from(mail_opt.from)
            .subject("[LIGA] Token")
            .text(&context.secret)
            .to(mail_opt.to)
            .build()
            .unwrap();

        mailer.send(email.into()).unwrap();
    }

    // Create league context
    let shared_context = Arc::new(Mutex::new(context));

    HttpServer::new(move ||
        App::new()
            .data(shared_context.clone())
            .service(index)
            .service(get_token)
            .service(update)
            .service(upload)
            .service(replay)
            .service(remove)
            .service(actix_files::Files::new("/resource", "./asset"))
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
