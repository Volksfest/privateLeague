mod league;
mod parser;
mod ws;

use crate::league::league::League;
use crate::league::game::Race;
use crate::parser::command::{Command, LeagueCommand};

use actix_files as fs;

use clap::Clap;
use std::path::Path;
use std::sync::mpsc::Sender;
use std::net::TcpListener;
use std::time::Duration;
use std::cmp::Ordering;
use chrono::prelude::*;

use std::sync::Mutex;
use std::sync::Arc;

use actix_web::{get, web, App, HttpServer, Responder, HttpRequest, HttpResponse};
use std::ops::{DerefMut, Deref};

fn create_html(league : &League) -> String {
    let mut builder = string_builder::Builder::default();


    builder.append("
    <head>
        <meta charset=\"UTF-8\">
        <title>StartCraft 2 Private League</title>
        <link rel=\"stylesheet\" href=\"resource/style.css\" />
    </head>");

    builder.append(r#"
<body>
    <div id="container">
        <h2>StartCraft 2 Private League - Season 2</h2>

        <div id="output">
            <div id="player">
            "#);
    // Player

    let score = league.get_score();

    builder.append("<div class=\"row\">
        <div class=\"col1\">#</div>\
        <div class=\"col2\">Name</div>\
        <div class=\"col3\">Sp</div>\
        <div class=\"col4\">S</div>\
        <div class=\"col5\">N</div></div>");

    let mut add_row = | num : usize , name : &String, win:usize, lose : usize, class : String| {
        builder.append(format!(
            "<div class=\"{}\">
            <div class=\"col1\">{}</div>\
            <div class=\"col2\">{}</div>\
            <div class=\"col3\">{}</div>\
            <div class=\"col4\">{}</div>\
            <div class=\"col5\">{}</div></div>",class, num, name, win+lose, win, lose));
    };

    let mut sorted_score = score.clone();
    sorted_score.sort_by(|a,b|
        match a.1.cmp(&b.1) {
            Ordering::Equal => a.2.cmp(&b.2),
            Ordering::Less => Ordering::Greater,
            Ordering::Greater => Ordering::Less
        }
    );

    for (i,r) in sorted_score.iter().enumerate() {
        let mut class = "row";
        if i == 0 { class = "row winner_back" }
        if i == score.len() - 1 { class = "row loser_back" }

        add_row(i+1, &r.0, r.1, r.2, String::from(class));
    }

    builder.append(r#"
            </div>

            <div id="match">
            "#);
    // Matches


    let todays_week = Local::today().naive_local().iso_week().week();
    let mpw = league.matches_per_week();
    for w in 0..league.weeks_count() {
        let begin_date = NaiveDate::from_isoywd(Local::today().year(), league.start_week + w as u32, Weekday::Mon);
        let end_date = NaiveDate::from_isoywd(Local::today().year(), league.start_week + w as u32, Weekday::Sun);
        builder.append(format!(r#"<div class="week{}"><div class="date">{:02}.{:02}<br>-<br>{:02}.{:02}</div>"#,
            if league.start_week + w as u32== todays_week {" highlighted"} else {""},
            begin_date.day(), begin_date.month(),
            end_date.day(), end_date.month()
        ));

        for i in 0..mpw {
            let m = league.get_match(w * mpw + i).unwrap();
            let winner = match m.winner() {
                None => None,
                Some(t) => Some(t == m.get_first_player())
            };
            builder.append(format!(
               r#"<div class="{}">
                    <div class="{}">{}</div>
                    <div class="{}">{}</div>
                  "#,
               match winner {
                    None => if m.empty() {"unplayed"} else {"ongoing"},
                    Some(_) => "played"
               },
               match winner {
                   None => "normal_player",
                   Some(t) => if t {"winner"} else {"loser"}
               }, score[m.get_first_player()].0,
               match winner {
                   None => "normal_player",
                   Some(t) => if t {"loser"} else {"winner"}
               }, score[m.get_second_player()].0
            ));

            if !m.empty() {
                builder.append(r#"<div class="info"><div class="score"><div>"#);
                    for (won,race) in m.get_first_player_data() {
                        builder.append(format!(r#"<span class="{}">{}</span>"#,
                        if won {"winner"} else {"loser"},
                        race.race_to_char()));
                    }
                builder.append(r#"</div><div>"#);
                    for (won,race) in m.get_second_player_data() {
                        builder.append(format!(r#"<span class="{}">{}</span>"#,
                        if won {"winner"} else {"loser"},
                        race.race_to_char()));
                    }
                builder.append(r#"</div></div></div>"#);
            }
            builder.append("</div>");
        }
        builder.append("</div>");
    }


    builder.append(r#"
            </div>
        </div>
    </div>

    <div id="popup" class="hidden">
        <datalist id="race">
            <option value="Terran"></option>
            <option value="Zerg"></option>
            <option value="Protoss"></option>
        </datalist>

        <div class="row">
            <label id="first_player_label" for="first_player_race_input"></label>
            <input id="first_player_race_input" name="first_player_race" list="race" onfocus="this.value=''"/>
            <input type="radio" id="first_player_win_radio" name="first_player_won" value="true" checked />
        </div>
        <div class="row">
            <label id="second_player_label" for="second_player_race_input"></label>
            <input id="second_player_race_input" name="second_player_race" list="race" onfocus="this.value=''"/>
            <input type="radio" id="second_player_win_radio" name="first_player_won" value="false" />
        </div>
        <div class="row">
            <label id="duration_label" for="duration_text">Spieldauer</label>
            <input type="text" id="duration_text" name="duration" value="0:00" />
        </div>
        <div class="row">
            <div id="error_output"></div>
        </div>
        <div class="row">
            <input type="button" id="submit" name="game_submit" value="Add" style="margin-right:5px" onclick="addGame()"/>
            <input type="button" id="cancel" name="game_cancel" value="Cancel" onclick="hidePopup()"/>
        </div>
    </div>
    <script src="/resource/script.js"></script>
</body>
"#);


    builder.string().unwrap()
}

#[get("/")]
async fn single(data: web::Data<Arc<Mutex<League>>>) -> impl Responder {
    let league = data.lock().unwrap();
    HttpResponse::Ok()
        .body(create_html(&league))
}

#[get("/api/")]
async fn api() -> impl Responder {
    HttpResponse::Ok().body("Hi")
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

fn keyboard_input(sender : Sender<Command>) {
    loop {
        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Parse it
        let command = parser::parse_input(&mut guess);
        match command {
            Ok(cmd) => { if let Err(_) = sender.send(cmd) {
                println!("Could not send cmd from input");
            }},
            Err(e) => {println!("{}", e);},
        }
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
        League::new(&opts.players.unwrap(),
                    Local::today().naive_local().iso_week().week())
    };

    if !league.is_consistent() {
        println!("Config file has a broken state!");
        std::process::exit(1);
    }

    // League checked

    let shared_league = Arc::new(Mutex::new(league));

    HttpServer::new(move ||
        App::new()
            .data(shared_league.clone())
            .service(single)
            .service(api)
            .service(fs::Files::new("/resource", "./asset").show_files_listing())
    )
        .bind("127.0.0.1:8080")?
        .run()
        .await


/*
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
                if let Err(_) = client.set_nonblocking(true) {
                    println!("Some Error with setting nonblocking");
                }
                // TODO remove by building the initial dom before?!?
                if let Ok(text_msg) = serde_json::to_string(&league) {
                    if client.send_message(&websocket::Message::text(text_msg)).is_err() {
                        println!("Some Error with Websocket")
                    }
                }
                clients.push(client);},
            // unimportant error or handled http request
            Err(Some(e)) => println!("{}",e),
            Err(None) => (),
        }

        // handle receive from keyboard
        some_msg = match receiver.try_recv() {
            Ok(m) => Some(m),
            Err(_) => None // no data or disconnect; latter should not be able to happen...
        };

        // handle receive from clients
        // TODO [actually will be never done] make a _fair_ scheduling to prevent DOS
        if some_msg.is_none() {
            for client in &mut clients {
                match ws::ws::handle_client(client) {
                    Ok(Some(cmd)) => {
                        some_msg = Some(cmd);
                        break; },
                    Err(e) => println!("{}",e),
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
                    // TODO Move to a function
                    if let Err(err) = league.add_game(&game) {
                        println!("{}", err);
                    }
                    save(&opts.config, &league);
                    for client in &mut clients {
                        if let Ok(text_msg) = serde_json::to_string(&league) {
                            if client.send_message(&websocket::Message::text(text_msg)).is_err() {
                                println!("Some Error with Websocket")
                            }
                        }
                    }
                },
                LeagueCommand::RemoveGames(game) => {
                    if let Err(err) = league.remove_game(&game) {
                        println!("{}", err);
                    }
                    save(&opts.config, &league);
                    for client in &mut clients {
                        if let Ok(text_msg) = serde_json::to_string(&league) {
                            if client.send_message(&websocket::Message::text(text_msg)).is_err() {
                                println!("Some Error with Websocket")
                            }
                        }
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


 */
    /*
     TODO Better Web
       TODO gen TODOS...

     TODO Statistics

     TODO Telegram-Bot
     TODO Registration
     TODO Date with remembering
    */
}
