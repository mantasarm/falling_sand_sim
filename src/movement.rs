use std::collections::HashMap;

use notan::math::Vec2;

use crate::{element::{Cell, State, solid_element}, grid::{ROWS, COLS, in_bound}};

pub fn downward(f_grid: &mut Box<[[Cell; ROWS]; COLS]>, i: usize, j: usize, chunks: &HashMap<(i32, i32), Box<[[Cell; ROWS]; COLS]>>, index: (i32, i32), c_swaps: &mut Vec<(i32, i32, usize, usize, Cell)>) -> bool {
	if get(i, j, i as i32, j as i32 + 1, f_grid, chunks, index).density <  f_grid[i][j].density {
		swap(f_grid, i, j, i as i32, j as i32+ 1, chunks, index, c_swaps);
		return true;
	}
	false
}

pub fn downward_sides(f_grid: &mut Box<[[Cell; ROWS]; COLS]>, i: usize, j: usize, chunks: &HashMap<(i32, i32), Box<[[Cell; ROWS]; COLS]>>, index: (i32, i32), c_swaps: &mut Vec<(i32, i32, usize, usize, Cell)>) -> bool {
	let d = f_grid[i][j].density;

	let left_element = get(i, j, i as i32 - 1, j as i32 + 1, f_grid, chunks, index);
	let right_element = get(i, j, i as i32 + 1, j as i32 + 1, f_grid, chunks, index);
	
	if left_element.density < d && right_element.density < d {
		if fastrand::bool() {
			swap(f_grid, i, j, i as i32 - 1, j as i32 + 1, chunks, index, c_swaps);
		} else {
			swap(f_grid, i, j, i as i32 + 1, j as i32 + 1, chunks, index, c_swaps);
		}
	} else if right_element.density < d {
		swap(f_grid, i, j, i as i32 + 1, j as i32 + 1, chunks, index, c_swaps);
	} else if left_element.density < d {
		swap(f_grid, i, j, i as i32 - 1, j as i32 + 1, chunks, index, c_swaps);
	}
	
	false
}

pub fn apply_velocity(f_grid: &mut Box<[[Cell; ROWS]; COLS]>, i: usize, j: usize, chunks: &HashMap<(i32, i32), Box<[[Cell; ROWS]; COLS]>>, index: (i32, i32), c_swaps: &mut Vec<(i32, i32, usize, usize, Cell)>) -> bool {
	let dist = (f_grid[i][j].velocity.x.powf(2.) + f_grid[i][j].velocity.y.powf(2.)).sqrt();

	if dist < 0.5 {
		return false;
	}

	f_grid[i][j].velocity.x /= 1.05;
	if f_grid[i][j].velocity.x.abs() < 1.0 {
		f_grid[i][j].velocity.x = 0.;
	}

	if f_grid[i][j].velocity.y.abs() < 0. {
		f_grid[i][j].velocity.y = 0.;
	}

	let (force_x, force_y) = (f_grid[i][j].velocity.x / dist, f_grid[i][j].velocity.y / dist);

	if force_x == 0. && force_y == 0. {
		return false;
	}

	let d = f_grid[i][j].density;
	let (mut dx, mut dy) = (i as i32, j as i32);
	for m in 1..=dist.round() as i32 {
		let (x, y) = ((i as f32 + (force_x * m as f32)).round() as i32, (j as f32 + (force_y * m as f32)).round() as i32);

		let get_el = get(i, j, x, y, f_grid, chunks, index);

		if m == dist.round() as i32 {
			swap(f_grid, i, j, dx, dy, chunks, index, c_swaps);
			return true;
		} else if !(get_el.density < d) {
			if m == 1 {
				f_grid[i][j].velocity = Vec2::ZERO;
				return false;
			}
			if get_el.state == State::Solid {
				f_grid[i][j].velocity = Vec2::ZERO;
			}

			swap(f_grid, i, j, dx, dy, chunks, index, c_swaps);
			return true;
		} else {
			let drag = get(i, j, x, y, f_grid, chunks, index).drag;
			f_grid[i][j].velocity *= drag;
		}
		
		(dx, dy) = (x, y);
	}

	false
}

