use crate::league::league::League;

use std::fs;

pub fn save_state(state : &League, path : &String) {
    let json = serde_json::to_string_pretty(state).unwrap();
    fs::write(path, json).expect("Could not write serialize");
}