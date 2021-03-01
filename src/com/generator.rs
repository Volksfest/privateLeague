use std::cmp::Ordering;
use chrono::prelude::*;

use crate::league::league::League;
use crate::com::tree::Tree;

// TODO make the code more generetive and less repititive...

pub fn create_single_match(league: &League, idx : usize) -> Tree {

    let m = league.get_match(idx).unwrap();

    let tree = Tree::new("div").insert_attribute("id",format!("match_{}", idx));

    let tree = tree.insert_attribute("class",
                          format!("match_box {}",
                                    match m.winner() {
                                        Some(_) => "played",
                                        None =>
                                            if m.empty() {
                                                "unplayed"
                                            } else {
                                                "ongoing"
                                            }
                                    }));

    let players = &league.players;

    let tree = tree.insert_tree(
        Tree::new("div")
            .insert_attribute("class","player_box")
            .insert_tree(
                Tree::new("div")
                    .insert_attribute("class", format!("name_box {}", "normal_player"))
                    .insert_text(players[m.get_first_player()].get_name())
            )
            .insert_tree(Tree::new("div").insert_attribute("class","space_box").insert_text("vs"))
            .insert_tree(
                Tree::new("div")
                    .insert_attribute("class", format!("name_box {}", "normal_player"))
                    .insert_text(players[m.get_second_player()].get_name())
            )
    );


    let mut game_box = Tree::new("div").insert_attribute("class","games_box");

    if !m.empty() {
        let p1 = m.get_first_player_data();
        let p2 = m.get_second_player_data();
        let duration = m.get_durations();
        for (dur, ((p1_won, p1_race), (p2_won, p2_race))) in duration.iter().zip(p1.iter().zip(p2.iter())) {
            game_box = game_box.insert_tree(
                Tree::new("div").insert_attribute("class","game_box")
                    .insert_tree(
                        Tree::new("div")
                            .insert_attribute("class",format!("race_box {}", if *p1_won{ "winner" } else { "loser" }))
                            .insert_text(p1_race.race_to_char())
                    )
                    .insert_tree(
                        Tree::new("div").insert_attribute("class","time_box")
                            .insert_text(format!("{:02}:{:02}", dur.min, dur.sec))

                    )
                    .insert_tree(
                        Tree::new("div")
                            .insert_attribute("class",format!("race_box {}", if *p2_won{ "winner" } else { "loser" }))
                            .insert_text(p2_race.race_to_char())
                    )
            );
        }
    }

    tree.insert_tree(game_box)
}

fn create_matches(league : &League) -> Tree {

    let mut all_matches = Tree::new("div").insert_attribute("id", "match");

    let todays_week = Local::today().naive_local().iso_week().week();
    let mpw = league.matches_per_week();

    for w in 0..league.weeks_count() {
        let begin_date = NaiveDate::from_isoywd(Local::today().year(), league.start_week + w as u32, Weekday::Mon);
        let end_date = NaiveDate::from_isoywd(Local::today().year(), league.start_week + w as u32, Weekday::Sun);

        let week = Tree::new("div").insert_attribute("class",
            if league.start_week + w as u32 == todays_week {
                "week highlighted"
            } else {
                "week"
            }
        );

        let week = week.insert_tree(
            Tree::new("div")
                .insert_attribute("class","date")
                .insert_text(format!("{:02}.{:02}",
                    begin_date.day(),
                    begin_date.month()))
                .insert_tree(Tree::new("br"))
                .insert_text("-")
                .insert_tree(Tree::new("br"))
                .insert_text(format!("{:02}.{:02}",
                    end_date.day(),
                    end_date.month()))
        );

        let mut weekly_matches = Tree::new("div").insert_attribute("class", "week_matches");

        for i in 0..mpw {
            let match_index = w * mpw + i;

            weekly_matches = weekly_matches.insert_tree(
                create_single_match(league, match_index)
            );
        }
        let week = week.insert_tree(weekly_matches);
        all_matches = all_matches.insert_tree(week);
    }

    all_matches
}


pub fn create_table(league : &League) -> Tree {

    let score = league.get_score();

    let mut player =
        Tree::new("div").insert_attribute("id","player")
            .insert_tree(
                Tree::new("div").insert_attribute("class","row")
                    .insert_tree(
                        Tree::new("div")
                            .insert_attribute("class","col1")
                            .insert_text("#")
                    )
                    .insert_tree(
                        Tree::new("div")
                            .insert_attribute("class","col2")
                            .insert_text("Name")
                    )
                    .insert_tree(
                        Tree::new("div")
                            .insert_attribute("class","col3")
                            .insert_text("Sp")
                    )
                    .insert_tree(
                        Tree::new("div")
                            .insert_attribute("class","col4")
                            .insert_text("S")
                    )
                    .insert_tree(
                        Tree::new("div")
                            .insert_attribute("class","col5")
                            .insert_text("N")
                    )
            );

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

        player = player.insert_tree(
            Tree::new("div").insert_attribute("class",class)
                .insert_tree(
                    Tree::new("div")
                        .insert_attribute("class", "col1")
                        .insert_text(i+1)
                )
                .insert_tree(
                    Tree::new("div")
                        .insert_attribute("class", "col2")
                        .insert_text(&r.0)
                )
                .insert_tree(
                    Tree::new("div")
                        .insert_attribute("class", "col3")
                        .insert_text(r.1 + r.2)
                )
                .insert_tree(
                    Tree::new("div")
                        .insert_attribute("class", "col4")
                        .insert_text(r.1)
                )
                .insert_tree(
                    Tree::new("div")
                        .insert_attribute("class", "col5")
                        .insert_text(r.2)
                )
        );
    }

    player
}

