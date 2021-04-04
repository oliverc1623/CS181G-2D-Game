use crate::tiles::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Player {
    pub id: i32,
    pub pos: Vec2i,
    pub vel: Vec2i,
    pub world: i32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            id: -1,
            pos: Vec2i(0, 0),
            vel: Vec2i(0, 0),
            world: 0,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: u16,
    pub h: u16,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Deserialize, Serialize)]
pub struct Vec2i(pub i32, pub i32);

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct Rgba(pub u8, pub u8, pub u8, pub u8);

// Feel free to add impl blocks with convenience functions
