use crate::{element::{Cell, Element, Action, air_element, fire_element}, chunk::{Grid, in_bound, DirtyRect}, chunk_manager::WorldChunks, base_movement::{get_wanted_chunk, get_new_element_coord, set, get}};

pub fn handle_actions(future_grid: &mut Grid, i: usize, j: usize, chunks: &mut WorldChunks, index: (i32, i32), keep_active: &mut bool, dirty_rect: &mut DirtyRect) {
    match future_grid[i][j].action {
        Some(action) => 'action: {
            match action {
                Action::Burn => {
                    if future_grid[i][j].lifetime == -1 {
                        future_grid[i][j].lifetime = 400;
                        future_grid[i][j].color[0] = future_grid[i][j].color[0] / 2;
                        future_grid[i][j].color[1] = future_grid[i][j].color[1] / 2;
                        future_grid[i][j].color[2] = future_grid[i][j].color[2] / 2;
                    } else if future_grid[i][j].lifetime < 0 {
                        future_grid[i][j] = air_element();
                        break 'action;
                    }

                    let rand = fastrand::i32(0..8);

                    *keep_active = true;
                    dirty_rect.set_temp(i, j);

                    future_grid[i][j].lifetime -= rand;
                    if future_grid[i][j].lifetime == -1 {
                        future_grid[i][j].lifetime -= 1;
                    }

                    match rand {
                        0 => if get(i as i32, j as i32 - 1,  future_grid, chunks, index).element == Element::Air {
                            set(i as i32, j as i32 - 1,  future_grid, chunks, index, fire_element());
                        },
                        1 => if get(i as i32 + 1, j as i32,  future_grid, chunks, index).element == Element::Air {
                            set(i as i32 + 1, j as i32,  future_grid, chunks, index, fire_element());
                        },
                        2 => if get(i as i32, j as i32 + 1,  future_grid, chunks, index).element == Element::Air {
                            set(i as i32, j as i32 + 1,  future_grid, chunks, index, fire_element());
                        },
                        3 => if get(i as i32 - 1, j as i32,  future_grid, chunks, index).element == Element::Air {
                            set(i as i32 - 1, j as i32,  future_grid, chunks, index, fire_element());
                        },
                        _ => ()
                    }
                }
            }
        },
        _ => ()
    }
}

pub fn is_flammable(cell: &Cell) -> bool {
    match cell.element {
        Element::Wood | Element::SawDust => true,
        _ => false
    }
}

pub fn set_action(i: i32, j: i32, f_grid: &mut Grid, chunks: &mut WorldChunks, index: (i32, i32), action: Option<Action>) {
	if in_bound(i, j) {
		f_grid[i as usize][j as usize].action = action;
	} else {
		let wanted_chunk = get_wanted_chunk(index, i, j);
		
		if chunks.contains_key(&wanted_chunk) {
			let (x, y) = get_new_element_coord(i, j);
			chunks.get_mut(&wanted_chunk).unwrap().grid[x as usize][y as usize].action = action;
		}
	}
}