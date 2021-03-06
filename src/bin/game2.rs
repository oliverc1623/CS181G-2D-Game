use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;
use rusttype::Font;
use winit::dpi::LogicalSize;

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
            .with_title("Adventure of Wiry")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .with_resizable(false)
    };
    // Here's our resources...
    let mut rsrc = Resources::new();
    let tileset = Rc::new(Tileset::new(
        vec![
            Tile {solid: false,jump_reset: false,restart: false},//0
            Tile {solid: false,jump_reset: false,restart: false},//1
            Tile {solid: false,jump_reset: false,restart: false},//2
            Tile {solid: false,jump_reset: false,restart: true},//3
            Tile {solid: false,jump_reset: false,restart: false},//4
            Tile {solid: false,jump_reset: false,restart: false},//5
            Tile {solid: false,jump_reset: false,restart: false},//6
            Tile {solid: false,jump_reset: false,restart: false},//7
            Tile {solid: true, jump_reset: true,restart: false},//8
            Tile {solid: false,jump_reset: false,restart: false},//9
            Tile {solid: false,jump_reset: false,restart: false},//10
            Tile {solid: false,jump_reset: false,restart: false},//11
            Tile {solid: false,jump_reset: false,restart: false},//12
            Tile {solid: false,jump_reset: false,restart: false},//13
            Tile {solid: false,jump_reset: false,restart: false},//14
            Tile {solid: false,jump_reset: false, restart: true},//15
        ],
        &rsrc.load_texture(Path::new("content/tilesheet.png")),
    ));
    // overworld tileset
    let overworld_tileset = Rc::new(Tileset::new(
        vec![
            Tile {solid: false,jump_reset: false,restart: false},//0
            Tile {solid: false,jump_reset: false,restart: false},//1
            Tile {solid: false,jump_reset: false,restart: false},//2
            Tile {solid: false,jump_reset: false,restart: false},//3
            Tile {solid: false,jump_reset: false,restart: false},//4
            Tile {solid: false,jump_reset: false,restart: false},//5
            Tile {solid: false,jump_reset: false,restart: false},//6
            Tile {solid: false,jump_reset: false,restart: false},//7
            Tile {solid: false,jump_reset: false,restart: false},//8
            Tile {solid: false,jump_reset: false,restart: false},//9
            Tile {solid: true,jump_reset: false,restart: false},//10
            Tile {solid: false,jump_reset: false,restart: false},//11
            Tile {solid: false,jump_reset: false,restart: false},//12
            Tile {solid: false,jump_reset: false,restart: false},//13
            Tile {solid: false,jump_reset: false,restart: false},//14
            Tile {solid: false,jump_reset: false, restart: true},//15
        ],
        &rsrc.load_texture(Path::new("content/tilesheet.png")),
    ));

    pub fn get_maps(tileset: &Rc<Tileset>) -> Vec<Tilemap> {
        let map = Tilemap::new(
            Vec2i(0, 0),
            (8, 8),
            &tileset,
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
        );
        let map2 = Tilemap::new(
            Vec2i(256, 0),
            (8, 8),
            &tileset,
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
        );
        let map3 = Tilemap::new(
            Vec2i(0, 256),
            (8, 8),
            &tileset,
            vec![
                8, 8, 8, 8, 8, 8, 8, 8,
                8, 8, 8, 8, 8, 8, 8, 8,
                8, 8, 8, 8, 8, 8, 8, 8,
                8, 8, 8, 9, 9, 8, 9, 8,
                8, 8, 8, 0, 0, 8, 8, 8,
                8, 8, 8, 0, 0, 8, 8, 8,
                8, 8, 8, 8, 8, 9, 8, 8,
                8, 8, 8, 9, 8, 8, 8, 8,
            ],
        );
        let map4 = Tilemap::new(
            Vec2i(256, 256),
            (8, 8),
            &tileset,
            vec![
                8, 8, 8, 8, 8, 12, 8, 8,
                8, 8, 8, 0, 8, 8, 8, 8,
                8, 8, 8, 0, 8, 8, 8, 8,
                8, 12, 8, 0, 9, 8, 8, 8,
                8, 8, 8, 0, 2, 2, 2, 8,
                8, 8, 8, 8, 8, 8, 8, 8,
                8, 8, 8, 12, 8, 8, 8, 8,
                8, 8, 8, 8, 8, 8, 8, 8,
            ],
        );
        let map5 = Tilemap::new(
            Vec2i(0, 256 * 2),
            (8, 8),
            &tileset,
            vec![
                8, 8, 8, 8, 8, 8, 8, 8,
                8, 12, 8, 12, 8, 8, 8, 8,
                8, 8, 8, 8, 8, 12, 8, 8,
                12, 8, 8, 9, 8, 8, 8, 8,
                8, 8, 12, 0, 8, 12, 8, 8,
                8, 8, 8, 8, 8, 8, 8, 8,
                8, 8, 12, 8, 8, 8, 8, 8,
                12, 8, 8, 8, 12, 8, 8, 8,
            ],
        );
        let map6 = Tilemap::new(
            Vec2i(0, 256 * 3),
            (8, 8),
            &tileset,
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
        );
        let map7 = Tilemap::new(
            Vec2i(256, 256 * 2),
            (8, 8),
            &tileset,
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
        );
        let map8 = Tilemap::new(
            Vec2i(256, 256 * 3),
            (8, 8),
            &tileset,
            vec![
                8, 8, 8, 8, 8, 8, 8, 8,
                8, 8, 12, 8, 0, 8, 8, 8,
                8, 8, 8, 8, 0, 8, 8, 8,
                8, 8, 8, 8, 0, 8, 8, 8,
                8, 0, 2, 2, 2, 8, 8, 8,
                8, 0, 8, 8, 8, 8, 8, 8,
                8, 8, 8, 8, 8, 8, 12, 8,
                8, 8, 8, 12, 8, 8, 8, 8,
            ],
        );
        let map9 = Tilemap::new(
            Vec2i(256 * 2, 0),
            (8, 8),
            &tileset,
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
        );
        let map10 = Tilemap::new(
            Vec2i(256 * 2, 256),
            (8, 8),
            &tileset,
            vec![
                8, 8, 8, 8, 8, 8, 8, 8,
                8, 4, 8, 8, 8, 8, 5, 8,
                8, 8, 8, 8, 8, 8, 8, 8,
                8, 8, 8, 12, 12, 8, 8, 8,
                8, 8, 8, 12, 12, 8, 8, 8,
                8, 8, 8, 8, 8, 8, 8, 8,
                8, 7, 8, 8, 8, 8, 6, 8,
                8, 8, 8, 8, 8, 8, 8, 8,
            ],
        );
        let map11 = Tilemap::new(
            Vec2i(256 * 2, 256 * 2),
            (8, 8),
            &tileset,
            vec![
                8, 8, 8, 8, 8, 8, 8, 8,
                8, 8, 8, 8, 8, 8, 8, 8,
                8, 8, 8, 8, 12, 8, 8, 8,
                8, 8, 8, 12, 10, 12, 8, 8,
                8, 8, 8, 8, 12, 8, 8, 8,
                8, 8, 8, 8, 8, 8, 8, 8,
                8, 8, 8, 8, 8, 8, 8, 8,
                8, 8, 8, 8, 8, 8, 8, 8,
            ],
        );
        let map12 = Tilemap::new(
            Vec2i(256 * 2, 256 * 3),
            (8, 8),
            &tileset,
            vec![
                8, 8, 8, 8, 8, 8, 8, 8,
                9, 9, 9, 9, 9, 9, 9, 9,
                2, 2, 2, 2, 2, 2, 2, 2,
                9, 9, 9, 9, 9, 9, 9, 9,
                8, 8, 8, 8, 8, 8, 8, 8,
                8, 8, 8, 12, 12, 8, 8, 8,
                8, 8, 8, 8, 8, 8, 8, 8,
                8, 8, 8, 8, 8, 8, 8, 8,
            ],
        );
        let map13 = Tilemap::new(
            Vec2i(256 * 3, 0),
            (8, 8),
            &tileset,
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
        );
        let map14 = Tilemap::new(
            Vec2i(256 * 3, 256),
            (8, 8),
            &tileset,
            vec![
                8, 8, 8, 8, 8, 8, 8, 8,
                8, 8, 8, 8, 8, 8, 8, 8,
                8, 8, 9, 9, 9, 9, 8, 8,
                8, 8, 12, 8, 8, 12, 8, 8,
                8, 8, 9, 9, 9, 9, 8, 8,
                8, 8, 8, 8, 8, 8, 8, 8,
                8, 8, 8, 8, 12, 8, 8, 8,
                8, 8, 7, 8, 8, 8, 8, 8,
            ],
        );
        let map15 = Tilemap::new(
            Vec2i(256 * 3, 256 * 2),
            (8, 8),
            &tileset,
            vec![
                8, 8, 8, 8, 8, 8, 8, 8,
                8, 8, 8, 8, 0, 0, 8, 8,
                8, 8, 8, 0, 0, 0, 8, 8,
                8, 8, 0, 0, 0, 0, 8, 8,
                8, 8, 8, 0, 0, 8, 8, 8,
                8, 8, 8, 8, 0, 8, 8, 8,
                8, 8, 12, 8, 9, 9, 9, 8,
                8, 8, 8, 8, 6, 8, 8, 8,
            ],
        );
        let map16 = Tilemap::new(
            Vec2i(256 * 3, 256 * 3),
            (8, 8),
            &tileset,
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
        );
        return vec![
            map, map2, map3, map4, map5, map6, map7, map8, map9, map10, map11, map12, map13, map14,
            map15, map16,
        ];
    }

    fn get_side_maps(tileset: &Rc<Tileset>) -> Vec<Tilemap>{
        return vec![Tilemap::new(
            Vec2i(0, 0),
            // Map size
            (130, 54),
            &tileset,
            // Tile grid
            vec![8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,8,8,8,8,8,8,8,8,8,8,8,8,8,1,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,8,8,1,1,1,1,1,1,1,1,1,1,1,1,8,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 8,1,1,1,1,1,1,1,1,8,8,1,1,1,1,1,8,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,8,8,8,8,8,8,8,8,1,8,8,8,8,8,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,8,1,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,8,1,1,1,1,1,8,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 8,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,8,1,1,1,1,1,8,1,1,1,8,1,1,8,8,8,8,8,8,8,8,8,1,1,8,1,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 8,1,1,5,1,1,1,1,1,1,1,1,1,8,1,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,8,1,1,1,1,1,8,1,1,1,8,1,1,8,1,1,1,1,1,1,1,8,1,1,8,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 8,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,5,1,8,0,0,8,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,8,1,1,1,1,1,8,1,1,1,8,1,1,8,1,1,1,1,1,1,1,8,1,1,8,1,8,1,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 8,1,1,1,1,1,1,1,1,1,1,8,1,1,1,1,1,1,8,0,0,8,8,1,1,1,8,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,1,1,8,1,1,1,8,1,1,1,1,1,8,1,1,1,8,1,1,8,1,1,1,1,1,1,1,8,1,1,8,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 8,1,1,1,1,1,1,1,1,1,8,1,1,1,1,1,1,1,8,0,0,8,8,1,1,1,8,8,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,8,1,1,1,1,1,8,1,1,1,8,1,1,1,1,1,1,1,1,1,8,1,1,8,1,1,1,8,1,1,1,8,1,1,8,1,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 8,1,1,1,1,1,1,1,1,8,1,1,1,1,1,1,1,1,8,0,0,8,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,8,1,1,1,1,1,8,1,1,1,8,1,1,1,1,1,1,1,1,1,8,1,1,8,1,8,1,8,1,1,1,8,1,1,8,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 8,1,1,8,8,8,1,1,1,8,1,1,4,1,1,1,1,1,8,0,0,8,8,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,8,1,1,1,1,8,1,1,1,1,1,8,1,1,1,8,1,1,1,1,1,1,1,1,1,8,1,1,8,1,1,1,8,1,1,1,8,1,1,8,1,8,1,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 8,1,1,1,1,1,1,1,1,8,1,1,1,1,1,1,1,1,8,0,0,8,8,1,1,1,1,1,1,1,1,1,1,8,1,8,1,1,1,1,1,1,1,8,1,1,1,1,1,8,1,1,1,8,1,1,1,1,1,1,1,1,1,8,1,1,8,1,1,1,8,1,1,1,8,1,1,8,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,5,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 8,2,2,2,2,2,2,2,2,8,1,1,1,1,1,1,1,1,8,0,0,8,8,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,1,1,1,1,8,1,1,1,1,1,8,1,1,1,8,1,1,1,1,1,1,1,1,1,8,1,1,8,1,1,1,8,1,1,1,8,1,1,8,1,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,1,1,1,1,1,8,1,1,1,8,1,1,1,1,1,1,1,1,1,8,1,1,8,1,1,1,8,1,1,1,8,1,1,8,1,8,1,1,1,1,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,1,1,8,1,1,1,8,1,1,1,1,1,1,1,1,1,8,1,1,8,1,1,1,8,1,1,1,8,1,1,8,1,8,1,1,1,1,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,1,1,8,1,1,1,8,1,1,1,1,8,1,1,1,1,8,1,1,8,1,1,1,8,1,1,1,8,1,1,8,1,8,1,1,1,1,1,1,1,8,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,8,1,1,1,8,1,1,1,1,1,1,1,1,8,1,1,1,1,8,1,1,8,1,1,1,8,1,1,1,8,1,1,8,1,8,1,1,1,1,1,1,1,8,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,2,2,1,1,1,1,1,1,8,1,1,1,8,1,8,1,1,1,1,1,1,1,1,8,1,1,1,1,8,1,1,8,1,1,1,8,1,1,1,8,1,1,8,1,8,1,1,1,8,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,2,1,1,2,1,1,1,1,1,8,1,8,1,1,1,8,1,1,1,1,1,1,1,1,8,1,1,1,1,8,1,1,8,1,1,1,8,1,1,1,8,1,1,8,0,8,1,1,1,8,1,1,1,8,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,2,1,1,1,1,2,1,1,1,1,8,1,1,8,1,1,8,1,1,1,8,1,1,1,1,8,1,1,1,1,8,1,1,8,1,1,1,8,1,1,1,8,1,1,8,0,8,1,1,1,8,1,1,1,1,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 1,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,1,1,1,1,2,1,1,1,1,1,1,2,1,1,1,8,1,1,1,8,1,8,1,1,1,8,1,1,1,1,8,1,1,1,1,8,1,1,8,8,1,8,8,1,1,1,8,1,1,8,0,8,1,4,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,2,1,1,1,1,1,1,1,1,2,1,1,8,1,1,8,1,1,8,1,1,1,8,1,1,1,1,8,1,1,1,1,8,1,1,8,1,1,1,8,1,1,1,8,1,1,8,1,8,1,1,1,8,1,1,1,1,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,0,1,1,2,2,1,1,2,2,1,1,0,1,8,1,8,1,1,1,8,1,1,1,1,1,1,1,1,8,1,1,1,1,8,1,1,8,1,1,1,8,1,1,1,8,1,1,8,1,8,1,1,1,8,1,1,1,8,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,0,1,1,1,1,1,1,1,1,1,1,0,1,8,1,1,1,1,1,8,1,1,1,1,1,1,1,1,8,1,1,1,1,8,1,1,8,1,1,1,8,1,1,1,8,1,1,8,1,8,1,1,1,8,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,0,1,1,1,1,1,1,1,1,1,1,0,1,8,1,1,1,1,1,8,1,1,1,1,1,1,1,1,8,1,1,1,1,8,1,1,8,1,1,1,8,1,1,1,8,1,1,8,1,8,1,1,1,8,1,1,1,1,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,0,1,1,1,1,1,1,1,1,1,1,0,1,8,1,1,1,1,1,8,1,1,1,1,1,1,1,1,8,1,1,1,1,8,1,1,8,1,1,1,8,1,1,1,8,1,1,8,1,8,1,1,1,8,1,1,1,1,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,1,1,1,1,1,1,1,8,1,1,0,1,1,1,2,1,1,2,1,1,1,0,1,8,1,1,1,1,1,8,1,1,1,1,1,1,1,1,8,1,1,1,1,8,1,1,8,1,1,1,8,1,1,1,8,1,1,8,1,8,1,1,1,8,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 1,8,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,1,1,8,1,1,1,1,1,1,1,8,1,1,0,1,1,1,1,2,2,1,1,1,1,0,1,8,1,1,1,1,1,8,1,1,1,1,1,1,1,1,8,1,1,1,1,8,1,1,8,1,1,1,8,1,1,1,8,1,1,8,1,8,1,1,1,8,1,1,1,8,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,2,1,1,1,1,1,1,1,1,2,1,1,8,1,1,8,1,1,8,1,1,1,1,1,1,1,1,8,1,1,1,1,8,1,1,8,1,1,1,8,1,1,1,8,1,1,8,1,8,1,1,1,8,1,1,1,1,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 1,8,1,1,1,1,1,1,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,1,2,1,1,1,1,1,1,2,1,1,1,8,1,1,8,1,1,8,1,1,1,8,1,1,1,1,8,8,1,1,1,8,1,1,8,1,1,1,8,1,1,1,8,1,1,8,1,8,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,1,1,2,1,1,1,1,2,1,1,1,1,8,1,1,8,1,1,8,1,1,1,8,1,1,1,1,1,1,1,1,1,8,1,1,8,1,4,1,8,1,1,1,8,1,1,8,1,8,1,1,1,8,1,1,1,1,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 1,8,1,1,1,1,1,1,1,8,1,1,1,1,1,1,1,1,1,1,8,1,1,1,1,1,1,8,1,1,1,1,1,1,2,1,1,2,1,1,1,1,1,8,1,1,8,1,1,8,1,1,1,8,1,1,1,1,1,1,1,1,1,8,1,1,8,1,1,1,8,1,1,1,8,1,1,8,1,8,1,1,1,8,1,1,1,8,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,1,1,1,1,2,2,1,1,1,1,1,1,8,1,1,8,1,1,8,1,1,1,8,1,1,1,1,1,1,1,1,1,8,1,1,8,1,1,1,8,1,1,1,8,1,1,8,1,8,1,1,1,8,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 1,8,1,1,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,8,1,1,8,1,1,1,8,1,1,1,1,1,1,1,1,1,8,1,1,8,1,1,1,8,1,1,1,8,1,1,8,1,8,1,1,1,8,1,1,1,1,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,5,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,8,1,1,8,1,1,1,8,1,1,1,1,1,1,1,1,1,8,1,1,8,1,1,1,8,1,1,1,8,1,1,8,0,8,1,1,1,8,1,1,1,1,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 1,8,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,8,8,8,1,8,1,1,1,8,1,1,1,1,8,1,1,1,1,8,1,1,8,1,1,1,8,1,1,1,8,1,1,8,0,8,1,1,1,1,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,1,1,8,1,1,1,1,1,1,1,1,8,1,1,1,1,8,1,1,8,1,1,1,8,1,1,1,8,1,1,8,0,8,1,1,1,1,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 1,8,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,1,1,8,1,1,1,1,1,1,1,1,8,1,1,1,1,8,1,1,8,1,1,1,8,1,1,1,8,1,1,8,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,4,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,1,1,8,1,1,1,1,1,1,1,1,8,1,1,1,1,8,1,1,8,1,1,1,8,1,1,1,8,1,1,8,1,8,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,
                 1,8,8,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,8,8,8,8,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,1,1,8,1,1,8,8,1,1,1,1,8,1,1,1,1,8,1,1,8,1,1,1,1,1,1,1,8,1,1,8,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,1,1,8,1,1,1,1,1,1,1,1,8,1,1,1,1,8,1,1,8,3,3,3,1,1,1,1,8,1,1,8,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,1,1,8,1,1,1,8,1,1,1,1,8,1,1,1,1,8,1,1,1,1,1,1,3,1,1,1,3,1,1,8,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,1,1,8,1,1,1,8,1,1,1,1,8,1,1,1,1,8,1,1,1,1,1,1,3,1,1,1,3,1,1,8,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,1,1,8,1,1,1,8,1,1,1,1,8,1,1,1,1,8,1,1,1,1,1,1,3,1,1,1,3,1,1,8,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,1,1,8,1,1,1,8,1,1,1,1,8,1,1,1,1,8,1,1,1,1,1,1,3,1,5,1,3,1,1,8,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,1,1,8,1,1,1,8,1,1,1,1,8,1,1,1,1,8,1,1,1,1,1,1,3,1,1,1,3,1,1,8,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,1,1,8,1,1,1,8,1,1,1,1,8,1,1,1,1,8,1,1,1,1,1,1,1,3,3,3,1,1,1,8,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,1,1,8,1,1,1,8,1,1,1,1,8,1,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,1,1,8,8,8,8,8,8,8,8,1,8,1,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,1,1,1,1,8,1,1,1,1,1,1,1,1,1,1,1,1,1,8,4,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,5,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                 1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,3,3,3,3,8,1,1,1,1,1,1,1,1,1,1,1,1,1,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]
        )]
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
    let enemy_tex = rsrc.load_texture(Path::new("content/tilesheet.png"));
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
    server.connect("45.10.152.68:32306");
    let mut player = load("save2.json");
    player.id = server.id;

    let cam = Vec2i((player.pos.0 - WIDTH as i32 / 2).max(0), (player.pos.1 - HEIGHT as i32 / 2).max(0));
    let stack: Vec<Box<dyn State>> = vec![if player.world == 0 { Box::new(Title()) } else { Box::new(Scroll()) }];
    let level:usize = 1 - player.world as usize;
    let mut players = HashMap::<i32, Player>::new();
    players.entry(player.id).or_insert(player);

    let mut map_x_boundary = 1024 as i32;
    let mut map_y_boundary = 1024 as i32;

    let other_tileset = Rc::new(Tileset::new(
        vec![
            Tile {solid: false,jump_reset: false,restart: false},//0
            Tile {solid: false,jump_reset: false,restart: false},//1
            Tile {solid: false,jump_reset: false,restart: false},//2
            Tile {solid: false,jump_reset: false,restart: false},//3
            Tile {solid: false,jump_reset: false,restart: false},//4
            Tile {solid: false,jump_reset: false,restart: false},//5
            Tile {solid: false,jump_reset: false,restart: false},//6
            Tile {solid: false,jump_reset: false,restart: false},//7
            Tile {solid: false,jump_reset: false,restart: false},//8
            Tile {solid: false,jump_reset: false,restart: false},//9
            Tile {solid: true,jump_reset: false,restart: false},//10
            Tile {solid: false,jump_reset: false,restart: false},//11
            Tile {solid: false,jump_reset: false,restart: false},//12
            Tile {solid: false,jump_reset: false,restart: false},//13
            Tile {solid: false,jump_reset: false,restart: false},//14
            Tile {solid: false,jump_reset: false, restart: true},//15
        ],
        &rsrc.load_texture(Path::new("content/tilesheet.png")),
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
        level: 1 - 1,
        // Camera position
        camera: cam,
        // background position
        background_pos: Vec2i(0, 0),
        state_stack: stack,
        game_data: GameData {
            score: 0,
            speed_multiplier: 1,
            num_jumps: 0,
            portals: vec![(Vec2i(26*32, 40*32), Vec2i(3*32, 8*32)), (Vec2i(3008,416), Vec2i(68*32,30*32)), (Vec2i(2306, 1412), Vec2i(79*32, 50*32)), (Vec2i(3328, 1664), Vec2i(87*32, 20*32))],
            restart: false,
        },
        map_x_boundary,
        map_y_boundary,
        tt_tileset: overworld_tileset,
        maps: get_maps(&other_tileset),
        side_map: get_side_maps(&tileset),
        // font,
        game: 2, 
        spawn_point: Vec2i(3*32, 37*32),
        texts_overworld:vec![
            Text::new(Vec2i(150,75),"Use arrow keys",&font,25.0),
            Text::new(Vec2i(150,100),"to move",&font,25.0),
            Text::new(Vec2i(400,400),"Try to press P",&font,25.0)
        ],
        texts_sidescroll:vec![
            Text::new(Vec2i(26*32-100, 40*32-25),"Come here",&font,25.0),
            Text::new(Vec2i(880,400),"You can wall climb",&font,25.0),
            Text::new(Vec2i(114*32,33*32),"You won!",&font,25.0),
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
    save(&state.players[&state.server.id],"save2.json");
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
