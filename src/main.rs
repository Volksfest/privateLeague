mod league;
mod parser;

use crate::league::league::League;
use crate::parser::command::LeagueCommand;

use actix_files as fs;

use clap::Clap;
use std::path::Path;
use std::cmp::Ordering;
use chrono::prelude::*;

use std::sync::Mutex;
use std::sync::Arc;

use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};

// TODO beautify
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
        builder.append(format!(r#"<div class="week{}"><div class="date">{:02}.{:02}<br>-<br>{:02}.{:02}</div><div class="week_matches">"#,
            if league.start_week + w as u32== todays_week {" highlighted"} else {""},
            begin_date.day(), begin_date.month(),
            end_date.day(), end_date.month()
        ));

        for i in 0..mpw {
            let match_index = w * mpw + i;
            let m = league.get_match(match_index).unwrap();
            let winner = match m.winner() {
                None => None,
                Some(t) => Some(t == m.get_first_player())
            };
            builder.append(format!(
               r#"<div id="match_{}" class="match_box {}">
                    <div class="player_box">
                        <div class="name_box {}">{}</div>
                        <div class="space_box">vs</div>
                        <div class="name_box {}">{}</div>
                    </div>
                  "#,
               match_index,
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


            builder.append(r#"<div class="games_box">"#);
            if !m.empty() {
                let p1 = m.get_first_player_data();
                let p2 = m.get_second_player_data();
                let duration = m.get_durations();
                for (dur, ((p1_won, p1_race), (p2_won, p2_race))) in duration.iter().zip(p1.iter().zip(p2.iter())) {
                    builder.append(format!(r#"<div class="game_box"><div class="race_box {}">{}</div><div class="time_box">{:02}:{:02}</div><div class="race_box {}">{}</div></div>"#,
                                           if *p1_won { "winner" } else { "loser" },
                                           p1_race.race_to_char(),
                                           dur.min, dur.sec,
                                           if *p2_won { "winner" } else { "loser" },
                                           p2_race.race_to_char()));
                }
            }
            builder.append(r#"</div></div>"#);

        }
        builder.append("</div></div>");
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
async fn single(data: web::Data<Arc<Mutex<(String,League)>>>) -> impl Responder {
    let g = data.lock().unwrap();
    HttpResponse::Ok()
        .body(create_html(&g.1))
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
