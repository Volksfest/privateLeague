use std::cmp::Ordering;
use chrono::prelude::*;
use horrorshow::prelude::*;

use crate::league::league::League;

pub fn create_single_match( l: &League, idx: usize) -> impl Render + '_{
    let m = &l.matches[idx]; // TODO get out the league
    let games = m.get_games();

    horrorshow::owned_html! {
        div(
            id = format!("match_{}",idx),
            class = format!("match_box {}",
                match m.winner() {
                    Some(_) => "played",
                    None => if m.empty() {"unplayed"} else {"ongoing"}})
        ) {
            div (class = "player_box") {
                // TODO change normal_player to winner or loser
                div (class = "name_box normal_player") : l.players[m.get_first_player()].get_name().clone();
                div (class = "space_box") : "vs";
                div (class = "name_box normal_player") : l.players[m.get_second_player()].get_name().clone();
            }

            div (class = "games_box") {
                @ for g in games {
                    div (class = "game_box") {
                        div (class = format!("race_box {}", if g.players[0].win{"winner"} else {"loser"})) : g.players[0].race.race_to_char();
                        div (class = "time_box") : format!("{:02}:{:02}", g.duration.min, g.duration.sec);
                        div (class = format!("race_box {}", if g.players[1].win{"winner"} else {"loser"})) : g.players[1].race.race_to_char();
                    }
                }
            }
        }
   }
}

fn create_matches(league : &League) -> impl Render + '_ {

    let todays_week = Local::today().naive_local().iso_week().week();
    let mpw = league.matches_per_week();

    horrorshow::owned_html! {
        div (id = "match") {
            @ for w in 0..league.weeks_count() {
                div (class = if league.start_week + w as u32 == todays_week {"week highlighted"} else {"week"}) {
                    div (class ="date") {
                        | tmpl | {
                            let begin_date = NaiveDate::from_isoywd(Local::today().year(), league.start_week + w as u32, Weekday::Mon);
                            let end_date = NaiveDate::from_isoywd(Local::today().year(), league.start_week + w as u32, Weekday::Sun);
                            tmpl << Raw(format!(
                                "{:02}.{:02}<br>-<br>{:02}.{:02}",
                                begin_date.day(), begin_date.month(),
                                end_date.day(), end_date.month()
                            ));
                        }
                    }
                    div (class = "week_matches") {
                        @ for i in 0..mpw {
                            | tmpl | {
                                tmpl << create_single_match(league, w * mpw + i);
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn create_table(league : &League) -> impl Render + '_ {
    let score = league.get_score();
    let mut sorted_score = score.clone();
    sorted_score.sort_by(|a,b|
        match a.1.cmp(&b.1) {
            Ordering::Equal => a.2.cmp(&b.2),
            Ordering::Less => Ordering::Greater,
            Ordering::Greater => Ordering::Less
        }
    );

    horrorshow::owned_html! {
        div (id = "player") {
            div (class ="row") {
                div (class = "col1") : "#";
                div (class = "col2") : "Name";
                div (class = "col3") : "Sp";
                div (class = "col4") : "S";
                div (class = "col5") : "N";
            }
            @ for (i,r) in sorted_score.iter().enumerate() {
                div (class = if i == score.len()-1 {"row loser_back"} else if i == 0 {"row winner_back"} else {"row"})
                {
                    div (class = "col1") : i+1;
                    div (class = "col2") : r.0.clone();
                    div (class = "col3") : r.1 + r.2;
                    div (class = "col4") : r.1;
                    div (class = "col5") : r.2;
                }
            }
        }
    }
}

fn create_content(league : &League) -> impl Render + '_ {
    horrorshow::owned_html! {
        div (id = "container") {
            h2 : "StarCraft 2 Private League - Season 2";
            div (id = "output") {
                | tmpl | {
                    tmpl << create_table(&league);
                }
                | tmpl | {
                    tmpl << create_matches(&league);
                }
            }
        }
    }
}

fn create_header() -> impl Render {
    horrorshow::owned_html! {
        head {
            meta (charset = "UTF-8");
            title : "Hi tester";
            link (rel = "stylesheet", href="resource/style.css");
            link (rel = "icon", href="resource/favicon.ico", type="image/x-icon");
        }
    }
}


pub fn create_html(league : &League) -> impl Render + '_ {
    horrorshow::owned_html! {
        : horrorshow::helper::doctype::HTML;
        html {
            | tmpl | { tmpl << create_header();}
            body {
                | tmpl | {tmpl << create_content(&league);}
                //script (src = "resource/script.js"); //TODO change first
            }
        }
    }
}
