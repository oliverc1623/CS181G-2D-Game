use std::path::Path;
use std::rc::Rc;

use fontdue::layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle};
use fontdue::Font;

use winit::dpi::LogicalSize;
use winit::event::VirtualKeyCode;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use Game2DEngine::animation::*;
use Game2DEngine::graphics::Screen;
use Game2DEngine::tiles::*;
use Game2DEngine::types::*;

// use engine2d::collision::*;
// Imagine a Resources struct (we'll call it AssetDB or Assets in the future)
// which wraps all accesses to textures, sounds, animations, etc.
use Game2DEngine::resources::*;
use Game2DEngine::texture::Texture;

const WIDTH: usize = 320 * 2;
const HEIGHT: usize = 240 * 2;
const CHARACTER: char = 'b';
const SIZE: f32 = 20.0;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum EntityType {
    Player,
    Enemy,
}

type Level = (Tilemap, Vec<(EntityType, i32, i32)>);

struct GameData {
    score: usize,
    speed_multiplier: usize,
}

struct GameState {
    // Every entity has a position, a size, a texture, and animation state.
    // Assume entity 0 is the player
    types: Vec<EntityType>,
    positions: Vec<Vec2i>,
    velocities: Vec<Vec2i>,
    sizes: Vec<(usize, usize)>,
    textures: Vec<Rc<Texture>>,
    anim_state: Vec<AnimationState>,
    // Current level
    level: usize,
    // Camera position
    camera: Vec2i,
    state_stack: Vec<Box<dyn State>>,
    game_data: GameData,
}

// Probably should be WinitInputHelper
type Input = str;

