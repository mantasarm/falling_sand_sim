use crate::{phys_world::element::*, phys_world::chunk::{Grid, in_bound, MovData}, phys_world::base_movement::*};

use super::chunk;

pub fn handle_actions(future_grid: &mut Grid, i: usize, j: usize, mov_dt: &mut MovData) {
    match future_grid[i][j].action {
        Some(action) => 'action: {
            match action {
                Action::Burn => {
                    let (lifetime, burn_element, emit_fire, darken) = get_flammable_info(&future_grid[i][j].element);
                    if future_grid[i][j].lifetime == -1 {
                        future_grid[i][j].lifetime = lifetime;
                        if darken {
                            future_grid[i][j].color[0] /= 2;
                            future_grid[i][j].color[1] /= 2;
                            future_grid[i][j].color[2] /= 2;
                            chunk::update_byte(&mut mov_dt.bytes, i, j, &future_grid[i][j].color);
                        }
                    } else if future_grid[i][j].lifetime < 0 {
                        future_grid[i][j] = burn_element;
                        break 'action;
                    }

                    let rand = fastrand::i32(0..8);

                    *mov_dt.keep_active = true;
                    mov_dt.dirty_rect.set_temp(i, j);

                    future_grid[i][j].lifetime -= rand;
                    if future_grid[i][j].lifetime == -1 {
                        future_grid[i][j].lifetime -= 1;
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
            }
        },
        _ => ()
    }
}

pub fn is_flammable(cell: &Cell) -> bool {
    matches!(cell.element, Element::Wood | Element::SawDust | Element::Coal | Element::Methane | Element::Water  | Element::Petrol)
}

pub fn get_flammable_info(element: &Element) -> (i32, Cell, bool, bool) {
    match element {
        Element::Wood => (300, air_element(), true, true),
        Element::Coal => (400, smoke_element(), true, true),
        Element::SawDust => (215, air_element(), true, true),
        Element::Methane => (0, fire_element(), true, false),
        Element::Water => (-1, steam_element(), false, false),
        Element::Petrol => (80, fire_element(), true, false),
        _ => (0, air_element(), false, false)
    }
}

pub fn set_action(i: i32, j: i32, f_grid: &mut Grid, mov_dt: &mut MovData, action: Option<Action>) {
	if in_bound(i, j) {
		f_grid[i as usize][j as usize].action = action;
	} else {
		let wanted_chunk = get_wanted_chunk(mov_dt.index, i, j);
		
		if mov_dt.chunks.contains_key(&wanted_chunk) {
			let (x, y) = get_new_element_coord(i, j);
			mov_dt.chunks.get_mut(&wanted_chunk).unwrap().grid[x as usize][y as usize].action = action;
		}
	}
}
