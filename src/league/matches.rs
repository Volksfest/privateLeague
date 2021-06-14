use super::game::Game;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Match {
    pub(super) players: (usize, usize),
    pub(super) games: Vec<Game>,
}

impl Match {
    pub fn empty(&self) -> bool {
        self.games.len()==0
    }

    pub fn get_games(&self) -> &Vec<Game> {
        &self.games
    }

    pub fn winner(&self) -> Option<usize> {
        let mut a = 0;
        let mut b = 0;
        for i in 0..3 {
            match self.games.get(i) {
                None => break,
                Some(s) => match s.players[0].win {
                    true => a += 1,
                    false => b += 1
                }
            }
        }

        if a == 2 {
            return Some(self.players.0);
        }
        if b == 2 {
            return Some(self.players.1);
        }
        None
    }

    pub fn get_first_player(&self) -> usize {
        self.players.0
    }

    pub fn get_second_player(&self) -> usize {
        self.players.1
    }
}