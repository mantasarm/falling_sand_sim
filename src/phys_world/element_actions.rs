use crate::{phys_world::element::*, phys_world::chunk::{Grid, in_bound, MovData}, phys_world::base_movement::*};

use super::chunk;

pub fn handle_actions(future_grid: &mut Grid, i: usize, j: usize, mov_dt: &mut MovData, frame_count: u128) {
    match future_grid[i][j].action {
        Some(action) => 'action: {
            match action {
                Action::Burn => {
                    let (lifetime, burn_element, emit_fire, darken, light_other) = get_flammable_info(&future_grid[i][j].element);
                    if future_grid[i][j].lifetime == -1 {
                        future_grid[i][j].lifetime = lifetime;
                        if darken {
                            future_grid[i][j].color[0] /= 2;
                            future_grid[i][j].color[1] /= 2;
                            future_grid[i][j].color[2] /= 2;
                            chunk::update_byte(&mut mov_dt.bytes, i, j, &future_grid[i][j].color);
                        }
                    } else if future_grid[i][j].lifetime < 0 && future_grid[i][j].lifetime != -100 {
                        set(i as i32, j as i32, future_grid, mov_dt, burn_element);
                        *mov_dt.keep_active = true;
                        mov_dt.dirty_rect.set_temp(i, j);
                        *mov_dt.colliders_dirty = true;
                        break 'action;
                    }

                    *mov_dt.keep_active = true;
                    mov_dt.dirty_rect.set_temp(i, j);

                    // INFO: We do this instead of fastrand for performace reasons
                    let rand = (i as u128 + j as u128 + frame_count) as i32 % 10;
                    if future_grid[i][j].lifetime != -100 {
                        future_grid[i][j].lifetime -= rand;
                        if future_grid[i][j].lifetime == -1 {
                            future_grid[i][j].lifetime -= 1;
                        }
                    }

                    if light_other {
                        spread_fire(future_grid, i, j, mov_dt);
                    }

                    if emit_fire {
                        match rand {
                            0 => if get(i as i32, j as i32 - 1,  future_grid, mov_dt).element == Element::Air {
                                set(i as i32, j as i32 - 1,  future_grid, mov_dt, fire_element());
                            },
                            1 => if get(i as i32 + 1, j as i32,  future_grid, mov_dt).element == Element::Air {
                                set(i as i32 + 1, j as i32,  future_grid, mov_dt, fire_element());
                            },
                            2 => if get(i as i32, j as i32 + 1,  future_grid, mov_dt).element == Element::Air {
                                set(i as i32, j as i32 + 1,  future_grid, mov_dt, fire_element());
                            },
                            3 => if get(i as i32 - 1, j as i32,  future_grid, mov_dt).element == Element::Air {
                                set(i as i32 - 1, j as i32,  future_grid, mov_dt, fire_element());
                            },
                            _ => ()
                        }
                    }
                }
                Action::EmitSource(emit_element) => {
                    let up = get(i as i32, j as i32 - 1,  future_grid, mov_dt);
                    let down = get(i as i32, j as i32 + 1,  future_grid, mov_dt);
                    let left = get(i as i32 - 1, j as i32,  future_grid, mov_dt);
                    let right = get(i as i32 + 1, j as i32,  future_grid, mov_dt);
                    match emit_element {
                        Element::Air => {
                            if up.state != State::Solid {
                                future_grid[i][j].action = Some(Action::EmitSource(up.element));
                            } else if down.state != State::Solid {
                                future_grid[i][j].action = Some(Action::EmitSource(down.element));
                            } else if left.state != State::Solid {
                                future_grid[i][j].action = Some(Action::EmitSource(left.element));
                            } else if right.state != State::Solid {
                                future_grid[i][j].action = Some(Action::EmitSource(right.element));
                            }
                        },
                        _ => {
                            if up.state == State::Gas {
                                set(i as i32, j as i32 - 1,  future_grid, mov_dt, el_from_enum(emit_element));
                            }
                            if down.state == State::Gas {
                                set(i as i32, j as i32 + 1,  future_grid, mov_dt, el_from_enum(emit_element));
                            }
                            if left.state == State::Gas {
                                set(i as i32 - 1, j as i32,  future_grid, mov_dt, el_from_enum(emit_element));
                            }
                            if right.state == State::Gas {
                                set(i as i32 + 1, j as i32,  future_grid, mov_dt, el_from_enum(emit_element));
                            }
                        }
                    }
                },
                Action::Grow => {
                    let up = get(i as i32, j as i32 - 1,  future_grid, mov_dt);
                    let down = get(i as i32, j as i32 + 1,  future_grid, mov_dt);
                    let left = get(i as i32 - 1, j as i32,  future_grid, mov_dt);
                    let right = get(i as i32 + 1, j as i32,  future_grid, mov_dt);

                    let up_right = get(i as i32 + 1, j as i32 - 1,  future_grid, mov_dt);
                    let up_left = get(i as i32 - 1, j as i32 - 1,  future_grid, mov_dt);
                    let down_right = get(i as i32 + 1, j as i32 + 1,  future_grid, mov_dt);
                    let down_left = get(i as i32 - 1, j as i32 + 1,  future_grid, mov_dt);

                    let mut active = false;
                    if left.element == Element::Air && down_left.element == Element::SolidDirt {
                        set(i as i32 - 1, j as i32, future_grid, mov_dt, grass_element());
                        active = true;
                    }
                    if right.element == Element::Air && down_right.element == Element::SolidDirt {
                        set(i as i32 + 1, j as i32, future_grid, mov_dt, grass_element());
                        active = true;
                    }
                    if down.element == Element::SolidDirt {
                        if down_right.element == Element::Air {
                            set(i as i32 + 1, j as i32 + 1, future_grid, mov_dt, grass_element());
                            active = true;
                        }
                        if down_left.element == Element::Air {
                            set(i as i32 - 1, j as i32 + 1, future_grid, mov_dt, grass_element());
                            active = true;
                        }
                    }
                    if down.element == Element::Air &&
                       (down_right.element == Element::SolidDirt || down_left.element == Element::SolidDirt) {
                            set(i as i32, j as i32 + 1, future_grid, mov_dt, grass_element());
                            active = true;
                    }
                    if up.element == Element::Air &&
                       (up_right.element == Element::SolidDirt || up_left.element == Element::SolidDirt) {
                            set(i as i32, j as i32 - 1, future_grid, mov_dt, grass_element());
                            active = true;
                    }
                    if right.element == Element::SolidDirt && up_right.element == Element::Air {
                        set(i as i32 + 1, j as i32 - 1, future_grid, mov_dt, grass_element());
                        active = true;
                    }
                    if left.element == Element::SolidDirt && up_left.element == Element::Air {
                        set(i as i32 - 1, j as i32 - 1, future_grid, mov_dt, grass_element());
                        active = true;
                    }

                    let mut height = 1;
                    let growth_chance = fastrand::f32();
                    while growth_chance < 1 as f32 / (height as f32).powf(2.) {
                        if get(i as i32, j as i32 - height,  future_grid, mov_dt).element != Element::Air || height >= 6 {
                            break;
                        }
                        let mut grass_el = grass_element();
                        grass_el.action = None;
                        set(i as i32, j as i32 - height, future_grid, mov_dt, grass_el.clone());
                        height += 1;
                    }

                    if !active {
                        future_grid[i][j].action = None;
                    }

                    *mov_dt.keep_active = true;
                    mov_dt.dirty_rect.set_temp(i, j);
                }
            }
        },
        _ => ()
    }
}

