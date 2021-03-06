use crate::animation::*;
use crate::collision::*;
use crate::graphics::*;
use crate::resources::*;
use crate::server::Server;
use crate::texture::*;
use crate::tiles::*;
use crate::types::*;
use imageproc::drawing::draw_text;
use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage, Rgb, RgbaImage};
use rusttype::Font;
use std::collections::HashMap;
use std::rc::Rc;
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

const WIDTH: usize = 320 * 2;
const HEIGHT: usize = 240 * 2;
const TILE_MAP_SIZE: usize = 256;
const TILE_SZ: usize = 32;

pub struct GameData {
    pub score: usize,
    pub speed_multiplier: usize,
    pub num_jumps: usize,
    pub portals: Vec<(Vec2i, Vec2i)>,
    pub restart: bool,
}

pub struct Text {
    pub from: Rect,
    pub to: Vec2i,
    pub tex: Texture,
}

impl Text {
    pub fn new(map_to: Vec2i, text: &str, font: &Font, scale: f32) -> Self {
        let mut img: RgbaImage = ImageBuffer::new(512, 512);
        let sc = rusttype::Scale { x: scale, y: scale };
        let text_img = draw_text(&mut img, image::Rgba([0, 0, 0, 255]), 0, 0, sc, font, text);
        let tex = Texture::new(text_img);
        Text {
            tex,
            from: Rect { x: 0, y: 0, w: 512, h: 512 },
            to: map_to,
        }
    }
}

pub struct GameState {
    // Every entity has a position, a size, a texture, and animation state.
    // Assume entity 0 is the player
    // pub types: Vec<Player>,
    // pub positions: Vec<Vec2i>,
    // pub velocities: Vec<Vec2i>,
    // pub current_player: Player,
    pub server: Server,
    pub players: HashMap<i32, Player>,
    // pub positions: Vec<Vec2i>,
    pub sizes: Vec<(usize, usize)>,
    pub textures: Vec<Rc<Texture>>,
    pub anim_state: Vec<AnimationState>,
    // Current level
    pub level: usize,
    // Camera position
    pub camera: Vec2i,
    // background position
    pub background_pos: Vec2i,
    pub state_stack: Vec<Box<dyn State>>,
    pub game_data: GameData,
    pub map_x_boundary: i32,
    pub map_y_boundary: i32,
    pub tt_tileset: Rc<Tileset>,
    pub maps: Vec<Tilemap>,
    pub side_map: Vec<Tilemap>,
    // pub font: Font<'static>,
    pub game: usize,
    pub spawn_point: Vec2i,
    pub texts_overworld: Vec<Text>,
    pub texts_sidescroll: Vec<Text>,
}

#[derive(Debug)]
pub enum StateResult {
    // Pop this state off the stack, update the one before me
    Remove,
    // Keep this state as is, quit propagating updates
    Keep,
    // Swap this state for a new one, update the new one too
    Swap(Box<dyn State>),
    // Push a new state on top of this one, update it too
    Push(Box<dyn State>),
}

pub trait State: std::fmt::Debug {
    fn update(
        &mut self,
        game: &mut GameState,
        // input: &Input,
        resources: &Resources,
        levels: &Vec<Level>,
        frame: usize,
        key_input: &WinitInputHelper,
    ) -> StateResult;
    // Probably needs to take an &Screen, could return a bool if it
    // wants other states to display too
    fn display(
        &self,
        game: &GameState,
        resources: &Resources,
        levels: &Vec<Level>,
        screen: &mut Screen,
        frame: usize,
    );
}

/*
Currently using sketch level timemap
*/
#[derive(Debug)]
pub struct Title(); // overworld map

