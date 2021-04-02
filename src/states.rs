use crate::tiles::*;
use crate::types::*;
use crate::texture::*;
use std::rc::Rc;
use crate::animation::*;
use crate::resources::*;
use crate::graphics::*;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use winit::event::VirtualKeyCode;
use crate::collision::*;

const WIDTH: usize = 320*2;
const HEIGHT: usize = 240*2;

pub struct GameData {
    pub score: usize,
    pub speed_multiplier: usize,
    pub num_jumps: usize,
}

pub struct GameState {
    // Every entity has a position, a size, a texture, and animation state.
    // Assume entity 0 is the player
    pub types: Vec<Player>,
    pub positions: Vec<Vec2i>,
    pub velocities: Vec<Vec2i>,
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
}

// Probably should be WinitInputHelper
type Input = str;

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
        let max_vel = 20;
        let min_vel = -20;
        if key_input.key_held(VirtualKeyCode::Right){
            _game.velocities[0].0 = 2;
            _game.anim_state[0].tick();
        } 
        if key_input.key_held(VirtualKeyCode::Left) {
            _game.velocities[0].0 = -2;
        } 
        if key_input.key_held(VirtualKeyCode::Up) {
            _game.velocities[0].1 = -2;
        } 
        if key_input.key_held(VirtualKeyCode::Down) {
            _game.velocities[0].1 = 2;
        } 
        for (posn, vel) in _game.positions.iter_mut().zip(_game.velocities.iter()) {
            posn.0 += vel.0;
            posn.1 += vel.1;
        }
        // update camera after restitution
        _game.camera.0 += _game.velocities[0].0;
        // _game.background_pos.0 += -1*_game.velocities[0].0;

        if key_input.key_held(VirtualKeyCode::P) {
            // println!("hitting p");
            StateResult::Swap(Box::new(Scroll()))
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
        screen.clear(Rgba(211, 211, 211, 255));
        screen.set_scroll(_game.camera);
        // levels[_game.level].0.draw(screen);
        let maps = &levels[1].0;
        for map in maps {
            map.draw(screen);
        }
        for ((pos, tex), anim) in _game
            .positions
            .iter()
            .zip(_game.textures.iter())
            .zip(_game.anim_state.iter())
        {
            screen.bitblt(tex, anim.frame(), *pos);
        }
    }
}
#[derive(Debug)]
pub struct Scroll(); // side scroller
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
        // StateResult::Keep
        let max_vel = 20;
        let min_vel = -20;
        // println!("before {:}", _game.velocities[0].0);
        if key_input.key_held(VirtualKeyCode::Right){
            // _game.velocities[0].0 = 7;
            // println!("right vel {:}", _game.velocities[0].0);
            if _game.velocities[0].0 < max_vel {
                // println!("inside");
                _game.velocities[0].0 +=2;
                // println!("inside {:}", _game.velocities[0].0);
            }
            _game.anim_state[0].tick();
        } else if key_input.key_released(VirtualKeyCode::Right) {
            _game.velocities[0].0 = (_game.velocities[0].0 as f32 * 0.25) as i32;
            // println!("after {:}", _game.velocities[0].0);
        }
        if key_input.key_held(VirtualKeyCode::Left) {
            if _game.velocities[0].0 > min_vel {
                // println!("inside");
                _game.velocities[0].0 -=2;
                // println!("inside {:}", _game.velocities[0].0);
            }
        } else if key_input.key_released(VirtualKeyCode::Left) {
            _game.velocities[0].0 = (_game.velocities[0].0 as f32 * 0.25) as i32;
        }
        if key_input.key_held(VirtualKeyCode::Up) {
            println!("jumps {:}", _game.game_data.num_jumps);
            if _game.game_data.num_jumps < 2 {
                _game.velocities[0].1 = -5;            
                _game.game_data.num_jumps += 1;
            }
        } else if key_input.key_released(VirtualKeyCode::Up) {
            _game.velocities[0].1 /= 2;
            _game.velocities[0].1 = 2;            
        }
        if key_input.key_held(VirtualKeyCode::Down) {
            if _game.velocities[0].1 < max_vel {
                println!("inside");
                _game.velocities[0].1 +=2;
                // println!("inside {:}", _game.velocities[0].0);
            }
        } else if key_input.key_released(VirtualKeyCode::Down) {
            _game.velocities[0].0 = (_game.velocities[0].0 as f32 * 0.25) as i32;
        }
        // Determine enemy velocity
        // Update all entities' positions
        // let speed_multiplier = _game.game_data.speed_multiplier;
        for (posn, vel) in _game.positions.iter_mut().zip(_game.velocities.iter()) {
            posn.0 += vel.0;
            posn.1 += vel.1;
        }
        // reset number of jumps
        // Detect collisions: Convert positions and sizes to collision bodies, generate contacts
        // Outline of a possible approach to tile collision:
        let mut contacts = vec![];
        gather_contacts(
            &_game.positions,
            &_game.sizes,
            &[&levels[_game.level].0[0]],
            &mut contacts,
            &mut _game.game_data.num_jumps,
        );
        restitute(
            &mut _game.positions,
            &_game.sizes,
            &mut _game.velocities,
            &mut _game.camera,
            &[&levels[_game.level].0[0]],
            &mut contacts,
        );

        // update camera after restitution
        _game.camera.0 += _game.velocities[0].0;
        _game.background_pos.0 += -1*_game.velocities[0].0;
        // _game.camera.1 += _game.velocities[0].1;
        // update tilemap after restitution    
        // _game.camera.1 += _game.velocities[0].1;

        if key_input.key_held(VirtualKeyCode::X) {
            StateResult::Remove
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
        screen.bitblt(&_game.textures[2], Rect{x: 0, y: 0, w: WIDTH as u16, h:HEIGHT as u16}, _game.background_pos);
        screen.set_scroll(_game.camera);
        // let x = levels[0].0[0];
        levels[_game.level].0[0].draw(screen); //levels[0].0
        for ((pos, tex), anim) in _game
            .positions
            .iter()
            .zip(_game.textures.iter())
            .zip(_game.anim_state.iter())
        {
            screen.bitblt(tex, anim.frame(), *pos);
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