pub fn is_flammable(cell: &Cell) -> bool {
    matches!(cell.element, Element::Wood | Element::SawDust | Element::Coal
                            | Element::Methane | Element::Water  | Element::Petrol | Element::Grass
                            | Element::Snow | Element::Ice)
}

// INFO: Lifetime -1 burns up immediately, -100 burns forever
pub fn get_flammable_info(element: &Element) -> (i32, Cell, bool, bool, bool) {
    match element {
        Element::Wood => (300, air_element(), true, true, false),
        Element::Coal => (400, smoke_element(), true, true, false),
        Element::SawDust => (215, air_element(), true, true, false),
        Element::Methane => (0, fire_element(), true, false, false),
        Element::Water => (-1, steam_element(), false, false, false),
        Element::Petrol => (80, fire_element(), true, false, false),
        Element::Lava => (-100, fire_element(), true, false, true),
        Element::Grass => (2, fire_element(), true, true, false),
        Element::Snow => (-1, water_element(), false, false, false),
        Element::Ice => (-1, water_element(), false, false, false),
        _ => (0, air_element(), false, false, false)
    }
}

pub fn set_action(i: i32, j: i32, f_grid: &mut Grid, mov_dt: &mut MovData, action: Option<Action>) {
	if in_bound(i, j) {
		f_grid[i as usize][j as usize].action = action;
	} else {
		let wanted_chunk = get_wanted_chunk(mov_dt.index, i, j);
		
		match mov_dt.chunks.get_mut(&wanted_chunk) {
		    Some(chunk) => {
		        let (x, y) = get_new_element_coord(i, j);
		        chunk.grid[x as usize][y as usize].action = action;
		    },
		    _ => ()
		}
	}
}