#[derive(Debug)]
enum StateResult {
    // Pop this state off the stack, update the one before me
    Remove,
    // Keep this state as is, quit propagating updates
    Keep,
    // Swap this state for a new one, update the new one too
    Swap(Box<dyn State>),
    // Push a new state on top of this one, update it too
    Push(Box<dyn State>),
}
trait State: std::fmt::Debug {
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
struct Title();
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
        if key_input.key_held(VirtualKeyCode::Down) {
            _game.velocities[0].1 = 5;
        } else {
            _game.velocities[0].1 /= 2;
        }
        if key_input.key_held(VirtualKeyCode::P) {
            println!("hitting p");
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
        println!("Title: p to play");
        screen.clear(Rgba(80, 80, 80, 255));
        screen.set_scroll(_game.camera);
        levels[_game.level].0.draw(screen);
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
struct Scroll();
impl State for Scroll {
    fn update(
        &mut self,
        game: &mut GameState,
        resources: &Resources,
        levels: &Vec<Level>,
        frame: usize,
        key_input: &WinitInputHelper,
    ) -> StateResult {
        StateResult::Keep
    }
    fn display(
        &self,
        game: &GameState,
        resources: &Resources,
        levels: &Vec<Level>,
        screen: &mut Screen,
        frame: usize,
    ) {
        
    }
}

fn process_input(
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

fn main() {
    let window_builder = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Anim2D")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .with_resizable(false)
    };
    // Here's our resources...
    let mut rsrc = Resources::new();
    let tileset = Rc::new(Tileset::new(
        vec![
            Tile { solid: false }, // 0
            Tile { solid: false }, // 1
            Tile { solid: false }, // 2
            Tile { solid: false }, // 3
            Tile { solid: true },  // 4
            Tile { solid: true },  // 5
            Tile { solid: false }, // 6
            Tile { solid: false }, // 7
            Tile { solid: true },  // 8
            Tile { solid: false }, // 9
            Tile { solid: true },  // 10
            Tile { solid: true },  // 11
            Tile { solid: false }, // 12
            Tile { solid: false }, // 13
            Tile { solid: false }, // 14
            Tile { solid: false }, // 15
            Tile { solid: false },
        ],
        &rsrc.load_texture(Path::new("content/tiles128.png")),
    ));
    // Here's our game rules (the engine doesn't know about these)
    let levels: Vec<Level> = vec![
        (
            // The map
            Tilemap::new(
                Vec2i(0, 0),
                // Map size
                (32, 8),
                &tileset,
                // Tile grid
                vec![
                    2, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 9, 7,
                    0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 7, 0, 0, 9, 0, 0, 0,
                    0, 0, 0, 0, 9, 0, 10, 0, 0, 0, 0, 0, 2, 0, 4, 4, 0, 0, 0, 0, 9, 0, 6, 0, 0, 0,
                    0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 7, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
                    9, 0, 0, 6, 8, 8, 8, 8, 9, 8, 8, 8, 8, 8, 8, 8, 9, 0, 0, 0, 5, 0, 0, 0, 2, 0,
                    0, 0, 5, 0, 0, 0, 9, 0, 0, 0, 10, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0,
                    0, 0, 0, 0, 2, 0, 0, 0, 5, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0,
                    0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 2, 9, 0, 0, 0, 0, 0, 0, 2,
                    9, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
                    8, 8, 8, 8, 8, 8, 9, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0,
                ],
            ),
            // Initial entities on level start
            vec![(EntityType::Player, 10, 6), (EntityType::Enemy, 10, 10)],
        ),
        (
            // The map
            Tilemap::new(
                Vec2i(0, 0),
                // Map size
                (32, 32),
                &tileset,
                // Tile grid
                vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 7,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 8, 0, 9, 0, 10, 0, 0, 0, 0, 0, 8, 8, 8, 0, 8, 8, 8, 0, 8, 8, 8, 0, 8, 0,
                    0, 0, 0, 0, 8, 0, 0, 8, 0, 0, 9, 0, 0, 7, 0, 0, 0, 0, 8, 0, 8, 0, 0, 8, 0, 0,
                    8, 0, 8, 0, 8, 0, 8, 8, 8, 0, 0, 8, 0, 8, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 8, 8,
                    8, 0, 8, 0, 0, 0, 8, 8, 8, 0, 8, 0, 8, 0, 8, 0, 0, 0, 8, 0, 0, 0, 9, 0, 0, 0,
                    0, 0, 0, 0, 8, 0, 0, 0, 8, 8, 8, 0, 8, 0, 0, 0, 8, 0, 8, 0, 8, 8, 0, 0, 8, 0,
                    0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 8,
                    8, 8, 0, 0, 8, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 8, 0,
                    0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 8, 8, 8, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 9, 7, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 8, 0, 0, 0, 0, 7, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0,
                    9, 0, 10, 0, 0, 0, 0, 0, 8, 0, 0, 0, 8, 0, 8, 0, 8, 0, 6, 0, 0, 0, 0, 0, 9, 0,
                    0, 0, 0, 0, 0, 0, 9, 0, 0, 7, 0, 0, 0, 0, 8, 0, 0, 0, 8, 0, 8, 8, 8, 0, 0, 6,
                    8, 8, 8, 8, 9, 8, 8, 8, 8, 8, 8, 8, 9, 0, 0, 0, 5, 0, 0, 0, 8, 0, 0, 0, 8, 0,
                    8, 0, 0, 0, 0, 0, 10, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0,
                    8, 0, 8, 0, 8, 0, 8, 8, 8, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 9, 0,
                    0, 0, 0, 0, 0, 0, 8, 8, 0, 8, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 9, 0, 0, 0,
                    0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 8,
                    8, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 9, 7, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 8, 0, 8, 0, 0, 0, 0, 7, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 9, 0, 10, 0,
                    0, 0, 0, 0, 8, 0, 0, 0, 8, 0, 8, 0, 8, 0, 6, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0,
                    0, 0, 9, 0, 0, 7, 0, 0, 0, 0, 8, 0, 0, 0, 8, 0, 8, 8, 8, 0, 0, 6, 8, 8, 8, 8,
                    9, 8, 8, 8, 8, 8, 8, 8, 9, 0, 0, 0, 5, 0, 0, 0, 8, 0, 0, 0, 8, 0, 8, 0, 0, 0,
                    0, 0, 10, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 8, 0, 8, 0,
                    8, 0, 8, 8, 8, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0,
                    0, 0, 8, 8, 0, 8, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 9, 0, 0, 0, 0, 0, 0, 0,
                    9, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 8, 8, 0, 0, 0,
                    0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 9, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    8, 0, 8, 0, 0, 0, 0, 7, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 9, 0, 10, 0, 0, 0, 0, 0,
                    8, 0, 0, 0, 8, 0, 8, 0, 8, 0, 6, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 9, 0,
                    0, 7, 0, 0, 0, 0, 8, 0, 0, 0, 8, 0, 8, 8, 8, 0, 0, 6, 8, 8, 8, 8, 9, 8, 8, 8,
                    8, 8, 8, 8, 9, 0, 0, 0, 5, 0, 0, 0, 8, 0, 0, 0, 8, 0, 8, 0, 0, 0, 0, 0, 10, 0,
                    0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 8, 0, 8, 0, 8, 0, 8, 8,
                    8, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 8, 8,
                    0, 8, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 9, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0,
                    0, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
            ),
            // Initial entities on level start
            vec![(EntityType::Player, 10, 6), (EntityType::Enemy, 20, 16)],
        ),
    ];
    let player_tex = rsrc.load_texture(Path::new("content/tiles128.png"));
    let player_anim = Rc::new(Animation::freeze(Rect {
        x: 0,
        y: 32,
        w: 32,
        h: 32,
    }));
    let enemy_tex = Rc::clone(&player_tex);
    let enemy_anim = Rc::new(Animation::freeze(Rect {
        x: 16,
        y: 0,
        w: 16,
        h: 16,
    }));
    // And here's our game state, which is just stuff that changes.
    // We'll say an entity is a type, a position, a velocity, a size, a texture, and an animation state.
    // State here will stitch them all together.
    let mut game = GameState {
        // Every entity has a position, a size, a texture, and animation state.
        // Assume entity 0 is the player
        types: vec![
            // In a real example we'd provide nicer accessors than this
            levels[0].1[0].0,
            levels[0].1[1].0,
        ],
        positions: vec![
            Vec2i(levels[0].1[0].1 * 16, levels[0].1[0].2 * 16),
            Vec2i(levels[0].1[1].1 * 16, levels[0].1[1].2 * 16),
        ],
        velocities: vec![Vec2i(0, 0), Vec2i(0, 0)],
        sizes: vec![(16, 16), (16, 16)],
        // Could be texture handles instead, let's talk about that in two weeks
        textures: vec![Rc::clone(&player_tex), Rc::clone(&enemy_tex)],
        anim_state: vec![player_anim.start(), enemy_anim.start()],
        // Current level
        level: 1,
        // Camera position
        camera: Vec2i(0, 0),
        state_stack: vec![Box::new(Title())],
        game_data: GameData {
            score: 0,
            speed_multiplier: 1,
        },
    };
    Game2DEngine::run(
        WIDTH,
        HEIGHT,
        window_builder,
        rsrc,
        levels,
        game,
        draw_game,
        update_game,
    );
}

fn draw_game(
    resources: &Resources,
    levels: &Vec<Level>,
    state: &GameState,
    screen: &mut Screen,
    frame: usize,
) {
    state
        .state_stack
        .last()
        .unwrap()
        .display(&state, resources, levels, screen, frame);
}

fn update_game(
    resources: &Resources,
    levels: &Vec<Level>,
    state: &mut GameState,
    key_input: &WinitInputHelper,
    frame: usize,
) {
    process_input(state, resources, levels, frame, key_input);
}