fn create_content(league: &League) -> Tree {
    Tree::new("div")
        .insert_attribute("id","container")
        .insert_tree(
            Tree::new("h2")
                .insert_text("StarCraft 2 Private League - Season 2")
        )
        .insert_tree(
            Tree::new("div")
                .insert_attribute("id", "output")
                .insert_tree(create_table(&league)) // Player
                .insert_tree(create_matches(&league)) // Matches
        )
}

fn create_popup() -> Tree {
    Tree::new("div")
        .insert_attribute("id", "popup")
        .insert_attribute("class", "hidden")
        .insert_tree(
            Tree::new("datalist")
                .insert_attribute("id", "race")
                .insert_tree(Tree::new("option").insert_attribute("value","Terran"))
                .insert_tree(Tree::new("option").insert_attribute("value","Zerg"))
                .insert_tree(Tree::new("option").insert_attribute("value","Protoss"))
        )
        .insert_tree(
            Tree::new("div")
                .insert_attribute("class","row")
                .insert_tree(
                    Tree::new("label")
                        .insert_attribute("id","first_player_label")
                        .insert_attribute("for", "first_player_race_input")
                )
                .insert_tree(
                    Tree::new("input")
                        .insert_attribute("id", "first_player_race_input")
                        .insert_attribute("name","first_player_race")
                        .insert_attribute("list","race")
                        .insert_attribute("onfocus","this.value=''")
                )
                .insert_tree(
                    Tree::new("input")
                        .insert_attribute("id", "first_player_win_radio")
                        .insert_attribute("name","first_player_won")
                        .insert_attribute("type","radio")
                        .insert_attribute("value","true")
                        .insert_key("checked")
                )
        )
        .insert_tree(
            Tree::new("div")
                .insert_attribute("class","row")
                .insert_tree(
                    Tree::new("label")
                        .insert_attribute("id","second_player_label")
                        .insert_attribute("for", "second_player_race_input")
                )
                .insert_tree(
                    Tree::new("input")
                        .insert_attribute("id", "second_player_race_input")
                        .insert_attribute("name","second_player_race")
                        .insert_attribute("list","race")
                        .insert_attribute("onfocus","this.value=''")
                )
                .insert_tree(
                    Tree::new("input")
                        .insert_attribute("id", "second_player_win_radio")
                        .insert_attribute("name","first_player_won")
                        .insert_attribute("type","radio")
                        .insert_attribute("value","true")
                )
        )
        .insert_tree(
            Tree::new("div")
                .insert_attribute("class","row")
                .insert_tree(
                    Tree::new("label")
                        .insert_attribute("id","duration")
                        .insert_attribute("for", "duration_text")
                        .insert_text("Spieldauer")
                )
                .insert_tree(
                    Tree::new("input")
                        .insert_attribute("id", "duration_text")
                        .insert_attribute("name","duration")
                        .insert_attribute("type","text")
                        .insert_attribute("value","0:00")
                )
        )
        .insert_tree(
            Tree::new("div")
                .insert_attribute("class","row")
                .insert_tree(
                    Tree::new("div")
                        .insert_attribute("id","error_output")
                )
        )
        .insert_tree(
            Tree::new("div")
                .insert_attribute("class","row")
                .insert_tree(
                    Tree::new("input")
                        .insert_attribute("id", "submit")
                        .insert_attribute("name","game_submit")
                        .insert_attribute("type","button")
                        .insert_attribute("value","Add")
                        .insert_attribute("onclick","addGame()")
                )
                .insert_tree(
                    Tree::new("input")
                        .insert_attribute("id", "cancel")
                        .insert_attribute("name","game_cancel")
                        .insert_attribute("type","button")
                        .insert_attribute("value","Cancel")
                        .insert_attribute("onclick","hidePopup()")
                )
        )
}

fn create_header() -> Tree {
    Tree::new("head")
        .insert_tree(
            Tree::new("meta")
                .insert_attribute("charset", "UTF-8"))

        .insert_tree(
            Tree::new("link")
        )

        .insert_tree(
            Tree::new("title")
                .insert_text("StarCraft 2 Private League"))

        .insert_tree(
            Tree::new("link")
                .insert_attribute("rel", "stylesheet")
                .insert_attribute("href", "resource/style.css")
        )

        .insert_tree(
            Tree::new("link")
                .insert_attribute("rel", "icon")
                .insert_attribute("href", "resource/favicon.ico")
                .insert_attribute("type", "image/x-icon")
        )
}

pub fn create_html(league : &League) -> String {

    let html = Tree::new("html");

    html.insert_tree(create_header())
        .insert_tree(
        Tree::new("body")
            .insert_tree(create_content(&league))
            .insert_tree(create_popup())
            .insert_tree(
                Tree::new("script")
                    .insert_attribute("src", "/resource/script.js")
            )
        )
        .print()
}