use crate::{element::{Cell, Element, State}, grid::{ROWS, COLS}};

pub fn downward(f_grid: &mut Box<[[Cell; ROWS]; COLS]>, i: usize, j: usize) -> bool {
	if f_grid[i][j + 1].density <  f_grid[i][j].density {
		swap(f_grid, i, j, i, j + 1);
		return true;
	}
	false
}

pub fn downward_sides(f_grid: &mut Box<[[Cell; ROWS]; COLS]>, i: usize, j: usize) -> bool {
	let d = f_grid[i][j].density;
	
	if f_grid[i - 1][j + 1].density < d && f_grid[i + 1][j + 1].density < d {
		if fastrand::bool() {
			swap(f_grid, i, j, i - 1, j + 1);
		} else {
			swap(f_grid, i, j, i + 1, j + 1);
		}
	} else if f_grid[i + 1][j + 1].density < d {
		swap(f_grid, i, j, i + 1, j + 1);
	} else if f_grid[i - 1][j + 1].density < d {
		swap(f_grid, i, j, i - 1, j + 1);
	}
	
	false
}

pub fn apply_velocity(f_grid: &mut Box<[[Cell; ROWS]; COLS]>, i: usize, j: usize) -> bool {
	let dist = (f_grid[i][j].velocity.x.powf(2.) + f_grid[i][j].velocity.y.powf(2.)).sqrt();

	if dist <= 0. {
		return false;
	}
	
	let (mut force_x, force_y) = (f_grid[i][j].velocity.x / dist, f_grid[i][j].velocity.y / dist);

	let d = f_grid[i][j].density;
	if force_x > 0. && f_grid[i + 1][j].velocity.x == 0. && f_grid[i + 1][j].density >= d {
		f_grid[i][j].velocity.x = 0.;
		force_x = 0.;
	} else if force_x < 0. && f_grid[i - 1][j].velocity.x == 0. && f_grid[i - 1][j].density >= d {
		f_grid[i][j].velocity.x = 0.;
		force_x = 0.;
	}
	if f_grid[i][j].velocity.x != 0. {
		if f_grid[i][j].velocity.x.abs() > 0.5 {
			f_grid[i][j].velocity.x /= 1.05;
		} else {
			f_grid[i][j].velocity.x = 0.;
		}
	}

	if force_x == 0. && force_y == 0. {
		return false;
	}

	let (mut dx, mut dy) = (i as i32, j as i32);
	for m in 1..=dist.round() as i32 {
		let (x, y) = ((i as f32 + (force_x * m as f32)).round() as i32, (j as f32 + (force_y * m as f32)).round() as i32);

		if !(x >= 0 && y >= 0 && x < COLS as i32 && y < ROWS as i32) {
			 return false;
		}
		if f_grid[x as usize][y as usize].density < d {
			f_grid[dx as usize][dy as usize].velocity *= f_grid[x as usize][y as usize].drag;
			swap(f_grid, dx as usize, dy as usize, x as usize, y as usize);
			(dx, dy) = (x, y);
		} else {
			if m == 1 {
				return false;
			}
			// break;
		}
	}

	true
}

pub fn apply_gravity(future_grid: &mut Box<[[Cell; ROWS]; COLS]>, i: usize, j: usize) {
	if future_grid[i][j + 1].density < future_grid[i][j].density {
		let mut limit = 6.;

		if future_grid[i][j].velocity.y <= limit {
			let g = 1.;
			future_grid[i][j].velocity.y += g;
		} else {
			future_grid[i][j].velocity.y = limit;
		}
	} else {
		if future_grid[i][j + 1].velocity.y == 0. {
			if future_grid[i][j].velocity.x == 0. {
				if fastrand::bool() {
					future_grid[i][j].velocity.x += future_grid[i][j].velocity.y / 3.;
				} else {
					future_grid[i][j].velocity.x -= future_grid[i][j].velocity.y / 3.;
				}
			}
			future_grid[i][j].velocity.y = 0.;
		}
	}
}

pub fn upward(f_grid: &mut Box<[[Cell; ROWS]; COLS]>, i: usize, j: usize) -> bool {
	if f_grid[i][j - 1].density > f_grid[i][j].density && f_grid[i][j - 1].state == State::Gas {
		swap(f_grid, i, j, i, j - 1);
		return true;
	}
	false
}

pub fn sideways_gas(f_grid: &mut Box<[[Cell; ROWS]; COLS]>, i: usize, j: usize, amount: i32) -> bool {
	let d = f_grid[i][j].density;
	
	let dir = if f_grid[i - 1][j].density > d && f_grid[i - 1][j].state == State::Gas && f_grid[i + 1][j].density > d && f_grid[i + 1][j].state == State::Gas {
		if fastrand::bool() {
			1
		} else {
			-1
		}
	} else if f_grid[i - 1][j].density > d && f_grid[i - 1][j].state == State::Gas {
		-1
	} else if f_grid[i + 1][j].density > d && f_grid[i + 1][j].state == State::Gas {
		1
	} else {
		0
	};

	if dir == 0 {
		return false
	}

	for x in 1..amount {
		if !(x >= 0 && x < COLS as i32) {
			 return false;
		}
		if f_grid[(i as i32 + x * dir) as usize][j].density > d && f_grid[(i as i32 + x * dir) as usize][j].state == State::Gas {
			swap(f_grid, (i as i32 + x * dir) as usize - dir as usize, j, (i as i32 + x * dir) as usize, j);
		} else {
			return true;
		}
	}
	
	false
}


pub fn swap(grid: &mut Box<[[Cell; ROWS]; COLS]>, i1: usize, j1: usize, i2: usize, j2: usize) {
	let temp = grid[i1][j1].clone();
	grid[i1][j1] = grid[i2][j2].clone();
	grid[i2][j2] = temp;
}