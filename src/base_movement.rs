use notan::math::Vec2;

use crate::{element::{Cell, State, solid_element}, chunk::{ROWS, COLS, in_bound, self, Grid, DirtyRect}, chunk_manager::WorldChunks};

pub fn downward(f_grid: &mut Grid, i: usize, j: usize, chunks: &mut WorldChunks, index: (i32, i32), dirty_rect: &mut DirtyRect) -> bool {
	if get(i as i32, j as i32 + 1, f_grid, chunks, index).density <  f_grid[i][j].density && get(i as i32, j as i32 + 2, f_grid, chunks, index).density >=  f_grid[i][j].density {
		return swap(f_grid, i, j, i as i32, j as i32 + 1, chunks, index, dirty_rect);
	}
	false
}

pub fn downward_sides(f_grid: &mut Grid, i: usize, j: usize, chunks: &mut WorldChunks, index: (i32, i32), dirty_rect: &mut DirtyRect) -> bool {
	let d = f_grid[i][j].density;

	let mut left = get(i as i32 - 1, j as i32 + 1, f_grid, chunks, index).density < d;
	let mut right = get(i as i32 + 1, j as i32 + 1, f_grid, chunks, index).density < d;
	
	if left && right {
		let rand = fastrand::bool();
		left = if rand { true } else { false };
		right = if rand { false } else { true };
	}

	if right {
		return swap(f_grid, i, j, i as i32 + 1, j as i32 + 1, chunks, index, dirty_rect);
	} else if left {
		return swap(f_grid, i, j, i as i32 - 1, j as i32 + 1, chunks, index, dirty_rect);
	}
	
	false
}

pub fn apply_velocity(f_grid: &mut Grid, i: usize, j: usize, chunks: &mut WorldChunks, index: (i32, i32), dirty_rect: &mut DirtyRect) -> bool {
	let dist = f_grid[i][j].velocity.length();

	if dist < 0.5 {
		f_grid[i][j].velocity = Vec2::ZERO;
		return false;
	}

	f_grid[i][j].velocity.x /= 1.05;
	if f_grid[i][j].velocity.x.abs() < 1.0 {
		f_grid[i][j].velocity.x = 0.;
	}

	let (force_x, force_y) = (f_grid[i][j].velocity.x / dist, f_grid[i][j].velocity.y / dist);

	if force_x == 0. && force_y == 0. {
		return false;
	}

	let d = f_grid[i][j].density;
	let (mut dx, mut dy) = (i as i32, j as i32);
	for m in 1..=dist.round() as i32 {
		let (x, y) = ((i as f32 + (force_x * m as f32)).round() as i32, (j as f32 + (force_y * m as f32)).round() as i32);

		let get_el = get(x, y, f_grid, chunks, index);

		if m == dist.round() as i32 {
			return swap(f_grid, i, j, dx, dy, chunks, index, dirty_rect);
		} else if !(get_el.density < d) && get_el.state != State::Plasma {
			if m == 1 {
				f_grid[i][j].velocity = Vec2::ZERO;
				return false;
			}
			if get_el.state == State::Solid {
				f_grid[i][j].velocity = Vec2::ZERO;
			}
			return swap(f_grid, i, j, dx, dy, chunks, index, dirty_rect);
		} else {
			let drag = get(x, y, f_grid, chunks, index).drag;
			f_grid[i][j].velocity *= drag;
		}
		
		(dx, dy) = (x, y);
	}
	f_grid[i][j].velocity = Vec2::ZERO;
	false
}

pub fn apply_gravity(future_grid: &mut Grid, i: usize, j: usize, chunks: &mut WorldChunks, index: (i32, i32)) {
	let below_element = get(i as i32, j as i32 + 1, future_grid, chunks, index);

	let max_speed = if ROWS > COLS { COLS as f32 } else {ROWS as f32 };
	future_grid[i][j].velocity = future_grid[i][j].velocity.clamp(Vec2::new(-max_speed, -max_speed), Vec2::new(max_speed, max_speed));
	
	if below_element.density < future_grid[i][j].density {
		const LIMIT: f32 = 7.;
		if future_grid[i][j].velocity.y < LIMIT {
			let g = 1.;
			future_grid[i][j].velocity.y += g;
		}
	} else {
		if below_element.velocity.y.abs() < 0.5 {
			if future_grid[i][j].velocity.x == 0. {
				if fastrand::bool() {
					future_grid[i][j].velocity.x += future_grid[i][j].velocity.y / 3.;
				} else {
					future_grid[i][j].velocity.x -= future_grid[i][j].velocity.y / 3.;
				}
			} else {
				if future_grid[i][j].velocity.x < 0. {
					future_grid[i][j].velocity.x -= (future_grid[i][j].velocity.y / 3.).abs();
				} else {
					future_grid[i][j].velocity.x += (future_grid[i][j].velocity.y / 3.).abs();
				}
			}
			future_grid[i][j].velocity.y = 0.;
		}
	}
}