pub fn apply_gravity(future_grid: &mut Box<[[Cell; ROWS]; COLS]>, i: usize, j: usize, chunks: &HashMap<(i32, i32), Box<[[Cell; ROWS]; COLS]>>, index: (i32, i32)) {
	let below_element = get(i, j, i as i32, j as i32 + 1, future_grid, chunks, index);
	let below_below_element = get(i, j, i as i32, j as i32 + 2, future_grid, chunks, index);
	
	if below_element.density < future_grid[i][j].density && below_below_element.density < future_grid[i][j].density {
		let limit = 7.;
		if future_grid[i][j].velocity.y < limit {
			let g = 1.;
			future_grid[i][j].velocity.y += g;
		} else {
			future_grid[i][j].velocity.y = limit;
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

pub fn upward(f_grid: &mut Box<[[Cell; ROWS]; COLS]>, i: usize, j: usize, chunks: &HashMap<(i32, i32), Box<[[Cell; ROWS]; COLS]>>, index: (i32, i32), c_swaps: &mut Vec<(i32, i32, usize, usize, Cell)>) -> bool {
	let cell_to_check = get(i, j, i as i32, j as i32 - 1, f_grid, chunks, index);
	if cell_to_check .density > f_grid[i][j].density && cell_to_check .state == State::Gas {
		swap(f_grid, i, j, i as i32, j as i32 - 1, chunks, index, c_swaps);
		return true;
	}
	false
}

pub fn sideways_gas(f_grid: &mut Box<[[Cell; ROWS]; COLS]>, i: usize, j: usize, amount: i32, chunks: &HashMap<(i32, i32), Box<[[Cell; ROWS]; COLS]>>, index: (i32, i32), c_swaps: &mut Vec<(i32, i32, usize, usize, Cell)>) -> bool {
	let d = f_grid[i][j].density;

	let left_element = get(i, j, i as i32 - 1, j as i32, f_grid, chunks, index);
	let right_element = get(i, j, i as i32 + 1, j as i32, f_grid, chunks, index);
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
		let el = get(i, j, i as i32 + x * dir, j as i32, f_grid, chunks, index);
		if !(el.density > d && el.state == State::Gas) {
			swap(f_grid, i, j, dx, dy, chunks, index, c_swaps);
			return true;
		} else if x == amount {
			swap(f_grid, i, j, i as i32 + x * dir, j as i32, chunks, index, c_swaps);
			return true;
		}
		(dx, dy) = (i as i32 + x * dir, j as i32)
	}
	false
}

pub fn get(i1: usize, j1: usize, i2: i32, j2: i32, f_grid: &mut Box<[[Cell; ROWS]; COLS]>, chunks: &HashMap<(i32, i32), Box<[[Cell; ROWS]; COLS]>>, index: (i32, i32)) -> Cell {
	if in_bound(i2, j2) {
		return f_grid[i2 as usize][j2 as usize]
	} else {
		let wanted_chunk = get_wanted_chunk(index, i2, j2);
		
		if chunks.contains_key(&wanted_chunk) {
			let (x, y) = get_new_element_coord(i1, j1, i2, j2);
			
			return chunks.get(&wanted_chunk).unwrap()[x as usize][y as usize];
		}
	}
	solid_element()
}

pub fn swap(grid: &mut Box<[[Cell; ROWS]; COLS]>, i1: usize, j1: usize, i2: i32, j2: i32, chunks: &HashMap<(i32, i32), Box<[[Cell; ROWS]; COLS]>>, index: (i32, i32), c_swaps: &mut Vec<(i32, i32, usize, usize, Cell)>) -> bool {
	if in_bound(i2, j2) {
		let temp = grid[i1][j1].clone();
		grid[i1][j1] = grid[i2 as usize][j2 as usize].clone();
		grid[i2 as usize][j2 as usize] = temp;
		return true;
	} else {
		let wanted_chunk = get_wanted_chunk(index, i2, j2);
		
		if chunks.contains_key(&wanted_chunk) {
			let (x, y) = get_new_element_coord(i1, j1, i2, j2);
						
			let element_cell = chunks.get(&wanted_chunk).unwrap()[x as usize][y as usize];
			c_swaps.push((wanted_chunk.0, wanted_chunk.1, x as usize, y as usize, grid[i1][j1].clone()));
			grid[i1][j1] = element_cell;
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

fn get_new_element_coord(i1: usize, j1: usize, i2: i32, j2: i32) -> (i32, i32) {
	let mut x = i2 as i32;
	if i2 < 0 || i2 > COLS as i32 - 1 {
		x = i2 as i32 - i1 as i32 - 1;

		if x < 0 {
			x = COLS as i32 + x + 1;
		}
	}
	
	let mut y = j2 as i32;
	if j2 < 0 || j2 > ROWS as i32 - 1 {
		y = j2 as i32 - j1 as i32 - 1;

		if y < 0 {
			y = ROWS as i32 + y + 1;
		}
	}

	(x, y)
}