use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

use winit::dpi::LogicalSize;
use rusttype::Font;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use Game2DEngine::animation::*;
use Game2DEngine::graphics::Screen;
use Game2DEngine::tiles::*;
use Game2DEngine::types::*;
// Imagine a Resources struct (we'll call it AssetDB or Assets in the future)
// which wraps all accesses to textures, sounds, animations, etc.
use Game2DEngine::resources::*;
use Game2DEngine::server::Server;
use Game2DEngine::states::*;
use Game2DEngine::save::*;

const WIDTH: usize = 320 * 2;
const HEIGHT: usize = 240 * 2;

// type Level = (Vec<Tilemap>, Vec<(EntityType, i32, i32)>);
fn main() {
    let window_builder = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Exploration of Wiry")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .with_resizable(true)
    };
    // Here's our resources...
    let rsrc = Resources::new();
    let tileset = Rc::new(Tileset::new(
        vec![
            Tile {
                solid: false,
                jump_reset: false,
                restart: false,
            }, // 0
            Tile {
                solid: false,
                jump_reset: false,
                restart: false,
            }, // 1
            Tile {
                solid: false,
                jump_reset: false,
                restart: false,
            }, // 2
            Tile {
                solid: false,
                jump_reset: false,
                restart: false,
            }, // 3
            Tile {
                solid: true,
                jump_reset: true,
                restart: false,
            }, // 4
            Tile {
                solid: false,
                jump_reset: false,
                restart: false,
            }, // 5
            Tile {
                solid: false,
                jump_reset: false,
                restart: false,
            }, // 6
            Tile {
                solid: false,
                jump_reset: false,
                restart: true,
            }, // 7
            Tile {
                solid: false,
                jump_reset: false,
                restart: false,
            }, // 8
            Tile {
                solid: false,
                jump_reset: false,
                restart: false,
            }, // 9
            Tile {
                solid: false,
                jump_reset: false,
                restart: false,
            }, // 10
            Tile {
                solid: false,
                jump_reset: false,
                restart: false,
            }, // 11
            Tile {
                solid: false,
                jump_reset: false,
                restart: false,
            }, // 12
            Tile {
                solid: false,
                jump_reset: false,
                restart: false,
            }, // 13
            Tile {
                solid: false,
                jump_reset: false,
                restart: false,
            }, // 14
            Tile {
                solid: false,
                jump_reset: false,
                restart: false,
            }, // 15
            Tile {
                solid: false,
                jump_reset: false,
                restart: false,
            }, //16
            Tile {
                solid: false,
                jump_reset: false,
                restart: false,
            },//17
            Tile {
                solid: false,
                jump_reset: false,
                restart: false,
            }, //18
            Tile {
                solid: false,
                jump_reset: false,
                restart: false,
            }, //19
            Tile {
                solid: false,
                jump_reset: false,
                restart: false,
            }, //20
            Tile {
                solid: false,
                jump_reset: false,
                restart: false,
            }, //21
            Tile {
                solid: false,
                jump_reset: false,
                restart: false,
            }, //22
            Tile {
                solid: false,
                jump_reset: false,
                restart: false,
            }, //23
            Tile {
                solid: false,
                jump_reset: false,
                restart: false,
            }, //24
            Tile {
                solid: false,
                jump_reset: false,
                restart: false,
            }, //25
            Tile {
                solid: false,
                jump_reset: false,
                restart: false,
            }, //26
            Tile {
                solid: false,
                jump_reset: false,
                restart: false,
            }, //27
            Tile {
                solid: false,
                jump_reset: false,
                restart: false,
            }, //28
            Tile {
                solid: false,
                jump_reset: false,
                restart: false,
            }, //29
            Tile {
                solid: false,
                jump_reset: false,
                restart: false,
            }, //30
            Tile {
                solid: false,
                jump_reset: false,
                restart: false,
            }, //31
            Tile {
                solid: false,
                jump_reset: false,
                restart: false,
            }, //32
            Tile {
                solid: false,
                jump_reset: false,
                restart: false,
            }, //33
            Tile {
                solid: false,
                jump_reset: false,
                restart: false,
            }, //34
            Tile {
                solid: false,
                jump_reset: false,
                restart: false,
            }, //35
        ],
        &rsrc.load_texture(Path::new("content/game2_tilesheet.png")),
    ));
    // overworld tileset
    let overworld_tileset = Rc::new(Tileset::new(
        vec![
            Tile { solid: false, jump_reset: false,restart:false },//0
            Tile { solid: false, jump_reset: false,restart:false },//1
            Tile { solid: false, jump_reset: false,restart:false },//2
            Tile { solid: false, jump_reset: false,restart:false },//3
            Tile { solid: false, jump_reset: false,restart:false },//4
            Tile { solid: false, jump_reset: false,restart:false },//5
            Tile { solid: false, jump_reset: false,restart:false },//6
            Tile { solid: true, jump_reset: false ,restart:false},//7
            Tile { solid: false, jump_reset: false,restart:false },//8
            Tile { solid: true, jump_reset: false ,restart:false},//9
            Tile { solid: false, jump_reset: false,restart:false },//10
            Tile { solid: false, jump_reset: false,restart:false },//11
            Tile { solid: false, jump_reset: false,restart:false },//12
            Tile { solid: false, jump_reset: false,restart:false },//13
            Tile { solid: false, jump_reset: false,restart:false },//14
            Tile { solid: false, jump_reset: false,restart:false },//15
            Tile { solid: false, jump_reset: false,restart:false },//16
            Tile { solid: false, jump_reset: false,restart:false },//17
            Tile { solid: false, jump_reset: false,restart:false },//18
            Tile { solid: false, jump_reset: false,restart:false },//19
            Tile { solid: false, jump_reset: false,restart:false },//20
            Tile { solid: false, jump_reset: false,restart:false },//21
            Tile { solid: false, jump_reset: false,restart:false },//22
            Tile { solid: false, jump_reset: false,restart:false },//23
            Tile { solid: false, jump_reset: false,restart:false },//24
            Tile { solid: false, jump_reset: false,restart:false },//25
            Tile { solid: false, jump_reset: false,restart:false },//26
            Tile { solid: false, jump_reset: false,restart:false },//27
            Tile { solid: false, jump_reset: false,restart:false },//28
            Tile { solid: false, jump_reset: false,restart:false },//29
            Tile { solid: false, jump_reset: false,restart:false },//30
            Tile { solid: false, jump_reset: false,restart:false },//31
            Tile { solid: false, jump_reset: false,restart:false },//32
            Tile { solid: false, jump_reset: false,restart:false },//33
            Tile { solid: false, jump_reset: false,restart:false },//34
            Tile { solid: false, jump_reset: false,restart:false },//35
        ],
        &rsrc.load_texture(Path::new("content/game2_tilesheet.png")),
    ));

    pub fn get_maps(tileset: &Rc<Tileset>) -> Vec<Tilemap> {
        let map = Tilemap::new(
            Vec2i(0, 0),
            (8, 8),
            &tileset,
            vec![
                3, 0, 1, 2, 0, 4, 4, 6,
                1, 0, 2, 3, 3, 6, 9, 9,
                0, 6, 0, 2, 0, 9, 9, 0,
                0, 0, 1, 1, 0, 6, 0, 0,
                4, 0, 0, 0, 0, 6, 0, 0,
                2, 1, 1, 2, 0, 9, 9, 0,
                1, 3, 0, 1, 2, 6, 9, 9,
                0, 3, 2, 1, 0, 3, 6, 6,
            ],
        );
        let map2 = Tilemap::new(
            Vec2i(256, 0),
            (8, 8),
            &tileset,
            vec![
                0, 2, 2, 1, 3, 0, 6, 0,
                9, 9, 9, 9, 9, 9, 0, 0,
                0, 0, 0, 0, 0, 9, 6, 6,
                0, 0, 0, 0, 0, 9, 2, 6,
                0, 0, 0, 0, 0, 9, 5, 6,
                0, 0, 0, 0, 0, 9, 2, 0,
                9, 0, 0, 0, 0, 9, 3, 3,
                9, 9, 0, 0, 0, 9, 0, 4,
            ],
        );
        let map3 = Tilemap::new(
            Vec2i(0, 256),
            (8, 8),
            &tileset,
            vec![
                3, 0, 0, 0, 1, 6, 0, 1,
                0, 7, 7, 0, 0, 0, 2, 3,
                1, 3, 0, 4, 0, 0, 2, 0,
                2, 0, 4, 9, 9, 9, 9, 9,
                1, 3, 9, 9, 0, 0, 0, 0,
                1, 3, 9, 0, 0, 9, 9, 9,
                0, 0, 9, 0, 0, 9, 6, 2,
                6, 2, 9, 0, 0, 9, 1, 0,
            ],
        );
        let map4 = Tilemap::new(
            Vec2i(256, 256),
            (8, 8),
            &tileset,
            vec![
                6, 9, 0, 0, 0, 9, 0, 3,
                1, 9, 0, 0, 0, 9, 4, 4,
                6, 9, 0, 0, 0, 9, 4, 1,
                9, 9, 0, 0, 0, 9, 0, 0,
                0, 0, 0, 0, 0, 9, 6, 3,
                9, 9, 9, 9, 9, 9, 0, 2,
                2, 0, 1, 2, 0, 0, 3, 3,
                2, 3, 0, 0, 3, 0, 2, 0,
            ],
        );
        let map5 = Tilemap::new(
            Vec2i(0, 256 * 2),
            (8, 8),
            &tileset,
            vec![
                0, 3, 9, 0, 0, 9, 9, 9,
                2, 1, 9, 0, 0, 0, 0, 0,
                0, 1, 9, 0, 0, 0, 0, 0,
                3, 6, 9, 9, 9, 9, 9, 9,
                3, 2, 4, 7, 2, 3, 0, 0,
                1, 3, 1, 1, 6, 0, 0, 2,
                0, 7, 10, 0, 0, 2, 6, 6,
                0, 0, 2, 3, 0, 14, 1, 0,
            ],
        );
        let map6 = Tilemap::new(
            Vec2i(0, 256 * 3),
            (8, 8),
            &tileset,
            vec![
                3, 0, 0, 0, 3, 0, 1, 0,
                2, 1, 0, 0, 1, 6, 0, 1,
                1, 2, 4, 0, 2, 3, 1, 0,
                1, 3, 1, 1, 6, 10, 3, 2,
                0, 1, 10, 0, 4, 2, 6, 6,
                6, 3, 1, 1, 4, 0, 6, 10,
                0, 6, 2, 4, 2, 1, 0, 2,
                4, 0, 2, 1, 3, 3, 0, 1,
            ],
        );
        let map7 = Tilemap::new(
            Vec2i(256, 256 * 2),
            (8, 8),
            &tileset,
            vec![
                9, 9, 9, 9, 9, 9, 9, 9,
                0, 0, 0, 0, 0, 0, 0, 9,
                0, 0, 0, 0, 0, 0, 0, 9,
                9, 9, 9, 9, 9, 0, 0, 9,
                0, 2, 2, 3, 9, 9, 0, 9,
                3, 1, 0, 3, 6, 9, 0, 9,
                0, 0, 9, 9, 9, 9, 0, 9,
                1, 2, 9, 0, 0, 0, 0, 9,
            ],
        );
        let map8 = Tilemap::new(
            Vec2i(256, 256 * 3),
            (8, 8),
            &tileset,
            vec![
                4, 3, 9, 0, 0, 0, 0, 9,
                2, 0, 9, 0, 9, 9, 9, 9,
                0, 10, 9, 0, 9, 9, 9, 9,
                3, 3, 9, 0, 0, 0, 0, 0,
                0, 4, 9, 0, 0, 0, 0, 0,
                6, 6, 9, 9, 9, 9, 9, 9,
                0, 3, 0, 2, 2, 0, 1, 1,
                1, 3, 0, 1, 6, 0, 3, 0,
            ],
        );
        let map9 = Tilemap::new(
            Vec2i(256 * 2, 0),
            (8, 8),
            &tileset,
            vec![
                3, 0, 4, 0, 3, 0, 1, 0,
                2, 1, 0, 3, 1, 6, 0, 1,
                3, 2, 4, 7, 2, 2, 0, 0,
                1, 3, 1, 1, 6, 0, 0, 2,
                0, 9, 9, 0, 0, 2, 6, 6,
                6, 9, 9, 1, 4, 0, 1, 1,
                0, 9, 2, 4, 3, 0, 0, 0,
                0, 2, 2, 0, 3, 3, 0, 6,
            ],
        );
        let map10 = Tilemap::new(
            Vec2i(256 * 2, 256),
            (8, 8),
            &tileset,
            vec![
                3, 0, 4, 0, 3, 0, 1, 0,
                2, 1, 0, 0, 1, 6, 0, 1,
                3, 2, 4, 7, 2, 3, 0, 0,
                1, 3, 1, 1, 6, 0, 0, 2,
                0, 7, 10, 0, 0, 2, 6, 6,
                6, 7, 1, 1, 4, 0, 6, 10,
                0, 7, 7, 4, 3, 1, 0, 0,
                0, 0, 2, 1, 3, 3, 0, 4,
            ],
        );
        let map11 = Tilemap::new(
            Vec2i(256 * 2, 256 * 2),
            (8, 8),
            &tileset,
            vec![
                3, 0, 0, 0, 3, 0, 1, 0,
                2, 0, 3, 0, 0, 6, 0, 1,
                1, 2, 1, 3, 9, 9, 9, 0,
                1, 2, 1, 1, 4, 2, 9, 9,
                0, 2, 1, 31, 0, 0, 6, 6,
                6, 4, 1, 1, 4, 0, 6, 6,
                0, 2, 0, 3, 4, 4, 0, 0,
                1, 0, 2, 1, 3, 3, 0, 4,
            ],
        );
        let map12 = Tilemap::new(
            Vec2i(256 * 2, 256 * 3),
            (8, 8),
            &tileset,
            vec![
                4, 4, 6, 1, 0, 2, 0, 2,
                6, 4, 1, 1, 4, 0, 6, 6,
                9, 9, 9, 9, 9, 9, 9, 9,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                9, 9, 9, 9, 9, 9, 9, 9,
                1, 3, 2, 3, 2, 3, 2, 0,
                6, 6, 1, 1, 4, 0, 6, 10,
            ],
        );
        let map13 = Tilemap::new(
            Vec2i(256 * 3, 0),
            (8, 8),
            &tileset,
            vec![
                3, 0, 0, 0, 9, 0, 3, 0,
                2, 1, 0, 9, 9, 9, 0, 3,
                3, 2, 9, 9, 9, 9, 9, 2,
                1, 3, 9, 9, 9, 9, 9, 9,
                0, 2, 9, 9, 9, 9, 9, 6,
                3, 6, 1, 1, 9, 9, 9, 0,
                1, 0, 0, 9, 9, 9, 9, 6,
                0, 0, 1, 2, 2, 6, 0, 4,
            ],
        );
        let map14 = Tilemap::new(
            Vec2i(256 * 3, 256),
            (8, 8),
            &tileset,
            vec![
                3, 0, 0, 0, 3, 0, 1, 2,
                2, 1, 0, 3, 0, 6, 4, 1,
                1, 3, 2, 3, 2, 3, 2, 0,
                1, 3, 1, 23, 6, 0, 0, 4,
                0, 2, 10, 10, 0, 0, 6, 6,
                3, 10, 7, 7, 10, 0, 1, 10,
                0, 10, 7, 7, 10, 4, 1, 0,
                0, 0, 10, 10, 3, 3, 0, 4,
            ],
        );
        let map15 = Tilemap::new(
            Vec2i(256 * 3, 256 * 2),
            (8, 8),
            &tileset,
            vec![
                3, 0, 0, 0, 3, 0, 1, 0,
                2, 1, 0, 0, 0, 6, 0, 1,
                3, 2, 2, 3, 2, 3, 0, 0,
                1, 3, 1, 1, 6, 0, 0, 2,
                0, 2, 1, 0, 0, 0, 6, 6,
                6, 6, 1, 1, 4, 0, 6, 10,
                0, 0, 0, 4, 4, 4, 0, 0,
                0, 0, 2, 1, 3, 3, 0, 4,
            ],
        );
        let map16 = Tilemap::new(
            Vec2i(256 * 3, 256 * 3),
            (8, 8),
            &tileset,
            vec![
                7, 7, 7, 7, 7, 7, 7, 7,
                7, 4, 14, 21, 28, 35, 4, 7,
                9, 12, 6, 6, 6, 6, 31, 7,
                6, 19, 6, 8, 8, 6, 24, 7,
                6, 26, 6, 8, 8, 6, 23, 7,
                9, 33, 6, 6, 6, 6, 16, 7,
                7, 4, 34, 27, 20, 13, 4, 7,
                7, 7, 7, 7, 7, 7, 7, 7,
            ],
        );
        return vec![
            map, map2, map3, map4, map5, map6, map7, map8, map9, map10, map11, map12, map13, map14,
            map15, map16,
        ];
    }

    fn get_side_maps(tileset: &Rc<Tileset>) -> Vec<Tilemap> {
        return vec![Tilemap::new(
            Vec2i(0, 0),
            // Map size
            (60, 16),
            &tileset,
            // Tile grid
            vec![
                4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
                4, 5, 5, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5, 4, 4, 4, 4, 5, 8, 4, 5, 5, 5, 5, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 4, 4, 4, 4, 4, 4, 4,
                4, 5, 5, 4, 4, 4, 5, 5, 5, 5, 4, 4, 7, 7, 7, 7, 4, 5, 5, 5, 5, 5, 4, 5, 4, 4, 5, 4, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 4, 4, 4, 4, 4, 5, 5, 5, 4, 5, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4,
                4, 5, 5, 4, 4, 4, 5, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 4, 4, 4, 5, 5, 5, 4, 4, 5, 4, 4, 5, 4, 4, 4, 4, 4, 4, 5, 5, 4, 5, 5, 5, 5, 4, 4, 5, 5, 4, 4, 5, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
                4, 5, 5, 4, 4, 4, 5, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 4, 4, 4, 4, 4, 5, 4, 4, 5, 4, 4, 5, 5, 5, 4, 5, 5, 4, 4, 4, 4, 5, 4, 4, 5, 4, 4, 4, 5, 5, 5, 5, 8, 4, 4, 4, 4, 4, 4,
                4, 5, 5, 4, 4, 4, 5, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 4, 4, 4, 4, 4, 5, 4, 4, 5, 5, 5, 5, 5, 5, 4, 5, 5, 4, 4, 4, 4, 5, 4, 4, 5, 4, 4, 4, 4, 4, 5, 4, 4, 4, 4, 4, 4, 4, 4,
                4, 5, 5, 4, 5, 5, 5, 5, 5, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 4, 4, 4, 4, 4, 5, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 4, 4, 4, 4, 5, 4, 4, 5, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 3, 10, 4, 4,
                4, 5, 5, 4, 5, 5, 4, 4, 5, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 4, 4, 4, 4, 4, 5, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 4, 4, 4, 4, 5, 5, 5, 5, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 6, 2, 4, 4,
                4, 5, 5, 4, 5, 4, 4, 4, 5, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 8, 4, 4, 5, 4, 4, 4, 4, 4, 5, 5, 5, 4, 4, 5, 4, 4, 4, 4, 4, 5, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
                4, 5, 5, 4, 5, 4, 4, 4, 5, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 4, 4, 5, 4, 4, 4, 4, 4, 5, 5, 5, 4, 5, 5, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
                4, 5, 5, 4, 5, 5, 4, 4, 5, 4, 4, 4, 4, 4, 4, 5, 4, 4, 4, 7, 4, 7, 4, 7, 4, 4, 5, 4, 4, 4, 4, 4, 4, 5, 5, 4, 7, 7, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
                4, 5, 5, 4, 4, 5, 4, 4, 5, 4, 4, 4, 4, 4, 4, 5, 4, 4, 4, 7, 4, 7, 4, 7, 4, 4, 5, 4, 4, 4, 4, 4, 4, 5, 5, 5, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
                4, 5, 5, 5, 5, 5, 4, 4, 9, 9, 9, 9, 9, 9, 4, 5, 5, 5, 4, 7, 4, 7, 4, 7, 4, 4, 5, 4, 4, 4, 4, 4, 4, 5, 5, 5, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 4, 4, 5, 5, 5, 5, 6, 6, 6, 4,
                4, 5, 5, 4, 4, 4, 4, 4, 9, 9, 9, 9, 9, 9, 4, 4, 4, 5, 4, 7, 4, 7, 4, 7, 4, 4, 5, 4, 4, 4, 4, 4, 4, 5, 5, 5, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 4, 5, 4, 4, 4, 6, 6, 6, 4,
                4, 5, 5, 4, 4, 4, 4, 4, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 4, 7, 4, 7, 4, 7, 4, 4, 5, 14, 5, 18, 5, 27, 5, 35, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 9, 9, 9, 9, 4, 4, 4, 6, 6, 8, 4,
                4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
            ],
        )];
    }

    // Here's our game rules (the engine doesn't know about these)
    let levels: Vec<Level> = vec![
        (
            // level 0 is the side scroller
            // The map
            get_side_maps(&tileset),
            // Initial entities on level start
            vec![
                (Player::new(), 15, 29),
                (Player::new(), 9, 6),
                (Player::new(), 10, 5),
                (Player::new(), 11, 6),
            ],
        ),
        (
            // level 1 is the overworld map
            // The map
            get_maps(&overworld_tileset),
            // Initial entities on level start
            vec![
                (Player::new(), 10, 6),
                (Player::new(), 9, 6),
                (Player::new(), 10, 5),
                (Player::new(), 11, 6),
            ],
        ),
    ];
    let player_tex = rsrc.load_texture(Path::new("content/wiry_all_side.png"));
    let player_anim = Rc::new(Animation::new(
        vec![
            (
                Rect {
                    x: 0,
                    y: 0,
                    w: 32,
                    h: 32,
                },
                8,
            ),
            (
                Rect {
                    x: 32,
                    y: 0,
                    w: 32,
                    h: 32,
                },
                8,
            ),
            (
                Rect {
                    x: 64,
                    y: 0,
                    w: 32,
                    h: 32,
                },
                8,
            ),
        ],
        true,
    ));
    let enemy_tex = rsrc.load_texture(Path::new("content/game2_tilesheet.png"));
    let enemy_anim = Rc::new(Animation::freeze(Rect {
        x: 0,
        y: 32,
        w: 32,
        h: 32,
    }));
    let overworld_player_tex = rsrc.load_texture(Path::new("content/wiry_all_side.png"));
    let overworld_player_anim = Rc::new(Animation::new(
        vec![
            (
                Rect {
                    x: 0,
                    y: 0,
                    w: 32,
                    h: 32,
                },
                8,
            ),
            (
                Rect {
                    x: 32,
                    y: 0,
                    w: 32,
                    h: 32,
                },
                8,
            ),
            (
                Rect {
                    x: 64,
                    y: 0,
                    w: 32,
                    h: 32,
                },
                8,
            ),
        ],
        true,
    ));
    let background_tex = rsrc.load_texture(Path::new("content/badland_background.png"));
    // And here's our game state, which is just stuff that changes.
    // We'll say an entity is a type, a position, a velocity, a size, a texture, and an animation state.
    // State here will stitch them all together.

    let mut server = Server::new();
    server.connect("45.10.152.68:16512");
    let mut player = load("save1.json");
    player.id = server.id;

    let cam = Vec2i((player.pos.0 - WIDTH as i32 / 2).max(0), (player.pos.1 - HEIGHT as i32 / 2).max(0));
    let stack: Vec<Box<dyn State>> = vec![if player.world == 0 { Box::new(Title()) } else { Box::new(Scroll()) }];
    let level: usize = 1 - player.world as usize;
    let mut players = HashMap::<i32, Player>::new();
    players.entry(player.id).or_insert(player);

    let map_x_boundary = 1024 as i32;
    let map_y_boundary = 1024 as i32;

    let other_tileset = Rc::new(Tileset::new(
        vec![
                Tile { solid: false, jump_reset: false,restart:false },//0
                Tile { solid: false, jump_reset: false,restart:false },//1
                Tile { solid: false, jump_reset: false,restart:false },//2
                Tile { solid: false, jump_reset: false,restart:false },//3
                Tile { solid: false, jump_reset: false,restart:false },//4
                Tile { solid: false, jump_reset: false,restart:false },//5
                Tile { solid: false, jump_reset: false,restart:false },//6
                Tile { solid: true, jump_reset: false ,restart:false},//7
                Tile { solid: false, jump_reset: false,restart:false },//8
                Tile { solid: true, jump_reset: false ,restart:false},//9
                Tile { solid: false, jump_reset: false,restart:false },//10
                Tile { solid: false, jump_reset: false,restart:false },//11
                Tile { solid: false, jump_reset: false,restart:false },//12
                Tile { solid: false, jump_reset: false,restart:false },//13
                Tile { solid: false, jump_reset: false,restart:false },//14
                Tile { solid: false, jump_reset: false,restart:false },//15
                Tile { solid: false, jump_reset: false,restart:false },//16
                Tile { solid: false, jump_reset: false,restart:false },//17
                Tile { solid: false, jump_reset: false,restart:false },//18
                Tile { solid: false, jump_reset: false,restart:false },//19
                Tile { solid: false, jump_reset: false,restart:false },//20
                Tile { solid: false, jump_reset: false,restart:false },//21
                Tile { solid: false, jump_reset: false,restart:false },//22
                Tile { solid: false, jump_reset: false,restart:false },//23
                Tile { solid: false, jump_reset: false,restart:false },//24
                Tile { solid: false, jump_reset: false,restart:false },//25
                Tile { solid: false, jump_reset: false,restart:false },//26
                Tile { solid: false, jump_reset: false,restart:false },//27
                Tile { solid: false, jump_reset: false,restart:false },//28
                Tile { solid: false, jump_reset: false,restart:false },//29
                Tile { solid: false, jump_reset: false,restart:false },//30
                Tile { solid: false, jump_reset: false,restart:false },//31
                Tile { solid: false, jump_reset: false,restart:false },//32
                Tile { solid: false, jump_reset: false,restart:false },//33
                Tile { solid: false, jump_reset: false,restart:false },//34
                Tile { solid: false, jump_reset: false,restart:false },//35
        ],
        &rsrc.load_texture(Path::new("content/game2_tilesheet.png")),
    ));

    let font_data: &[u8] = include_bytes!("../../content/helvetica.ttf");
    let font: Font<'static> = Font::try_from_bytes(font_data).unwrap();


    let game = GameState {
        // Every entity has a position, a size, a texture, and animation state.
        // Assume entity 0 is the player
        server,
        players,
        sizes: vec![(32, 32)],
        // Could be texture handles instead, let's talk about that in two weeks
        textures: vec![
            Rc::clone(&player_tex),
            Rc::clone(&enemy_tex),
            Rc::clone(&background_tex),
            Rc::clone(&overworld_player_tex),
        ],
        anim_state: vec![
            player_anim.start(),
            enemy_anim.start(),
            overworld_player_anim.start(),
        ],
        // Current level
        level,
        // Camera position
        camera: cam,
        // background position
        background_pos: Vec2i(0, 0),
        state_stack: stack,
        game_data: GameData {
            score: 0,
            speed_multiplier: 1,
            num_jumps: 0,
            portals: vec![(Vec2i(736, 256), Vec2i(672, 32)), (Vec2i(1696, 128), Vec2i(1856, 448))],
            restart: false,
        },
        map_x_boundary,
        map_y_boundary,
        tt_tileset: overworld_tileset,
        maps: get_maps(&other_tileset),
        side_map: get_side_maps(&tileset),
        // font,
        game: 1,
        spawn_point: Vec2i(50, 50),
        texts_overworld:vec![
            Text::new(Vec2i(230,75),"Use arrow keys",&font,25.0),
            Text::new(Vec2i(260,100),"to move",&font,25.0),
            Text::new(Vec2i(800,768),"Try to press P",&font,25.0)
        ],
        texts_sidescroll:vec![
            Text::new(Vec2i(300,150),"Don't touch the lava",&font,25.0),
            Text::new(Vec2i(880,400),"Walls are nice",&font,25.0),
        ]
    };

    let state = Game2DEngine::run(
        WIDTH,
        HEIGHT,
        window_builder,
        rsrc,
        levels,
        game,
        draw_game,
        update_game,
    );
    save(&state.players[&state.server.id], "save1.json");
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
