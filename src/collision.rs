use crate::tiles::*;
use crate::types::*;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ColliderID {
    Dynamic(usize),
    Tile(Tile, Rect),
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Contact {
    a: ColliderID,
    b: ColliderID,
    mtv: (i32, i32),
}
#[allow(dead_code)]
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

fn get_map_indx(tl: Vec2i, tr: Vec2i, br: Vec2i, bl: Vec2i, maps: &Vec<Tilemap>) -> usize{
    let mut result = 0;
    for (mi, m) in maps.iter().enumerate(){
        // println!("cur pos {:?}", pos);
        // println!("map pos {:?}", m.position);
        if (tl.0 <= m.position.0 + 256 && tl.1 <= m.position.1 + 256) &&
            (tr.0 <= m.position.0 + 256 && tr.1 <= m.position.1 + 256) &&
            (br.0 <= m.position.0 + 256 && br.1 <= m.position.1 + 256) &&
            (bl.0 <= m.position.0 + 256 && bl.1 <= m.position.1 + 256) {
            result = mi;
            break
        }
    }
    result
}

// Here we will be using push() on into, so it can't be a slice
pub fn gather_contacts(
    positions: &[Vec2i],
    sizes: &[(usize, usize)],
    tilemap: &Vec<Tilemap>,
    into: &mut Vec<Contact>,
    num_jumps: &mut usize,
    restart: &mut bool,
) {
    // collide mobiles against mobiles
    for (ai, (apos, asize)) in (positions.iter().zip(sizes.iter())).enumerate() {
        for (bi, (bpos, bsize)) in (positions.iter().zip(sizes.iter()))
            .enumerate()
            .skip(ai + 1)
        {
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
        let map_indx = get_map_indx(tl, tr, br, bl, &tilemap);
        // println!("map indx: {:}", map_indx);
        let map = &tilemap[map_indx];
        let (ttl, tlrect) = match map.tile_and_bounds_at(tl) {
            Some((ttl, tlrect)) => (ttl, tlrect),
            _ => (Tile{solid: false, jump_reset: false, restart: false}, Rect{x: 0, y: 0, w: 0, h: 0}),
        };
        let (ttr, trrect) = match map.tile_and_bounds_at(tr) {
            Some((ttr, trrect)) => (ttr, trrect),
            _ => (Tile{solid: false, jump_reset: false, restart: false}, Rect{x: 0, y: 0, w: 0, h: 0}),
        };
        let (btl, blrect) = match map.tile_and_bounds_at(bl) {
            Some((btl, blrect)) => (btl, blrect),
            _ => (Tile{solid: false, jump_reset: false, restart: false}, Rect{x: 0, y: 0, w: 0, h: 0}),
        };
        let (btr, brrect) = match map.tile_and_bounds_at(br) {
            Some((btr, brrect)) => (btr, brrect),
            _ => (Tile{solid: false, jump_reset: false, restart: false}, Rect{x: 0, y: 0, w: 0, h: 0}),
        };
        // println!("touching top left  {:?}", (ttl, tlrect));
        // println!("touching bottom right  {:?}", (btr, brrect));

        let sprite_rect = Rect {
            x: pos.0,
            y: pos.1,
            w: size.0 as u16,
            h: size.1 as u16,
        };
        if ttl.solid {
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

        if ttl.solid {
            // println!("jump reset");
        }
        if ttr.solid {
            // println!("touching top right");
            // println!("jump reset ttr");
        }
        if btl.jump_reset {
            // println!("touching bottom left");
            // println!("jump reset btl");
            *num_jumps = 0;
        }
        if btr.jump_reset {
            // println!("touching buttom right");
            // println!("jump reset btr");
            *num_jumps = 0;
        }

        if ttl.restart {
            *restart = true;
        }
    }
}

#[allow(unused_variables)]
pub fn restitute(
    positions: &mut [Vec2i],
    sizes: &[(usize, usize)],
    velocities: &mut [Vec2i],
    camera: &mut Vec2i,
    tilemap: &Vec<Tilemap>,
    contacts: &mut [Contact],
) {
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
                let mut a_rect = Rect {
                    x: positions[ai].0,
                    y: positions[ai].1,
                    w: sizes[ai].0 as u16,
                    h: sizes[ai].1 as u16,
                };
                let b_rect = Rect {
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
                        a_rect.x -= horizontal_mtv;
                    // disp[player_indx] = (horizontal_mtv, 0)
                    } else {
                        a_rect.x += horizontal_mtv;
                        // disp[player_indx] = (horizontal_mtv, 0)
                    }
                } else {
                    if a_rect.y < b_rect.y {
                        // println!("Box touched down side{:?}", c);
                        a_rect.y -= vertical_mtv;
                    // disp[player_indx] = (0, vertical_mtv)
                    } else {
                        // println!("Box touched up side{:?}", c);
                        a_rect.y += vertical_mtv;
                        // disp[player_indx] = (0, vertical_mtv)
                    }
                }
            }
            (ColliderID::Dynamic(ai), ColliderID::Tile(_bt, br)) => {
                // println!("INSIDE Dynamic tile case");
                // let horizontal_mtv = c.mtv.0;
                // let vertical_mtv = c.mtv.1;
                let a_rect = Rect {
                    x: positions[ai].0,
                    y: positions[ai].1,
                    w: sizes[ai].0 as u16,
                    h: sizes[ai].1 as u16,
                };
                if let Some((horizontal_mtv, vertical_mtv)) = rect_displacement(a_rect, br) {
                    if horizontal_mtv < vertical_mtv {
                        if a_rect.x < br.x {
                            // println!("Box touched left side{:?}", c);
                            positions[ai].0 -= horizontal_mtv;
                            velocities[ai].0 = 0;
                            camera.0 -= horizontal_mtv;
                        //
                        // disp[player_indx] = (horizontal_mtv, 0)
                        } else {
                            // println!("Box touched right side{:?}", c);
                            positions[ai].0 += horizontal_mtv;
                            velocities[ai].0 = 0;
                            camera.0 += horizontal_mtv;
                            // camera.0 = a_rect.x;
                            // disp[player_indx] = (horizontal_mtv, 0)
                        }
                    } else {
                        if a_rect.y < br.y {
                            // println!("Box touched down side{:?}", c);
                            positions[ai].1 -= vertical_mtv;
                            velocities[ai].1 = 0;
                            camera.1 -= vertical_mtv;
                        // camera.1 = a_rect.y;
                        // disp[player_indx] = (0, vertical_mtv)
                        } else {
                            // println!("Box touched up side{:?}", c);
                            positions[ai].1 += vertical_mtv;
                            velocities[ai].1 = 0;
                            camera.1 += vertical_mtv;
                            // camera.1 = a_rect.y;
                            // disp[player_indx] = (0, vertical_mtv)
                        }
                    }
                }
                // if disp.0 == horizontal_mtv || disp.1 == vertical_mtv {
                //     continue;
                // }
            }
            _ => {}
        };
    }
}