pub fn get(i: i32, j: i32, f_grid: &mut Grid, chunks: &mut WorldChunks, index: (i32, i32)) -> Cell {
	if in_bound(i, j) {
		return f_grid[i as usize][j as usize]
	} else {
		let wanted_chunk = get_wanted_chunk(index, i, j);
		
		if chunks.contains_key(&wanted_chunk) {
			let (x, y) = get_new_element_coord(i, j);
			
			return chunks.get(&wanted_chunk).unwrap().grid[x as usize][y as usize];
		}
	}
	solid_element()
}

pub fn set(i: i32, j: i32, f_grid: &mut Grid, chunks: &mut WorldChunks, index: (i32, i32), cell: Cell) {
	if in_bound(i, j) {
		f_grid[i as usize][j as usize] = cell;
	} else {
		let wanted_chunk = get_wanted_chunk(index, i, j);
		
		if chunks.contains_key(&wanted_chunk) {
			let (x, y) = get_new_element_coord(i, j);
			chunks.get_mut(&wanted_chunk).unwrap().grid[x as usize][y as usize] = cell;
		}
	}
}

pub fn swap(grid: &mut Grid, i1: usize, j1: usize, i2: i32, j2: i32, chunks: &mut WorldChunks, index: (i32, i32), dirty_rect: &mut DirtyRect) -> bool {
	if in_bound(i2, j2) {
		// INFO: Element swap happening inside of the chunk
		(grid[i1][j1], grid[i2 as usize][j2 as usize]) = (grid[i2 as usize][j2 as usize], grid[i1][j1]);

		dirty_rect.set_temp(i2 as usize, j2 as usize);

		// INFO: Wake up neighboring sleeping chunks if chunk edge element moves
		if i1 == 0 || i2 == 0 {
			wake_up_chunk(chunks, index, (-1, 0), (COLS - 1, j1));
		} else if i1 == COLS - 1 || i2 == COLS as i32 - 1 {
			wake_up_chunk(chunks, index, (1, 0), (0, j1));
		}

		if j1 == 0 || j2 == 0 {
			wake_up_chunk(chunks, index, (0, -1), (i1, ROWS - 1))
		} else if j1 == ROWS - 1 || j2 == ROWS as i32 - 1 {
			wake_up_chunk(chunks, index, (0, 1), (i1, 0));
		}

		return true;
	} else {
		// INFO: Element swap happening between two chunks
		let wanted_chunk = get_wanted_chunk(index, i2, j2);
		
		if chunks.contains_key(&wanted_chunk) {
			let (x, y) = get_new_element_coord(i2, j2);


			let chunk = chunks.get_mut(&wanted_chunk).unwrap();
			(grid[i1][j1], chunk.grid[x as usize][y as usize]) = (chunk.grid[x as usize][y as usize], grid[i1][j1]);

			if !chunk.active {
				chunk::activate(chunk);
				chunk.dirty_rect.set_temp(x as usize, y as usize);
			} else {
				chunk.dirty_rect.set_temp(x as usize, y as usize);
			}

			return true;
		}
	}
	false
}

fn wake_up_chunk(chunks: &mut WorldChunks, index: (i32, i32), dir: (i32, i32), dirty_coord: (usize, usize)) {
	if let Some(chunk) = chunks.get_mut(&(index.0 + dir.0, index.1 + dir.1)) {
		if !chunk.active { 
			chunk::activate(chunk)
		} else {
			chunk.dirty_rect.set_temp(dirty_coord.0, dirty_coord.1);
		}
	}
}

// INFO: Gets the chunk that the element wants to move to
pub fn get_wanted_chunk(index: (i32, i32), i2: i32, j2: i32) -> (i32, i32) {
	let mut wanted_chunk = index;
	if i2 > COLS as i32 - 1 {
		wanted_chunk.0 += 1;
	} else if i2 < 0 as i32 {
		wanted_chunk.0 -= 1;
	}
	if j2 > ROWS as i32 - 1 {
		wanted_chunk.1 += 1;
	} else if j2 < 0 as i32 {
		wanted_chunk.1 -= 1;
	}
	
	wanted_chunk
}

// INFO: Gets the new element coordinates when swapping is done between chunks
pub fn get_new_element_coord(i: i32, j: i32) -> (i32, i32) {
	let mut x = i;
	if i < 0 || i >= COLS as i32 {
		x = i - COLS as i32;

		if x < 0 {
			x = COLS as i32 + i
		}
	}
	
	let mut y = j;
	if j < 0 || j >= ROWS as i32 {
		y = j - ROWS as i32;

		if y < 0 {
			y = ROWS as i32 + j;
		}
	}
	
	x = x.clamp(0, COLS as i32 - 1);
	y = y.clamp(0, ROWS as i32 - 1);
	(x, y)
}