#[allow(unused_variables)]
impl State for Title {
    fn update(
        &mut self,
        _game: &mut GameState,
        // input: &Input,
        resources: &Resources,
        levels: &Vec<Level>,
        frame: usize,
        key_input: &WinitInputHelper,
    ) -> StateResult {
        // _game.positions[0] = Vec2i(levels[1].1[0].1 * 16, levels[1].1[0].2 * 16);
        let cur_player = _game.players.get_mut(&_game.server.id).unwrap();
        if key_input.key_held(VirtualKeyCode::Right) {
            cur_player.pos.0 += 5;
            if cur_player.pos.0 > (TILE_MAP_SIZE - TILE_SZ) as i32 {
                _game.camera.0 += 5;
            }
            // generate tile map
            if _game.camera.0 >= (_game.map_x_boundary - WIDTH as i32) {
                let mut i: i32 = 0;
                let mut psn: Vec2i = Vec2i(_game.map_x_boundary, 0);
                while i < _game.map_y_boundary {
                    _game.maps.push(Tileset::create_map(&_game.tt_tileset, psn, _game.game));
                    psn.1 += TILE_MAP_SIZE as i32;
                    i += TILE_MAP_SIZE as i32;
                }
                _game.map_x_boundary += TILE_MAP_SIZE as i32;
            }
        }
        if key_input.key_held(VirtualKeyCode::Left) {
            if cur_player.pos.0 > 0 {
                cur_player.pos.0 += -7;
            }
            if _game.camera.0 > 0 {
                _game.camera.0 -= 7;
            }
        }
        if key_input.key_held(VirtualKeyCode::Up) {
            if cur_player.pos.1 > 0 {
                cur_player.pos.1 += -7;
            }
            if _game.camera.1 > 0 {
                _game.camera.1 -= 7;
            }
        }
        if key_input.key_held(VirtualKeyCode::Down) {
            cur_player.pos.1 += 7;
            if cur_player.pos.1 > (TILE_MAP_SIZE - TILE_SZ) as i32 {
                _game.camera.1 += 7;
            }
            if _game.camera.1 >= (_game.map_y_boundary - HEIGHT as i32) {
                let mut i: i32 = 0;
                let mut psn: Vec2i = Vec2i(0, _game.map_y_boundary);
                while i < _game.map_x_boundary {
                    _game.maps.push(Tileset::create_map(&_game.tt_tileset, psn, _game.game));
                    psn.0 += TILE_MAP_SIZE as i32;
                    i += TILE_MAP_SIZE as i32;
                }
                _game.map_y_boundary += TILE_MAP_SIZE as i32;
            }
        }

        let mut all_pos: Vec<Vec2i> = vec![cur_player.pos];
        let mut all_vel: Vec<Vec2i> = vec![cur_player.vel];
        // reset number of jumps
        // Detect collisions: Convert positions and sizes to collision bodies, generate contacts
        // Outline of a possible approach to tile collision:
        let mut contacts = vec![];
        gather_contacts(
            all_pos.as_slice(),
            &_game.sizes,
            &_game.maps,
            &mut contacts,
            &mut _game.game_data.num_jumps,
            &mut _game.game_data.restart,
        );
        restitute(
            all_pos.as_mut_slice(),
            &_game.sizes,
            all_vel.as_mut_slice(),
            &mut _game.camera,
            &_game.maps,
            &mut contacts,
        );
        cur_player.pos = all_pos[0];
        cur_player.vel = all_vel[0];

        // _game.camera = cur_player.pos;

        _game.server.update_players(&mut _game.players);

        if key_input.key_held(VirtualKeyCode::P) {
            // println!("hitting p");
            _game.players.get_mut(&_game.server.id).unwrap().vel = Vec2i(0, 0);
            _game.players.get_mut(&_game.server.id).unwrap().world = 1;
            if _game.game == 2 {
                _game.players.get_mut(&_game.server.id).unwrap().pos.0 = 3 * 32;
                _game.players.get_mut(&_game.server.id).unwrap().pos.1 = 37 * 32;
            } else {
                _game.players.get_mut(&_game.server.id).unwrap().pos = Vec2i(50, 50);
            }
            StateResult::Swap(Box::new(Scroll()))
        } else {
            StateResult::Keep
        }
    }
    #[allow(unused_variables)]
    fn display(
        &self,
        _game: &GameState,
        resources: &Resources,
        levels: &Vec<Level>,
        screen: &mut Screen,
        frame: usize,
    ) {
        // println!("Title: p to play");
        screen.clear(Rgba(211, 211, 211, 255));
        screen.set_scroll(_game.camera);
        // levels[_game.level].0.draw(screen);
        // let maps = &levels[1].0;
        for map in _game.maps.iter() {
            map.draw(screen);
        }
        // draw main player
        // let curpos = _game.players[&_game.server.id].pos;
        // println!("player pos {:?}", &_game.textures[1].image);
        // screen.bitblt(&_game.textures[0], _game.anim_state[0].frame(), curpos);
        // draw positions of other players
        let cur_world = _game.players[&_game.server.id].world;
        for player in _game.players.iter()
            .filter(|p| p.1.world == cur_world)
        {
            // println!("drawing character {}", player.0);
            screen.bitblt(
                &_game.textures[0],
                _game.anim_state[0].frame(),
                player.1.pos,
            );
        }
        for text_img in &_game.texts_overworld {
            screen.bitblt(
                &text_img.tex,
                text_img.from,
                text_img.to,
            );
        }
    }
}

