use super::matches::Match;
use super::player::Player;
use super::game::Game;
use super::game::Duration;
use super::game::Race;

use crate::parser::command::GameArgs;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct League {
    pub(super) players: Vec<Player>,

    pub(super) matches: Vec<Match>,

    pub(super) weeks_count: usize,
}

impl League {
    pub fn new(names: &Vec<String>) -> Self {

        // Create league struct with mapped names
        let mut l = League {
            players: names.iter().map(|x| Player { name: x.clone() }).collect::<Vec<Player>>(),
            matches: Vec::new(),
            weeks_count: 0,
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
        l.weeks_count = weeks_num;


        l
    }

    pub fn is_consistent(&self) -> bool {
        let player_count = self.players.len();

        let weeks_num = player_count - 1 + player_count % 2;

        if self.weeks_count != weeks_num {
            return false;
        }

        if self.matches.len() != weeks_num * (weeks_num - 1) / 2 {
            return false;
        }

        true
    }

    pub fn add_game_raw(&mut self, idx: usize, win: bool, races: (Race, Race), duration : Duration) -> Result<(), String> {
        let m = &mut self.matches[idx];
        if m.winner().is_some() {
            return Err(String::from("Match is already finished"));
        }
        let g = Game { first_player_won: win, races, duration};
        m.games.push(g);
        Ok(())
    }

    pub fn add_game(&mut self, args : &GameArgs) -> Result<(), String>{
        let idx_finder = |n : &String| -> Option<usize> {
            for (i, p) in self.players.iter().enumerate() {
                if p.name == *n {
                    return Some(i);
                }
            }
            return None;
        };

        let i1 = match idx_finder(&args.player1.0) {
            Some(r) => r,
            None => return Err(String::from("Player not in the league"))
        };
        let i2 = match idx_finder(&args.player2.0) {
            Some(r) => r,
            None => return Err(String::from("Player not in the league"))
        };

        let match_idx_finder = |i1 :usize, i2: usize| -> Option<(usize,bool)> {
            for (i, m) in self.matches.iter().enumerate() {
                if m.players == (i1, i2) {
                    return Some((i,false));
                } else if m.players == (i2, i1) {
                    return Some((i, true));
                }
            }
            return None;
        };
        let idx = match match_idx_finder(i1, i2) {
            Some(r) => r,
            None => return Err(String::from("Match not found")) // actually should not be possible...
        };

        let race_converter = |r : char| -> Option<Race> {
            match r {
                'z' => Some(Race::Zerg),
                'p' => Some(Race::Protoss),
                't' => Some(Race::Terran),
                _ => None
            }
        };

        let r1 = match race_converter(args.player1.1) {
            Some(r) => r,
            None => return Err(String::from("Char is not a race code"))
        };
        let r2 = match race_converter(args.player2.1) {
            Some(r) => r,
            None => return Err(String::from("Char is not a race code"))
        };

        let win = args.first_player_win;
        self.add_game_raw(
            idx.0,
            (win && !idx.1) || (!win && idx.1),
            (r1, r2),
            Duration{ min: args.duration_min, sec: args.duration_sec })?;
        Ok(())
    }

    pub fn get_match(& self, idx : usize) -> Option<&Match> {
        self.matches.get(idx)
    }
}




