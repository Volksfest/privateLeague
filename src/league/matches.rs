use super::game::Game;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Match {
    pub(super) players: (usize, usize),
    pub(super) games: Vec<Game>,
}

impl Match {
    pub fn winner(&self) -> Option<usize> {
        let mut a = 0;
        let mut b = 0;
        for i in 0..3 {
            match self.games.get(i) {
                None => break,
                Some(s) => match s.first_player_won {
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
}