use crate::types::Player;
use serde_json;
use std::io::{Write, Read};

use std::fs::OpenOptions;
use std::fs::File;

#[allow(unused_must_use)]
pub fn save(player: &Player) {
    let s = serde_json::to_string(player).unwrap();
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("save.json").unwrap();
    file.write(s.as_bytes());
    file.flush();
}

#[allow(unused_must_use)]
pub fn load() -> Player {
    let file = File::open("save.json");
    match file {
        Ok(mut f) => {
            let mut s = String::new();
            f.read_to_string(&mut s);
            return serde_json::from_str(s.as_str()).unwrap();
        }
        Err(_) => {
            return Player::new();
        }
    }
}