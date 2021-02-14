use std::cmp::Ordering;
use chrono::prelude::*;

use crate::league::league::League;
use crate::league::matches::Match;

// TODO seriously, either find or write a HTML Generator....

// TODO change dependencies from Math, weird Vec to only League
pub fn create_single_match(m : &Match, score : &Vec<(String, usize, usize)>, idx : usize) -> String {
    let mut builder = string_builder::Builder::default();

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
        idx,
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


    builder.append(r#"  <div class="games_box">"#);
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
    builder.append(r#"
    </div>
</div>"#);

    builder.string().unwrap()
}

fn create_header() -> String {
    String::from(r#"
    <head>
        <meta charset="UTF-8">
        <title>StartCraft 2 Private League</title>
        <link rel="stylesheet" href="resource/style.css" />
    </head>
    "#)
}

fn create_table(score : &Vec<(String, usize, usize)>) -> String {
    let mut builder = string_builder::Builder::default();

    builder.append(r#"<div id="player">"#);

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

    builder.append("</div>");

    builder.string().unwrap()
}

fn create_matches(league : &League, score : &Vec<(String, usize, usize)>) -> String {
    let mut builder = string_builder::Builder::default();

    builder.append(r#"<div id="match">"#);
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


            builder.append(create_single_match(m, score, match_index));
        }
        builder.append("</div></div>");
    }

    builder.append("</div></div>");

    builder.string().unwrap()
}

fn create_content(league: &League) -> String {
    let mut builder = string_builder::Builder::default();

    builder.append(r#"
    <div id="container">
        <h2>StartCraft 2 Private League - Season 2</h2>

        <div id="output">
            "#);

    // Player
    let score = league.get_score();
    builder.append(create_table(&score));

    // Matches

    builder.append(create_matches(&league, &score));


    builder.append(r#"</div>"#);

    builder.string().unwrap()
}

fn create_popup() -> String {
    String::from(r#"<div id="popup" class="hidden">
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
    "#)
}

pub fn create_html(league : &League) -> String {
    let mut builder = string_builder::Builder::default();


    builder.append(create_header());

    builder.append("<body>");

    builder.append(create_content(&league));

    builder.append(create_popup());

    builder.append(r#"
            <script src="/resource/script.js"></script>
        </body>"#);


    builder.string().unwrap()
}