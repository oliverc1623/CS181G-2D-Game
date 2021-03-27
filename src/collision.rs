use crate::types::*;
use crate::tiles::*;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ColliderID {
    Dynamic(usize),
    Tile(Tile, Rect),
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Contact{
    a: ColliderID,
    b: ColliderID,
    mtv: (i32, i32),
}

// // pixels gives us an rgba8888 framebuffer
// fn clear(fb: &mut [u8], c: Color) {
//     // Four bytes per pixel; chunks_exact_mut gives an iterator over 4-element slices.
//     // So this way we can use copy_from_slice to copy our color slice into px very quickly.
//     for px in fb.chunks_exact_mut(4) {
//         px.copy_from_slice(&c);
//     }
// }

// #[allow(dead_code)]
// fn rect(fb: &mut [u8], r: Rect, c: Color) {
//     assert!(r.x < WIDTH as i32);
//     assert!(r.y < HEIGHT as i32);
//     // NOTE, very fragile! will break for out of bounds rects!  See next week for the fix.
//     let x1 = (r.x + r.w as i32).min(WIDTH as i32) as usize;
//     let y1 = (r.y + r.h as i32).min(HEIGHT as i32) as usize;
//     for row in fb[(r.y as usize * PITCH)..(y1 * PITCH)].chunks_exact_mut(PITCH) {
//         for p in row[(r.x as usize * DEPTH)..(x1 * DEPTH)].chunks_exact_mut(DEPTH) {
//             p.copy_from_slice(&c);
//         }
//     }
// }

fn rect_touching(r1: Rect, r2: Rect) -> bool {
    // r1 left is left of r2 right
    r1.x <= r2.x+r2.w as i32 &&
        // r2 left is left of r1 right
        r2.x <= r1.x+r1.w as i32 &&
        // those two conditions handle the x axis overlap;
        // the next two do the same for the y axis:
        r1.y <= r2.y+r2.h as i32 &&
        r2.y <= r1.y+r1.h as i32
}

pub fn rect_displacement(r1: Rect, r2: Rect) -> Option<(i32, i32)> {
    // Draw this out on paper to double check, but these quantities
    // will both be positive exactly when the conditions in rect_touching are true.
    let x_overlap = (r1.x + r1.w as i32).min(r2.x + r2.w as i32) - r1.x.max(r2.x);
    let y_overlap = (r1.y + r1.h as i32).min(r2.y + r2.h as i32) - r1.y.max(r2.y);
    if x_overlap >= 0 && y_overlap >= 0 {
        // This will return the magnitude of overlap in each axis.
        Some((x_overlap, y_overlap))
    } else {
        None
    }
}

// Here we will be using push() on into, so it can't be a slice
pub fn gather_contacts(positions: &[Vec2i], sizes: &[(usize,usize)], tilemap: &[&Tilemap], into: &mut Vec<Contact>) {
    // collide mobiles against mobiles
    for (ai, (apos, asize)) in (positions.iter().zip(sizes.iter())).enumerate() {
        for (bi, (bpos, bsize)) in (positions.iter().zip(sizes.iter())).enumerate().skip(ai + 1) {
            let arect = Rect {
                x: apos.0,
                y: apos.1,
                w: asize.0 as u16,
                h: asize.1 as u16,
            };
            let brect = Rect {
                x: bpos.0,
                y: bpos.1,
                w: bsize.0 as u16,
                h: bsize.1 as u16,
            };
            if let Some(disp) = rect_displacement(arect, brect) {
                into.push(Contact {
                    a: ColliderID::Dynamic(ai),
                    b: ColliderID::Dynamic(bi),
                    mtv: disp,
                });
            }
        }
    }
    // collide tiles
    for (ei, (pos, size)) in (positions.iter().zip(sizes.iter())).enumerate() {
        // get corner positions
        let tl = Vec2i(pos.0, pos.1);
        let tr = Vec2i(pos.0 + size.0 as i32, pos.1);
        let br = Vec2i(pos.0 + size.0 as i32, pos.1 + size.0 as i32);
        let bl = Vec2i(pos.0, pos.1 + size.0 as i32);

        // let map = &levels[_game.level].0;
        let map = &tilemap[0];
        let (ttl, tlrect) = map.tile_and_bounds_at(tl);
        let (ttr, trrect) = map.tile_and_bounds_at(tr);
        let (btl, blrect) = map.tile_and_bounds_at(bl);
        let (btr, brrect) = map.tile_and_bounds_at(br);

        let sprite_rect = Rect {
            x: pos.0,
            y: pos.1,
            w: size.0 as u16,
            h: size.1 as u16,
        };
        if ttl.solid {
            println!("touching top left");
            if let Some(displacement) = rect_displacement(sprite_rect, tlrect) {
                // make contact out of displacment
                // define contact
                into.push(Contact {
                    a: ColliderID::Dynamic(ei),
                    b: ColliderID::Tile(ttl, tlrect),
                    mtv: displacement,
                });
            }
        }
        if ttr.solid {
            println!("touching top right");
            if let Some(displacement) = rect_displacement(sprite_rect, trrect) {
                // make contact out of displacment
                // define contact
                into.push(Contact {
                    a: ColliderID::Dynamic(ei),
                    b: ColliderID::Tile(ttr, trrect),
                    mtv: displacement,
                });
            }
        }
        if btl.solid {
            println!("touching bottom left");
            if let Some(displacement) = rect_displacement(sprite_rect, blrect) {
                // make contact out of displacment
                // define contact
                into.push(Contact {
                    a: ColliderID::Dynamic(ei),
                    b: ColliderID::Tile(btl, blrect),
                    mtv: displacement,
                });
            }
        }
        if btr.solid {
            println!("touching buttom right");
            if let Some(displacement) = rect_displacement(sprite_rect, brrect) {
                // make contact out of displacment
                // define contact
                into.push(Contact {
                    a: ColliderID::Dynamic(ei),
                    b: ColliderID::Tile(btr, brrect),
                    mtv: displacement,
                });
            }
        }
    }
}

pub fn restitute(positions: &mut [Vec2i], sizes: &[(usize,usize)], velocities: &mut [Vec2i], camera: &mut Vec2i, tilemap: &[&Tilemap], contacts: &mut [Contact]) {
    // handle restitution of dynamics against dynamics and dynamics against statics wrt contacts.
    // You could instead make contacts `Vec<Contact>` if you think you might remove contacts.
    // You could also add an additional parameter, a slice or vec representing how far we've displaced each dynamic, to avoid allocations if you track a vec of how far things have been moved.
    // You might also want to pass in another &mut Vec<Contact> to be filled in with "real" touches that actually happened.
    contacts.sort_unstable_by_key(|c| -(c.mtv.0 * c.mtv.0 + c.mtv.1 * c.mtv.1));
    // Keep going!  Note that you can assume every contact has a dynamic object in .a.
    // You might decide to tweak the interface of this function to separately take dynamic-static and dynamic-dynamic contacts, to avoid a branch inside of the response calculation.
    // Or, you might decide to calculate signed mtvs taking direction into account instead of the unsigned displacements from rect_displacement up above.  Or calculate one MTV per involved entity, then apply displacements to both objects during restitution (sorting by the max or the sum of their magnitudes)
    // let mut disp = vec![(0, 0); dynamics.len()];
    // fix double-restitution issue
    for c in contacts.iter() {
        // let mtv = i32::min(c1.mtv.0, c1.mtv.1);
        // let mut collided_wall: <Wall>
        match (c.a, c.b) {
            (ColliderID::Dynamic(ai), ColliderID::Dynamic(bi)) => {
                // horizontal < vertical
                let horizontal_mtv = c.mtv.0;
                let vertical_mtv = c.mtv.1;
                let mut a_rect = Rect{ 
                    x: positions[ai].0,
                    y: positions[ai].1,
                    w: sizes[ai].0 as u16,
                    h: sizes[ai].1 as u16, 
                };
                let mut b_rect = Rect{ 
                    x: positions[bi].0,
                    y: positions[bi].1,
                    w: sizes[bi].0 as u16,
                    h: sizes[bi].1 as u16, 
                };
                let disp = rect_displacement(a_rect, b_rect).unwrap();
                if disp.0 == horizontal_mtv || disp.1 == vertical_mtv {
                    continue;
                }
                if horizontal_mtv < vertical_mtv {
                    if a_rect.x < b_rect.x {
                        println!("Box touched left side{:?}", c);
                        a_rect.x -= horizontal_mtv;
                        // disp[player_indx] = (horizontal_mtv, 0)
                    } else {
                        println!("Box touched right side{:?}", c);
                        a_rect.x += horizontal_mtv;
                        // disp[player_indx] = (horizontal_mtv, 0)
                    }
                } else {
                    if a_rect.y < b_rect.y {
                        println!("Box touched down side{:?}", c);
                        a_rect.y -= vertical_mtv;
                        // disp[player_indx] = (0, vertical_mtv)
                    } else {
                        println!("Box touched up side{:?}", c);
                        a_rect.y += vertical_mtv;
                        // disp[player_indx] = (0, vertical_mtv)
                    }
                }
            },
            (ColliderID::Dynamic(ai), ColliderID::Tile(bt, br)) => {
                println!("INSIDE Dynamic tile case");
                // let horizontal_mtv = c.mtv.0;
                // let vertical_mtv = c.mtv.1;
                let mut a_rect = Rect{ 
                    x: positions[ai].0,
                    y: positions[ai].1,
                    w: sizes[ai].0 as u16,
                    h: sizes[ai].1 as u16, 
                };
                if let Some((horizontal_mtv, vertical_mtv)) = rect_displacement(a_rect, br) {
                    if horizontal_mtv < vertical_mtv {
                        if a_rect.x < br.x {
                            println!("Box touched left side{:?}", c);
                            positions[ai].0 -= horizontal_mtv;
                            velocities[ai].0 = 0;
                            camera.0 += velocities[ai].0;                            
                            // 
                            // disp[player_indx] = (horizontal_mtv, 0)
                        } else {
                            println!("Box touched right side{:?}", c);
                            a_rect.x += horizontal_mtv;
                            velocities[ai].0 = 0;
                            camera.0 += velocities[ai].0;
                            // camera.0 = a_rect.x;
                            // disp[player_indx] = (horizontal_mtv, 0)
                        }
                    } else {
                        if a_rect.y < br.y {
                            println!("Box touched down side{:?}", c);
                            positions[ai].1 -= vertical_mtv;
                            velocities[ai].1 = 0;
                            camera.1 += velocities[ai].1;
                            // camera.1 = a_rect.y;
                            // disp[player_indx] = (0, vertical_mtv)
                        } else {
                            println!("Box touched up side{:?}", c);
                            positions[ai].1 += vertical_mtv;
                            velocities[ai].1 = 0;
                            camera.1 += velocities[ai].1;
                            // camera.1 = a_rect.y;
                            // disp[player_indx] = (0, vertical_mtv)
                        }
                    }          
                }
                // if disp.0 == horizontal_mtv || disp.1 == vertical_mtv {
                //     continue;
                // }
                      
            },
            _ => {}
        };
        
    }
}


// fn main() {
//     let event_loop = EventLoop::new();
//     let mut input = WinitInputHelper::new();
//     let window = {
//         let size = PhysicalSize::new(WIDTH as f64, HEIGHT as f64);
//         WindowBuilder::new()
//             .with_title("Collision2D")
//             .with_inner_size(size)
//             .with_min_inner_size(size)
//             .with_resizable(false)
//             .build(&event_loop)
//             .unwrap()
//     };
//     let mut pixels = {
//         let window_size = window.inner_size();
//         let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
//         Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture).unwrap()
//     };
//     let mut player = Mobile {
//         rect: Rect {
//             x: 32,
//             y: HEIGHT as i32 - 16 - 8,
//             w: 8,
//             h: 8,
//         },
//         vx: 0,
//         vy: 0,
//     };
//     let walls = [
//         Wall {
//             rect: Rect {
//                 x: 0,
//                 y: 0,
//                 w: WIDTH as u16,
//                 h: 16,
//             },
//         },
//         Wall {
//             rect: Rect {
//                 x: 0,
//                 y: 0,
//                 w: 16,
//                 h: HEIGHT as u16,
//             },
//         },
//         Wall {
//             rect: Rect {
//                 x: WIDTH as i32 - 16,
//                 y: 0,
//                 w: 16,
//                 h: HEIGHT as u16,
//             },
//         },
//         Wall {
//             rect: Rect {
//                 x: 0,
//                 y: HEIGHT as i32 - 16,
//                 w: WIDTH as u16,
//                 h: 16,
//             },
//         },
//         Wall {
//             rect: Rect {
//                 x: WIDTH as i32 / 2 - 16,
//                 y: HEIGHT as i32 / 2 - 16,
//                 w: 32,
//                 h: 32,
//             },
//         },
//     ];
//     // How many frames have we simulated?
//     let mut frame_count: usize = 0;
//     // How many unsimulated frames have we saved up?
//     let mut available_time = 0.0;
//     // Track beginning of play
//     let start = Instant::now();
//     let mut contacts: Vec<Contact> = vec![];
//     let mut mobiles = [player];
//     // Track end of the last frame
//     let mut since = Instant::now();
//     event_loop.run(move |event, _, control_flow| {
//         // Draw the current frame
//         if let Event::RedrawRequested(_) = event {
//             let fb = pixels.get_frame();
//             clear(fb, CLEAR_COL);
//             // Draw the walls
//             for w in walls.iter() {
//                 rect(fb, w.rect, WALL_COL);
//             }
//             // Draw the player
//             rect(fb, mobiles[0].rect, PLAYER_COL);
//             // Flip buffers
//             if pixels.render().is_err() {
//                 *control_flow = ControlFlow::Exit;
//                 return;
//             }

//             // Rendering has used up some time.
//             // The renderer "produces" time...
//             available_time += since.elapsed().as_secs_f64();
//         }
//         // Handle input events
//         if input.update(event) {
//             // Close events
//             if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
//                 *control_flow = ControlFlow::Exit;
//                 return;
//             }

//             // Resize the window if needed
//             if let Some(size) = input.window_resized() {
//                 pixels.resize(size.width, size.height);
//             }
//         }
//         // And the simulation "consumes" it
//         while available_time >= DT {
//             let player = &mut mobiles[0];
//             // Eat up one frame worth of time
//             available_time -= DT;

//             // println!("{}", available_time);
//             // Player control goes here; determine player acceleration
//             if input.key_held(VirtualKeyCode::Up) || input.quit() {
//                 // println!("pressing key");
//                 player.vy -= 1;
//                 return;
//             }
//             if input.key_held(VirtualKeyCode::Down) || input.quit() {
//                 player.vy += 1;
//                 return;
//             }
//             if input.key_held(VirtualKeyCode::Left) || input.quit() {
//                 player.vx -= 1;
//                 return;
//             }
//             if input.key_held(VirtualKeyCode::Right) || input.quit() {
//                 player.vx += 1;
//                 return;
//             }

//             // Determine player velocity
//             // Update player position
//             player.rect.x += player.vx/2;
//             player.rect.y += player.vy/2;

//             // Detect collisions: Generate contacts
//             contacts.clear();
//             gather_contacts(&walls, &mobiles, &mut contacts);

//             // Handle collisions: Apply restitution impulses.
//             restitute(&walls, &mut mobiles, &mut contacts);

//             // Update game rules: What happens when the player touches things?

//             // Increment the frame counter
//             frame_count += 1;
//         }
//         // Request redraw
//         window.request_redraw();
//         // When did the last frame end?
//         since = Instant::now();
//     });
// }

// Items (in this case a module) tagged with cfg(test) are only compiled
// in the test profile (e.g., `cargo test`)
// #[cfg(test)]
// mod tests {
//     // Bring in definitions from above
//     use super::*;
//     // Project out just the colliders from a slice of contacts
//     // This way we can write terse tests without worrying about displacements
//     fn contact_pairs(cs: &[Contact]) -> Vec<(ColliderID, ColliderID)> {
//         cs.iter().map(|c| (c.a, c.b)).collect()
//     }
//     const HW: u16 = WIDTH as u16 / 2;
//     const HH: u16 = HEIGHT as u16 / 2;
//     // Set up a level to use for unit tests.
//     // This one is a room with four walls, each of which is split into two halves.  There's also a square in the middle.
//     const LEVEL: [Wall; 9] = [
//         Wall {
//             rect: Rect {
//                 x: 0,
//                 y: 0,
//                 w: HW,
//                 h: 16,
//             },
//         },
//         Wall {
//             rect: Rect {
//                 x: HW as i32,
//                 y: 0,
//                 w: HW,
//                 h: 16,
//             },
//         },
//         Wall {
//             rect: Rect {
//                 x: 0,
//                 y: HEIGHT as i32 - 16,
//                 w: HW,
//                 h: 16,
//             },
//         },
//         Wall {
//             rect: Rect {
//                 x: HW as i32,
//                 y: HEIGHT as i32 - 16,
//                 w: HW,
//                 h: 16,
//             },
//         },
//         Wall {
//             rect: Rect {
//                 x: 0,
//                 y: 0,
//                 w: 16,
//                 h: HH,
//             },
//         },
//         Wall {
//             rect: Rect {
//                 x: 0,
//                 y: HH as i32,
//                 w: 16,
//                 h: HH,
//             },
//         },
//         Wall {
//             rect: Rect {
//                 x: WIDTH as i32 - 16,
//                 y: 0,
//                 w: 16,
//                 h: HH,
//             },
//         },
//         Wall {
//             rect: Rect {
//                 x: WIDTH as i32 - 16,
//                 y: HH as i32,
//                 w: 16,
//                 h: HH,
//             },
//         },
//         Wall {
//             rect: Rect {
//                 x: HW as i32 - 16,
//                 y: HH as i32 - 16,
//                 w: 32,
//                 h: 32,
//             },
//         },
//     ];
// Functions annotated `test` are unit test functions and are run automatically
// during `cargo test`
// #[test]
// fn test_collision_bl_br() {
//     let mut player = [Mobile {
//         rect: Rect {
//             x: 17,
//             y: HEIGHT as i32 - 16 - 8 - 1,
//             w: 8,
//             h: 8,
//         },
//         vx: 0,
//         vy: 0,
//     }];
//     let mut cs = vec![];
//     gather_contacts(&LEVEL, &player, &mut cs);
//     assert_eq!(cs, vec![]);
//     player[0].rect.x = 16;
//     // Slide the player down-rightly across the level
//     for _step in 0..((HW - 16 - 8 - 2) / 2) {
//         player[0].vx = 2;
//         player[0].vy = 2;
//         player[0].rect.x += player[0].vx;
//         player[0].rect.y += player[0].vy;
//         cs.clear();
//         gather_contacts(&LEVEL, &player, &mut cs);
//         assert_eq!(
//             contact_pairs(&cs),
//             vec![(ColliderID::Dynamic(0), ColliderID::Static(2))],
//             "{:?}",
//             player[0].rect
//         );
//         restitute(&LEVEL, &mut player, &mut cs);
//         assert!(player[0].rect.x < HW as i32 - 8);
//         assert_eq!(player[0].rect.y, HEIGHT as i32 - 16 - 8);
//     }
//     assert_eq!(player[0].rect.x, HW as i32 - 8 - 2);
//     // For some period they'll be touching both halves of the bottom wall
//     for _step in 0..5 {
//         player[0].vx = 2;
//         player[0].vy = 2;
//         let lx = player[0].rect.x;
//         player[0].rect.x += player[0].vx;
//         player[0].rect.y += player[0].vy;
//         cs.clear();
//         gather_contacts(&LEVEL, &player, &mut cs);
//         assert_eq!(
//             contact_pairs(&cs),
//             vec![
//                 (ColliderID::Dynamic(0), ColliderID::Static(2)),
//                 (ColliderID::Dynamic(0), ColliderID::Static(3))
//             ],
//             "{:?}",
//             player[0].rect
//         );
//         restitute(&LEVEL, &mut player, &mut cs);
//         assert!(player[0].rect.x <= HW as i32);
//         assert!(lx < player[0].rect.x);
//         assert_eq!(player[0].rect.y, HEIGHT as i32 - 16 - 8);
//     }
//     assert_eq!(player[0].rect.x, HW as i32);
//     // Then just the right half
//     for _step in 0..((HW - 16 - 8) / 2 - 1) {
//         player[0].vx = 2;
//         player[0].vy = 2;
//         player[0].rect.x += player[0].vx;
//         player[0].rect.y += player[0].vy;
//         cs.clear();
//         gather_contacts(&LEVEL, &player, &mut cs);
//         assert_eq!(
//             contact_pairs(&cs),
//             vec![(ColliderID::Dynamic(0), ColliderID::Static(3))],
//             "{:?}",
//             player[0].rect
//         );
//         restitute(&LEVEL, &mut player, &mut cs);
//         assert!(player[0].rect.x < WIDTH as i32 - 16 - 8);
//         assert_eq!(player[0].rect.y, HEIGHT as i32 - 16 - 8);
//     }
//     // And then the right half as well as the bottom half of the right wall
//     player[0].vx = 2;
//     player[0].vy = 2;
//     player[0].rect.x += player[0].vx;
//     player[0].rect.y += player[0].vy;
//     cs.clear();
//     gather_contacts(&LEVEL, &player, &mut cs);
//     assert_eq!(
//         contact_pairs(&cs),
//         vec![
//             (ColliderID::Dynamic(0), ColliderID::Static(3)),
//             (ColliderID::Dynamic(0), ColliderID::Static(7)),
//         ],
//         "{:?}",
//         player[0].rect
//     );
//     restitute(&LEVEL, &mut player, &mut cs);
//     assert_eq!(player[0].rect.x, WIDTH as i32 - 16 - 8);
//     assert_eq!(player[0].rect.y, HEIGHT as i32 - 16 - 8);
// }
// #[test]
// fn move_up_left() {
//     let mut player = [Mobile {
//         rect: Rect {
//             x: 17,
//             y: 17,
//             w: 8,
//             h: 8,
//         },
//         vx: -2,
//         vy: -2,
//     }];
//     let mut cs = vec![];
//     player[0].rect.x -= 2;
//     player[0].rect.y -= 2;
//     gather_contacts(&LEVEL, &player, &mut cs);
//     assert_eq!(
//         contact_pairs(&cs),
//         vec![
//             (ColliderID::Dynamic(0), ColliderID::Static(0)),
//             (ColliderID::Dynamic(0), ColliderID::Static(4))
//         ]
//     );
//     restitute(&LEVEL, &mut player, &mut cs);
//     assert_eq!(player[0].rect.x, 16);
//     assert_eq!(player[0].rect.y, 16);
// }
// You can add your own unit tests too, for example moving up and
// right along the right wall or moving around the central square
