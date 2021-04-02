use crate::types::{Vec2i,Player};
use std::net::{TcpStream, Shutdown};
use std::io::{Read, Write, Stderr};
use std;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::states::GameState;
use std::collections::HashMap;

const BUFSIZE: usize = 4096;

pub struct Server {
    pub id: i32,
    sock: Option<TcpStream>,
    pub connected: bool,
}

impl Server {
    pub fn new() -> Server {
        Server {
            id: -1,
            connected: false,
            sock: None,
        }
    }

    pub fn connect(&mut self, addr: &str) {
        let addr = std::net::SocketAddr::from_str(addr).unwrap();
        let mut stream;
        match TcpStream::connect(addr) {
            Ok(s) => stream = s,
            Err(e) => {
                println!("Cannot connect to server ({}). Using offline mode", e);
                return;
            }
        }
        let mut buf: [u8; BUFSIZE] = [0; BUFSIZE]; // well memory is cheap
        stream.read(&mut buf).unwrap();
        let s = std::str::from_utf8(&buf).unwrap();
        let term = s.find("\n").unwrap();
        let id = s[..term].parse::<i32>().unwrap();
        self.id = id;
        self.connected = true;
        self.sock = Some(stream);
    }

    fn disconnect(&mut self) {
        let mut sock = self.sock.as_ref().unwrap();
        sock.write("{\"op\":\"disconnect\"}\n".as_bytes()).unwrap();
        sock.shutdown(Shutdown::Both).unwrap();
    }

    fn update(&self, player: &Player) -> Result<Vec<Player>, Box<dyn std::error::Error>> {
        if !self.connected {
            return Ok(Vec::<Player>::new()); // empty vec
        }
        let mut sock = self.sock.as_ref().unwrap();
        let obj = serde_json::json!({
            "op":"update",
            "data":player
        });
        let j = serde_json::to_string(&obj).unwrap() + "\n";
        sock.write(j.as_bytes())?;
        let mut buf: [u8; BUFSIZE] = [0; BUFSIZE];
        sock.read(&mut buf)?;
        let s = std::str::from_utf8(&mut buf)?;
        if let Some(term) = s.find("\n") {
            let v: Vec<Player> = serde_json::from_str(&s[..term])?;
            println!("Recved from server: {}", s);
            Ok(v)
        } else {
            Ok(Vec::<Player>::new())
            // this is not ok but i can't get rust to throw something sensible
        }
    }

    pub fn update_game(&self, players:&mut HashMap<i32,Player>){
        let response=self.update(&players[&self.id]);
        match response{
            Ok(others)=>{
                for o in others.into_iter(){
                    let player=players.entry(o.id).or_insert(Player::new());
                    player.world=o.world;
                    player.vel=o.vel;
                    player.pos=o.pos;
                    player.id=o.id;
                }
            },
            _=>{println!("Cannot update player")}
        }
    }
}


impl Drop for Server {
    // destructor
    fn drop(&mut self) {
        if self.connected {
            self.disconnect();
        }
    }
}


