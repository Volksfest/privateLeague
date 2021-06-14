use super::matches::Match;
use super::matches::Winner;
use super::player::Player;
use super::game::Game;

use crate::com::command::RemoveGameArgs;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct League {
    pub players: Vec<Player>,

    pub matches: Vec<Match>,

    pub start_week: u32,
}

impl League {
    pub fn new(names: &Vec<String>, start_week : u32) -> Self {

        // Create league struct with mapped names
        let mut l = League {
            players: names.iter().map(|x| Player { name: x.clone() }).collect::<Vec<Player>>(),
            matches: Vec::new(),
            start_week
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

    pub fn weeks_count_static(player_count : usize) -> usize {
        match player_count % 2 {
            0 => player_count - 1,
            1 => player_count,
            _ => unreachable!()
        }
    }

    pub fn weeks_count(&self) -> usize {
        League::weeks_count_static(self.players.len())
    }

    pub fn match_count_static(player_count : usize) -> usize {
        player_count * (player_count - 1) / 2
    }

    pub fn match_count(&self) -> usize {
        League::match_count_static(self.players.len())
    }

    pub fn matches_per_week_static(player_count : usize) -> usize {
        League::match_count_static(player_count) / League::weeks_count_static(player_count)
    }

    pub fn matches_per_week(&self) -> usize {
        League::matches_per_week_static(self.players.len())
    }

    pub fn is_consistent(&self) -> bool {

        if self.match_count() != self.matches.len() {
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
    pub fn get_match_idx(&self, player1 : &String, player2 : &String) -> Option<(usize, bool)> {
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

    pub fn add_game(&mut self, game : &Game) -> Result<(), String>{
        if game.players.len() != 2 {
            return Err(String::from("Wrong number of players in the game"));
        }

        let (idx,_) = match self.get_match_idx(
            &game.players[0].name,
            &game.players[1].name
        ) {
            Some(r) => r,
            None => return Err(String::from("Match with the given players not found"))
        };

        let m = &mut self.matches[idx];
        if m.winner().is_finished() {
            return Err(String::from("Match is already finished"));
        }
        m.games.push(game.clone());
        Ok(())
    }

    pub fn remove_game(&mut self, args : &RemoveGameArgs) -> Result<(), String>{
        let idx = match self.get_match_idx(&args.player1, &args.player2) {
            Some((id, _)) => id,
            None => return Err(String::from("Match not found")),
        };

        self.matches[idx].games.clear();
        Ok(())
    }

    pub fn get_score(& self) -> Vec<(String, usize, usize)> {
        // TODO make a Score class
        let mut score:Vec<(String, usize,usize)> = Vec::new();
        for i in &self.players {
            score.push((i.name.clone(), 0, 0));
        }

        for m in &self.matches {
            match m.winner() {
                Winner::FirstPlayer => {
                    let w = m.players.0;
                    score[w].1 = score[w].1 + 1;
                    let l = match m.players.0 == w {
                        true => m.players.1,
                        false => m.players.0
                    };
                    score[l].2 = score[l].2 + 1;
                },
                Winner::SecondPlayer => {
                    let w = m.players.1;
                    score[w].1 = score[w].1 + 1;
                    let l = match m.players.0 == w {
                        true => m.players.1,
                        false => m.players.0
                    };
                    score[l].2 = score[l].2 + 1;
                },
                Winner::None => continue
            };
        }

        score
    }

    pub fn migrate_match(&mut self, mig : Vec<Game>, names : (&String, &String)) -> bool {
        let switched = (names.1,names.0);
        let mut idx = None;
        for (i, m) in self.matches.iter().enumerate() {
            let mn = (&self.players[m.players.0].name, &self.players[m.players.1].name);
            if mn == names {
                idx = Some((i, false));
                break;
            }

            if mn == switched {
                idx = Some((i, true));
                break;
            }
        }
        match idx{
            None => {return false;}
            Some((i,switch)) => {
                self.matches[i].games = mig;
                if switch {
                    let p = self.matches[i].players;
                    self.matches[i].players = (p.1,p.0);
                }
                true
            }
        }
    }

    pub fn kick(self, to_kick : Vec<String>) -> League {
        let names = self.players.iter().filter(|&x| !to_kick.contains(&x.name) ).map(|x| x.name.clone()).collect::<Vec<String>>();
        let mut new_league = League::new(&names, self.start_week);

        for i in self.matches {
            let (a,b) = i.players;
            new_league.migrate_match(i.games, (&self.players[a].name, &self.players[b].name));
        }

        new_league
    }

    #[allow(dead_code)]
    pub fn get_match(& self, idx : usize) -> Option<&Match> {
        self.matches.get(idx)
    }
}
