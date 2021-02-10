use super::game::Game;
use super::game::Race;
use super::game::Duration;
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

    pub fn get_first_player(&self) -> usize {
        self.players.0
    }

    pub fn get_first_player_data(&self) -> Vec<(bool, Race)> {
        let mut stats = Vec::new();
        for g in &self.games {
            stats.push((g.first_player_won, g.races.0.clone()));
        }

        stats
    }

    pub fn get_second_player(&self) -> usize {
        self.players.1
    }

    pub fn get_second_player_data(&self) -> Vec<(bool, Race)> {
        let mut stats = Vec::new();
        for g in &self.games {
            stats.push((!g.first_player_won, g.races.1.clone()));
        }

        stats
    }

    pub fn get_durations(&self) -> Vec<Duration> {
        let mut stats = Vec::new();
        for g in &self.games {
            stats.push(g.duration.clone());
        }
        stats
    }
}