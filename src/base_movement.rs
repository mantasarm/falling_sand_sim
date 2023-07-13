use std::collections::HashMap;

use notan::math::Vec2;

use crate::{element::{Cell, State, solid_element}, chunk::{ROWS, COLS, in_bound, Chunk, self, Grid}};

pub fn downward(f_grid: &mut Grid, i: usize, j: usize, chunks: &mut HashMap<(i32, i32), Chunk>, index: (i32, i32)) -> bool {
	if get(i as i32, j as i32 + 1, f_grid, chunks, index).density <  f_grid[i][j].density && get(i as i32, j as i32 + 2, f_grid, chunks, index).density >=  f_grid[i][j].density {
		return swap(f_grid, i, j, i as i32, j as i32 + 1, chunks, index);
	}
	false
}

pub fn downward_sides(f_grid: &mut Grid, i: usize, j: usize, chunks: &mut HashMap<(i32, i32), Chunk>, index: (i32, i32)) -> bool {
	let d = f_grid[i][j].density;

	let mut left = get(i as i32 - 1, j as i32 + 1, f_grid, chunks, index).density < d;
	let mut right = get(i as i32 + 1, j as i32 + 1, f_grid, chunks, index).density < d;
	
	if left && right {
		let rand = fastrand::bool();
		left = if rand { true } else { false };
		right = if rand { false } else { true };
	}

	if right {
		return swap(f_grid, i, j, i as i32 + 1, j as i32 + 1, chunks, index);
	} else if left {
		return swap(f_grid, i, j, i as i32 - 1, j as i32 + 1, chunks, index);
	}
	
	false
}

pub fn apply_velocity(f_grid: &mut Grid, i: usize, j: usize, chunks: &mut HashMap<(i32, i32), Chunk>, index: (i32, i32)) -> bool {
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
			return swap(f_grid, i, j, dx, dy, chunks, index);
		} else if !(get_el.density < d) {
			if m == 1 {
				f_grid[i][j].velocity = Vec2::ZERO;
				return false;
			}
			if get_el.state == State::Solid {
				f_grid[i][j].velocity = Vec2::ZERO;
			}

			return swap(f_grid, i, j, dx, dy, chunks, index);
		} else {
			let drag = get(x, y, f_grid, chunks, index).drag;
			f_grid[i][j].velocity *= drag;
		}
		
		(dx, dy) = (x, y);
	}
	f_grid[i][j].velocity = Vec2::ZERO;
	false
}

pub fn apply_gravity(future_grid: &mut Grid, i: usize, j: usize, chunks: &mut HashMap<(i32, i32), Chunk>, index: (i32, i32)) {
	let below_element = get(i as i32, j as i32 + 1, future_grid, chunks, index);

	//future_grid[i][j].velocity = future_grid[i][j].velocity.clamp(Vec2::new(-10., -10.), Vec2::new(10., 10.));
	
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

pub fn upward(f_grid: &mut Grid, i: usize, j: usize, chunks: &mut HashMap<(i32, i32), Chunk>, index: (i32, i32)) -> bool {
	let cell_to_check = get(i as i32, j as i32 - 1, f_grid, chunks, index);
	if cell_to_check .density > f_grid[i][j].density && cell_to_check .state == State::Gas {
		return swap(f_grid, i, j, i as i32, j as i32 - 1, chunks, index);
	}
	false
}

pub fn sideways_gas(f_grid: &mut Grid, i: usize, j: usize, amount: i32, chunks: &mut HashMap<(i32, i32), Chunk>, index: (i32, i32)) -> bool {
	let d = f_grid[i][j].density;

	let left_element = get(i as i32 - 1, j as i32, f_grid, chunks, index);
	let right_element = get(i as i32 + 1, j as i32, f_grid, chunks, index);
	let dir = if left_element.density > d && left_element.state == State::Gas && right_element.density > d && right_element.state == State::Gas {
		if fastrand::bool() {
			1
		} else {
			-1
		}
	} else if left_element.density > d && left_element.state == State::Gas {
		-1
	} else if right_element.density > d && right_element.state == State::Gas {
		1
	} else {
		0
	};

	if dir == 0 {
		return false
	}

	let (mut dx, mut dy) = (i as i32 + dir, j as i32);
	for x in 1..=amount {
		let el = get(i as i32 + x * dir, j as i32, f_grid, chunks, index);
		
		if x == amount {
			return swap(f_grid, i, j, dx, dy, chunks, index);
		} else if !(el.density > d && el.state == State::Gas) {
			return swap(f_grid, i, j, dx, dy, chunks, index);
		}
		(dx, dy) = (i as i32 + x * dir, j as i32)
	}
	false
}

pub fn get(i: i32, j: i32, f_grid: &mut Grid, chunks: &mut HashMap<(i32, i32), Chunk>, index: (i32, i32)) -> Cell {
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

pub fn swap(grid: &mut Grid, i1: usize, j1: usize, i2: i32, j2: i32, chunks: &mut HashMap<(i32, i32), Chunk>, index: (i32, i32)) -> bool {
	if in_bound(i2, j2) {
		(grid[i1][j1], grid[i2 as usize][j2 as usize]) = (grid[i2 as usize][j2 as usize], grid[i1][j1]);

		// INFO: Wake up neighboring sleeping chunks if chunk edge element moves
		if i1 == 0 || i2 == 0 {
			match chunks.get_mut(&(index.0 - 1, index.1)) {
				Some(chunk) => chunk::activate(chunk),
				_ => ()
			}
		} else if i1 == COLS - 1 || i2 == COLS as i32 - 1 {
			match chunks.get_mut(&(index.0 + 1, index.1)) {
				Some(chunk) => chunk::activate(chunk),
				_ => ()
			}
		}
		if j1 == 0 || j2 == 0{
			match chunks.get_mut(&(index.0, index.1 - 1)) {
				Some(chunk) => chunk::activate(chunk),
				_ => ()
			}
		} else if j1 == ROWS - 1 || j2 == ROWS as i32 - 1 {
			match chunks.get_mut(&(index.0, index.1 + 1)) {
				Some(chunk) => chunk::activate(chunk),
				_ => ()
			}
		}

		return true;
	} else {
		let wanted_chunk = get_wanted_chunk(index, i2, j2);
		
		if chunks.contains_key(&wanted_chunk) {
			let (x, y) = get_new_element_coord(i2, j2);

			(grid[i1][j1], chunks.get_mut(&wanted_chunk).unwrap().grid[x as usize][y as usize]) = (chunks.get(&wanted_chunk).unwrap().grid[x as usize][y as usize], grid[i1][j1]);
			chunk::activate(chunks.get_mut(&wanted_chunk).unwrap());
			return true;
		}
	}
	false
}

fn get_wanted_chunk(index: (i32, i32), i2: i32, j2: i32) -> (i32, i32) {
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

fn get_new_element_coord(i: i32, j: i32) -> (i32, i32) {
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