#[derive(Debug)]
pub struct Scroll();

// side scroller
#[allow(unused_variables)]
impl State for Scroll {
    fn update(
        &mut self,
        _game: &mut GameState,
        resources: &Resources,
        levels: &Vec<Level>,
        frame: usize,
        key_input: &WinitInputHelper,
    ) -> StateResult {
        _game.level = 0;
        let cur_player = _game.players.get_mut(&_game.server.id).unwrap();
        // StateResult::Keep
        let max_vel = 10;
        let min_vel = -10;
        let mut held_up = false;
        let mut vert_moving: bool = false;
        let mut horiz_moving = false;
        // println!("before {:}", _game.velocities[0].0);
        if key_input.key_held(VirtualKeyCode::Right) {
            // _game.velocities[0].0 = 7;
            // println!("right vel {:}", _game.velocities[0].0);
            if cur_player.vel.0 < max_vel {
                // println!("inside");
                cur_player.vel.0 += 2;
                // println!("inside {:}", _game.velocities[0].0);
            }
            horiz_moving = true;
            _game.anim_state[0].tick();
        } else if key_input.key_released(VirtualKeyCode::Right) {
            //cur_player.vel.0 = (cur_player.vel.0 as f32 * 0.25) as i32; 
            // println!("after {:}", _game.velocities[0].0);
        }
        if key_input.key_held(VirtualKeyCode::Left) {
            if cur_player.vel.0 > min_vel {
                // println!("inside");
                cur_player.vel.0 -= 2;
                // println!("inside {:}", _game.velocities[0].0);
            }
            horiz_moving = true;
        } else if key_input.key_released(VirtualKeyCode::Left) {
            //cur_player.vel.0 = (cur_player.vel.0 as f32 * 0.25) as i32;
        }
        if key_input.key_held(VirtualKeyCode::Up) {
            if _game.game_data.num_jumps < 2 && cur_player.vel.1 >= 0 && cur_player.vel.1 >= min_vel {
                cur_player.vel.1 = -18;
                _game.game_data.num_jumps += 1;
            } else if _game.game_data.num_jumps < 2 && cur_player.vel.1 < 0 {
                cur_player.vel.1 += -6;
            }
            vert_moving = true;
            held_up = true;
            if cur_player.vel.1 == 0 {
                held_up = false;
            }
        } else if key_input.key_released(VirtualKeyCode::Up) {}
        if key_input.key_held(VirtualKeyCode::Down) {
            if cur_player.vel.1 < max_vel {
                cur_player.vel.1 += 2;
                vert_moving = true;
                // println!("inside {:}", _game.velocities[0].0);
            }
        } else if key_input.key_released(VirtualKeyCode::Down) {
            // cur_player.vel.1 = (cur_player.vel.1 as f32 * 0.25) as i32;
        }

        if !vert_moving {
            cur_player.vel.1 = (cur_player.vel.1 as f32 * 0.5) as i32;
        }
        if !horiz_moving {
            cur_player.vel.0 = (cur_player.vel.0 as f32 * 0.5) as i32;
        }

        //if player is not holding the up key, start falling. 
        if !held_up {
            cur_player.vel.1 = 4;
        }
        // Update all entities' positions
        // update current player
        // println!("{}", cur_player.vel.1);
        cur_player.pos.0 += cur_player.vel.0;
        cur_player.pos.1 += cur_player.vel.1;

        if cur_player.vel.1 < 0 {
            cur_player.vel.1 /= 2;
        }
        if cur_player.vel.1 == -6 {
            cur_player.vel.1 = 2;
        }

        // for (posn, vel) in _game.positions.iter_mut().zip(_game.velocities.iter()) {
        //     posn.0 += vel.0;
        //     posn.1 += vel.1;
        // }
        let mut all_pos: Vec<Vec2i> = vec![cur_player.pos];
        let mut all_vel: Vec<Vec2i> = vec![cur_player.vel];
        // reset number of jumps
        // Detect collisions: Convert positions and sizes to collision bodies, generate contacts
        // Outline of a possible approach to tile collision:
        let mut contacts = vec![];
        gather_contacts(
            all_pos.as_slice(),
            &_game.sizes,
            &_game.side_map,
            &mut contacts,
            &mut _game.game_data.num_jumps,
            &mut _game.game_data.restart,
        );
        restitute(
            all_pos.as_mut_slice(),
            &_game.sizes,
            all_vel.as_mut_slice(),
            &mut _game.camera,
            &_game.side_map,
            &mut contacts,
        );
        cur_player.pos = all_pos[0];
        cur_player.vel = all_vel[0];

        if _game.game_data.restart {
            cur_player.pos = _game.spawn_point;
            _game.game_data.restart = false;
        }

        // println!("player pos {:?}", cur_player.pos);
        for (start, end) in _game.game_data.portals.iter(){
            if *start == cur_player.pos && _game.game == 1{
                cur_player.pos = *end;
                _game.spawn_point = *end;
            }
            if *start == cur_player.pos && _game.game == 2{
                cur_player.pos = *end;
                _game.spawn_point = *end;
            }
        }

        if cur_player.pos == Vec2i(114*32, 33*32) {
            println!("win!");
        }

        _game.camera.0 = _game.players[&_game.server.id].pos.0 - (WIDTH / 2) as i32;
        _game.camera.1 = _game.players[&_game.server.id].pos.1 - (HEIGHT / 2) as i32;

        _game.server.update_players(&mut _game.players);

        if key_input.key_held(VirtualKeyCode::X) {
            // StateResult::Remove
            _game.players.get_mut(&_game.server.id).unwrap().vel = Vec2i(0, 0);
            _game.players.get_mut(&_game.server.id).unwrap().world = 0;
            StateResult::Swap(Box::new(Title()))
        } else {
            StateResult::Keep
        }
    }
    fn display(
        &self,
        _game: &GameState,
        resources: &Resources,
        levels: &Vec<Level>,
        screen: &mut Screen,
        frame: usize,
    ) {
        // println!("Title: p to play");
        // screen.clear(Rgba(80, 80, 80, 255));
        screen.bitblt(
            &_game.textures[2],
            Rect {
                x: 0,
                y: 0,
                w: WIDTH as u16,
                h: HEIGHT as u16,
            },
            _game.background_pos,
        );
        screen.set_scroll(_game.camera);
        // let x = levels[0].0[0];
        levels[_game.level].0[0].draw(screen); //levels[0].0
        let cur_world = _game.players[&_game.server.id].world;
        for player in _game.players.iter()
            .filter(|p| p.1.world == cur_world)
        {
            screen.bitblt(
                &_game.textures[0],
                _game.anim_state[0].frame(),
                player.1.pos,
            );
        }
        for text_img in &_game.texts_sidescroll {
            screen.bitblt(
                &text_img.tex,
                text_img.from,
                text_img.to,
            );
        }
    }
}

pub fn process_input(
    game: &mut GameState,
    // input: &Input,
    resources: &Resources,
    levels: &Vec<Level>,
    frame: usize,
    key_input: &WinitInputHelper,
) {
    let mut this_state = game.state_stack.pop().unwrap();
    // println!("input {:?} on state {:?}", this_state);
    match this_state.update(game, resources, levels, frame, key_input) {
        StateResult::Remove => process_input(game, resources, levels, frame, key_input),
        StateResult::Keep => game.state_stack.push(this_state),
        StateResult::Push(new_state) => {
            game.state_stack.push(this_state);
            game.state_stack.push(new_state);
            process_input(game, resources, levels, frame, key_input);
        }
        StateResult::Swap(new_state) => {
            game.state_stack.push(new_state);
            process_input(game, resources, levels, frame, key_input);
        }
    }
}
