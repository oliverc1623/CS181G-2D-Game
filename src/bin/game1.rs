use std::path::Path;
use std::rc::Rc;

use fontdue::layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle};
use fontdue::Font;

use winit::dpi::LogicalSize;
use winit::event::VirtualKeyCode;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use Game2DEngine::animation::*;
use Game2DEngine::collision::*;
use Game2DEngine::graphics::Screen;
use Game2DEngine::tiles::*;
use Game2DEngine::types::*;
// Imagine a Resources struct (we'll call it AssetDB or Assets in the future)
// which wraps all accesses to textures, sounds, animations, etc.
use Game2DEngine::resources::*;
use Game2DEngine::texture::Texture;
use Game2DEngine::states::*;

const WIDTH: usize = 320*2;
const HEIGHT: usize = 240*2;
const CHARACTER: char = 'b';
const SIZE: f32 = 20.0;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum EntityType {
    // Player,
    Enemy,
}

// type Level = (Vec<Tilemap>, Vec<(EntityType, i32, i32)>);
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
            Tile { solid: false, jump_reset: false}, // 0
            Tile { solid: false,  jump_reset: false }, // 1
            Tile { solid: false,  jump_reset: false  }, // 2
            Tile { solid: false,  jump_reset: false  }, // 3
            Tile { solid: false,  jump_reset: false  },  // 4
            Tile { solid: false,  jump_reset: false  },  // 5
            Tile { solid: false ,  jump_reset: false }, // 6
            Tile { solid: false,  jump_reset: false  },  // 7
            Tile { solid: true,  jump_reset: true  },  // 8 stone
            Tile { solid: false,  jump_reset: false  },  // 9
            Tile { solid: false,  jump_reset: false  },  // 10
            Tile { solid: false ,  jump_reset: false },  // 11
            Tile { solid: false,  jump_reset: false  }, // 12
            Tile { solid: false,  jump_reset: false  }, // 13
            Tile { solid: false,  jump_reset: false  }, // 14
            Tile { solid: false,  jump_reset: false  }, // 15
            Tile { solid: false,  jump_reset: false  },
        ],
        &rsrc.load_texture(Path::new("content/tilesheet.png")),
    ));
    // overworld tileset
    let overworld_tileset = Rc::new(Tileset::new(
        vec![
            Tile { solid: true, jump_reset: false },
            Tile { solid: false, jump_reset: false },
            Tile { solid: true, jump_reset: false },
            Tile { solid: false, jump_reset: false },
            Tile { solid: false, jump_reset: false },
            Tile { solid: false, jump_reset: false },
            Tile { solid: false, jump_reset: false },
            Tile { solid: false, jump_reset: false },
            Tile { solid: false, jump_reset: false },
            Tile { solid: false, jump_reset: false },
            Tile { solid: true, jump_reset: false },
            Tile { solid: false, jump_reset: false },
            Tile { solid: true, jump_reset: false },
        ],
        &rsrc.load_texture(Path::new("content/tilesheet.png")),
    ));
    // Here's our game rules (the engine doesn't know about these)
    let levels: Vec<Level> = vec![
        ( // level 0 is the side scroller
            // The map
            vec![Tilemap::new(
                Vec2i(0, 0),
                // Map size
                (32, 16),
                &tileset,
                // Tile grid
                vec![
                    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 8, 
                    1, 1, 1, 1, 1, 1, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 1, 1, 1, 1, 1, 1, 1, 1, 8,
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 8,
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 8,
                    1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 8,
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 8,
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 8,
                    
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 8,
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 8,
                    1, 1, 1, 1, 1, 1, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 8,
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 8,
                    1, 1, 1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 8,
                    1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 8,
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 8,
                    8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
                ],
            )],
            // Initial entities on level start
            vec![(Player::new(), 15, 29)], // TODO: add three other player types
        ),
        ( // level 1 is the overworld map
            // The map
            vec![Tilemap::new(// tilemap 1
                Vec2i(0, 0),
                (8, 8),
                &overworld_tileset,
                vec![
                    8, 8, 8, 8, 8, 8, 8, 8,
                    8, 8, 8, 8, 8, 8, 5, 8,
                    8, 8, 12, 8, 8, 8, 8, 8,
                    8, 8, 8, 8, 12, 8, 8, 8,
                    8, 9, 9, 8, 8, 9, 8, 8,
                    2, 2, 2, 2, 2, 2, 2, 2,
                    8, 8, 9, 9, 8, 9, 9, 8,
                    8, 8, 8, 8, 8, 8, 12, 8,
                ]), Tilemap::new(// tilemap 2
                    Vec2i(256, 0),
                    (8, 8),
                    &overworld_tileset,
                    vec![
                        8, 8, 0, 8, 8, 8, 8, 8,
                        8, 8, 0, 8, 8, 8, 5, 8,
                        8, 8, 0, 8, 8, 8, 12, 8,
                        8, 8, 0, 8, 12, 8, 8, 8,
                        8, 9, 0, 8, 8, 8, 8, 8,
                        2, 2, 2, 8, 8, 6, 8, 8,
                        8, 8, 9, 9, 8, 8, 8, 7,
                        8, 8, 8, 8, 8, 8, 12, 8,
                    ],
                ), Tilemap::new(// tilemap 3
                    Vec2i(0, 256),
                    (8, 8),
                    &overworld_tileset,
                    vec![
                        8, 8, 0, 8, 8, 8, 8, 8,
                        8, 12, 0, 8, 8, 7, 8, 8,
                        8, 8, 0, 8, 8, 8, 8, 8,
                        8, 8, 0, 4, 8, 8, 8, 8,
                        8, 8, 0, 8, 8, 12, 8, 8,
                        8, 8, 0, 9, 8, 8, 8, 8,
                        8, 8, 2, 2, 8, 8, 8, 8,
                        8, 8, 8, 0, 8, 8, 8, 8,
                    ],
                ), Tilemap::new(// tilemap 4
                    Vec2i(256, 256),
                    (8, 8),
                    &overworld_tileset,
                    vec![
                        8, 8, 8, 8, 8, 8, 8, 8,
                        8, 8, 8, 8, 8, 8, 8, 8,
                        8, 8, 8, 8, 8, 12, 8, 8,
                        8, 8, 8, 8, 8, 8, 10, 7,
                        8, 8, 8, 8, 8, 8, 10, 6,
                        8, 8, 8, 8, 8, 12, 8, 8,
                        8, 8, 8, 8, 8, 8, 8, 8,
                        8, 8, 8, 8, 8, 8, 8, 8,
                    ],
                ), Tilemap::new(// tilemap 5
                    Vec2i(0, 256 * 2),
                    (8, 8),
                    &overworld_tileset,
                    vec![
                        8, 8, 8, 8, 8, 8, 8, 8,
                        8, 8, 8, 8, 8, 8, 5, 8,
                        8, 8, 12, 8, 8, 8, 8, 8,
                        8, 8, 8, 8, 12, 8, 8, 8,
                        8, 9, 9, 8, 8, 9, 8, 8,
                        2, 2, 2, 2, 2, 2, 2, 2,
                        8, 8, 9, 9, 8, 9, 9, 8,
                        8, 8, 8, 8, 8, 8, 12, 8,
                    ],
                ), Tilemap::new(// tilemap 6
                    Vec2i(0, 256 * 3),
                    (8, 8),
                    &overworld_tileset,
                    vec![
                        8, 8, 0, 8, 8, 8, 8, 8,
                        8, 8, 0, 8, 8, 8, 5, 8,
                        8, 8, 0, 8, 8, 8, 12, 8,
                        8, 8, 0, 8, 12, 8, 8, 8,
                        8, 9, 0, 8, 8, 8, 8, 8,
                        2, 2, 2, 8, 8, 6, 8, 8,
                        8, 8, 9, 9, 8, 8, 8, 7,
                        8, 8, 8, 8, 8, 8, 12, 8,
                    ],
                ), Tilemap::new(// tilemap 7
                    Vec2i(256, 256 * 2),
                    (8, 8),
                    &overworld_tileset,
                    vec![
                        8, 8, 0, 8, 8, 8, 8, 8,
                        8, 12, 0, 8, 8, 7, 8, 8,
                        8, 8, 0, 8, 8, 8, 8, 8,
                        8, 8, 0, 4, 8, 8, 8, 8,
                        8, 8, 0, 8, 8, 12, 8, 8,
                        8, 8, 0, 9, 8, 8, 8, 8,
                        8, 8, 2, 2, 8, 8, 8, 8,
                        8, 8, 8, 0, 8, 8, 8, 8,
                    ],
                ), Tilemap::new(// tilemap 8
                    Vec2i(256, 256 * 3),
                    (8, 8),
                    &overworld_tileset,
                    vec![
                        8, 8, 8, 8, 8, 8, 8, 8,
                        8, 8, 8, 8, 8, 8, 8, 8,
                        8, 8, 8, 8, 8, 12, 8, 8,
                        8, 8, 8, 8, 8, 8, 10, 7,
                        8, 8, 8, 8, 8, 8, 10, 6,
                        8, 8, 8, 8, 8, 12, 8, 8,
                        8, 8, 8, 8, 8, 8, 8, 8,
                        8, 8, 8, 8, 8, 8, 8, 8,
                    ],
                ), Tilemap::new(// tilemap 9
                    Vec2i(256 * 2, 0),
                    (8, 8),
                    &overworld_tileset,
                    vec![
                        8, 8, 8, 8, 8, 8, 8, 8,
                        8, 8, 8, 8, 8, 8, 5, 8,
                        8, 8, 12, 8, 8, 8, 8, 8,
                        8, 8, 8, 8, 12, 8, 8, 8,
                        8, 9, 9, 8, 8, 9, 8, 8,
                        2, 2, 2, 2, 2, 2, 2, 2,
                        8, 8, 9, 9, 8, 9, 9, 8,
                        8, 8, 8, 8, 8, 8, 12, 8,
                    ],
                ), Tilemap::new(// tilemap 10
                    Vec2i(256 * 2, 256),
                    (8, 8),
                    &overworld_tileset,
                    vec![
                        8, 8, 0, 8, 8, 8, 8, 8,
                        8, 8, 0, 8, 8, 8, 5, 8,
                        8, 8, 0, 8, 8, 8, 12, 8,
                        8, 8, 0, 8, 12, 8, 8, 8,
                        8, 9, 0, 8, 8, 8, 8, 8,
                        2, 2, 2, 8, 8, 6, 8, 8,
                        8, 8, 9, 9, 8, 8, 8, 7,
                        8, 8, 8, 8, 8, 8, 12, 8,
                    ],
                ), Tilemap::new(// tilemap 11
                    Vec2i(256 * 2, 256 * 2),
                    (8, 8),
                    &overworld_tileset,
                    vec![
                        8, 8, 0, 8, 8, 8, 8, 8,
                        8, 12, 0, 8, 8, 7, 8, 8,
                        8, 8, 0, 8, 8, 8, 8, 8,
                        8, 8, 0, 4, 8, 8, 8, 8,
                        8, 8, 0, 8, 8, 12, 8, 8,
                        8, 8, 0, 9, 8, 8, 8, 8,
                        8, 8, 2, 2, 8, 8, 8, 8,
                        8, 8, 8, 0, 8, 8, 8, 8,
                    ],
                ), Tilemap::new(// tilemap 12
                    Vec2i(256 * 2, 256 * 3),
                    (8, 8),
                    &overworld_tileset,
                    vec![
                        8, 8, 8, 8, 8, 8, 8, 8,
                        8, 8, 8, 8, 8, 8, 8, 8,
                        8, 8, 8, 8, 8, 12, 8, 8,
                        8, 8, 8, 8, 8, 8, 10, 7,
                        8, 8, 8, 8, 8, 8, 10, 6,
                        8, 8, 8, 8, 8, 12, 8, 8,
                        8, 8, 8, 8, 8, 8, 8, 8,
                        8, 8, 8, 8, 8, 8, 8, 8,
                    ],
                ), Tilemap::new(// tilemap 13
                    Vec2i(256 * 3, 0),
                    (8, 8),
                    &overworld_tileset,
                    vec![
                        8, 8, 8, 8, 8, 8, 8, 8,
                        8, 8, 8, 8, 8, 8, 5, 8,
                        8, 8, 12, 8, 8, 8, 8, 8,
                        8, 8, 8, 8, 12, 8, 8, 8,
                        8, 9, 9, 8, 8, 9, 8, 8,
                        2, 2, 2, 2, 2, 2, 2, 2,
                        8, 8, 9, 9, 8, 9, 9, 8,
                        8, 8, 8, 8, 8, 8, 12, 8,
                    ],
                ), Tilemap::new(// tilemap 14
                    Vec2i(256 * 3, 256),
                    (8, 8),
                    &overworld_tileset,
                    vec![
                        8, 8, 0, 8, 8, 8, 8, 8,
                        8, 8, 0, 8, 8, 8, 5, 8,
                        8, 8, 0, 8, 8, 8, 12, 8,
                        8, 8, 0, 8, 12, 8, 8, 8,
                        8, 9, 0, 8, 8, 8, 8, 8,
                        2, 2, 2, 8, 8, 6, 8, 8,
                        8, 8, 9, 9, 8, 8, 8, 7,
                        8, 8, 8, 8, 8, 8, 12, 8,
                    ],
                ), Tilemap::new(// tilemap 15
                    Vec2i(256 * 3, 256 * 2),
                    (8, 8),
                    &overworld_tileset,
                    vec![
                        8, 8, 0, 8, 8, 8, 8, 8,
                        8, 12, 0, 8, 8, 7, 8, 8,
                        8, 8, 0, 8, 8, 8, 8, 8,
                        8, 8, 0, 4, 8, 8, 8, 8,
                        8, 8, 0, 8, 8, 12, 8, 8,
                        8, 8, 0, 9, 8, 8, 8, 8,
                        8, 8, 2, 2, 8, 8, 8, 8,
                        8, 8, 8, 0, 8, 8, 8, 8,
                    ],
                ), Tilemap::new( // tilemap 16
                    Vec2i(256 * 3, 256 * 3),
                    (8, 8),
                    &overworld_tileset,
                    vec![
                        8, 8, 8, 8, 8, 8, 8, 8,
                        8, 8, 8, 8, 8, 8, 8, 8,
                        8, 8, 8, 8, 8, 12, 8, 8,
                        8, 8, 8, 8, 8, 8, 10, 7,
                        8, 8, 8, 8, 8, 8, 10, 6,
                        8, 8, 8, 8, 8, 12, 8, 8,
                        8, 8, 8, 8, 8, 8, 8, 8,
                        8, 8, 8, 8, 8, 8, 8, 8,
                    ],
                )],
            // Initial entities on level start
            vec![(Player::new(), 10, 6)],
        ),
    ];
    let player_tex = rsrc.load_texture(Path::new("content/wiry_all_side.png"));
    let player_anim = Rc::new(Animation::new(
        vec![
            (Rect {
                x: 0,
                y: 0,
                w: 32,
                h: 32,
            }, 8),
            (Rect {
                x: 32,
                y: 0,
                w: 32,
                h: 32,
            }, 8),
            (Rect {
                x: 64,
                y: 0,
                w: 32,
                h: 32,
            }, 8),
        ],  true));
    let enemy_tex = rsrc.load_texture(Path::new("content/tilesheet.png"));
    let enemy_anim = Rc::new(Animation::freeze(Rect {
        x: 0,
        y: 32,
        w: 32,
        h: 32,
    }));
    let overworld_player_tex = rsrc.load_texture(Path::new("content/wiry_all.png"));

    // flipping sprites: 1) take sprite sheet and copy sprite and flip
    // 2) when load image, make flipped version of spritesheet
    // 3) add parameter to bitblit: scale negative values to x/y direction and bool for flipped
    //    copy col 0 to col w and col 1 to col w-1. play around with row.b variables. 
    let overworld_player_anim = Rc::new(Animation::new(
        vec![
            (Rect {
                x: 0,
                y: 0,
                w: 32,
                h: 32,
            }, 8),
            (Rect {
                x: 32,
                y: 0,
                w: 32,
                h: 32,
            }, 8),
            (Rect {
                x: 64,
                y: 0,
                w: 32,
                h: 32,
            }, 8),
        ],  true));
    let background_tex = rsrc.load_texture(Path::new("content/badland_background.png"));
    // And here's our game state, which is just stuff that changes.
    // We'll say an entity is a type, a position, a velocity, a size, a texture, and an animation state.
    // State here will stitch them all together.
    let mut game = GameState {
        // Every entity has a position, a size, a texture, and animation state.
        // Assume entity 0 is the player
        types: vec![
            // In a real example we'd provide nicer accessors than this
            levels[0].1[0].0,
            // levels[0].1[1].0,
        ],
        positions: vec![
            Vec2i(levels[0].1[0].1 * 16, levels[0].1[0].2 * 16), // player pos
        ],
        velocities: vec![Vec2i(0, 0)],
        sizes: vec![(32, 32)],
        // Could be texture handles instead, let's talk about that in two weeks
        textures: vec![Rc::clone(&player_tex), Rc::clone(&enemy_tex), Rc::clone(&background_tex), Rc::clone(&overworld_player_tex)],
        anim_state: vec![player_anim.start(), enemy_anim.start(), overworld_player_anim.start()],
        // Current level
        level: 1,
        // Camera position
        camera: Vec2i(0, 64),
        // background position
        background_pos: Vec2i(0,0),
        state_stack: vec![Box::new(Title())],
        game_data: GameData {
            score: 0,
            speed_multiplier: 1,
            num_jumps: 0,
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
