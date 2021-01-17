use super::matches::Match;
use super::player::Player;
use super::game::Game;
use super::game::Duration;
use super::game::Race;

use crate::parser::command::{AddGameArgs, RemoveGameArgs};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct League {
    pub(super) players: Vec<Player>,

    pub(super) matches: Vec<Match>,
}

impl League {
    pub fn new(names: &Vec<String>) -> Self {

        // Create league struct with mapped names
        let mut l = League {
            players: names.iter().map(|x| Player { name: x.clone() }).collect::<Vec<Player>>(),
            matches: Vec::new(),
        };

        // Create matches
        let (weeks_num,
            lonely_player)
            = match l.players.len() % 2 {
            1 => {
                (l.players.len(), None)
            }
            0 => {
                (l.players.len() - 1, Some(l.players.len() - 1))
            }
            _ => unreachable!()
        };

        for week in 0..weeks_num {
            for i in 1..((weeks_num - 1) / 2 + 1) {
                l.matches.push(
                    Match {
                        players:
                        (match i > week {
                            true => weeks_num - i + week,
                            false
                            => week - i
                        },
                         (week + i) % weeks_num),
                        games: Vec::new(),
                    }
                );
            }

            match lonely_player {
                Some(p) =>
                    l.matches.push(
                        Match {
                            players:
                            (p, week),
                            games: Vec::new(),
                        }
                    ),
                None => ()
            }
        }
        l
    }

    pub fn weeks_count(player_count : usize) -> usize {
        match player_count % 2 {
            0 => player_count - 1,
            1 => player_count,
            _ => unreachable!()
        }
    }

    pub fn match_count(player_count : usize) -> usize {
        player_count * (player_count - 1) / 2
    }

    pub fn mathes_per_week(player_count : usize) -> usize {
        League::match_count(player_count) / League::weeks_count(player_count)
    }

    pub fn is_consistent(&self) -> bool {
        let player_count = self.players.len();

        if League::match_count(player_count) != self.matches.len() {
            return false;
        }

        true
    }

    fn get_player_idx(&self, player : &String) -> Option<usize> {
        for (i, p) in self.players.iter().enumerate() {
            if p.name == *player {
                return Some(i);
            }
        }
        None
    }

    // bool is for inverting player one and player two
    fn get_match_idx(&self, player1 : &String, player2 : &String) -> Option<(usize, bool)> {
        let p1_idx = self.get_player_idx(player1)?;
        let p2_idx = self.get_player_idx(player2)?;

        for (i, m) in self.matches.iter().enumerate() {
            if m.players == (p1_idx, p2_idx) {
                return Some((i, false));
            } else if m.players == (p2_idx, p1_idx) {
                return Some((i, true));
            }
        }
        None
    }

    pub fn add_game(&mut self, args : &AddGameArgs) -> Result<(), String>{

        let (idx, invert) = match self.get_match_idx(&args.player1.0, &args.player2.0) {
            Some(r) => r,
            None => return Err(String::from("Match not found")) // actually should not be possible...
        };

        let r1 = match Race::char_to_race(args.player1.1) {
            Some(r) => r,
            None => return Err(String::from("Char is not a race code"))
        };

        let r2 = match Race::char_to_race(args.player2.1) {
            Some(r) => r,
            None => return Err(String::from("Char is not a race code"))
        };

        let win = args.first_player_win;
        self.add_game_raw(
            idx,
            (win && !invert) || (!win && invert),
            (r1, r2),
            Duration{ min: args.duration_min, sec: args.duration_sec })?;
        Ok(())
    }

    fn add_game_raw(&mut self, idx: usize, win: bool, races: (Race, Race), duration : Duration) -> Result<(), String> {
        let m = &mut self.matches[idx];
        if m.winner().is_some() {
            return Err(String::from("Match is already finished"));
        }
        let g = Game { first_player_won: win, races, duration};
        m.games.push(g);
        Ok(())
    }

    pub fn remove_game(&mut self, args : &RemoveGameArgs) -> Result<(), String>{
        let idx = match self.get_match_idx(&args.player1, &args.player2) {
            Some((id, invert)) => id,
            None => return Err(String::from("Match not found")),
        };

        self.matches[idx].games.clear();
        Ok(())
    }

    pub fn get_match(& self, idx : usize) -> Option<&Match> {
        self.matches.get(idx)
    }
}
