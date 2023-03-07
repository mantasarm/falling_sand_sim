use crate::{element::{Cell, Element}, grid::{ROWS, COLS}};

pub fn downward(f_grid: &mut Box<[[Cell; ROWS]; COLS]>, i: usize, j: usize) -> bool {
	if f_grid[i][j + 1].element == Element::Air {
		swap(f_grid, i, j, i, j + 1);
		return true;
	}
	false
}

pub fn downward_sides(f_grid: &mut Box<[[Cell; ROWS]; COLS]>, i: usize, j: usize) -> bool {
	if f_grid[i - 1][j + 1].element == Element::Air && f_grid[i + 1][j + 1].element == Element::Air {
		if fastrand::bool() {
			swap(f_grid, i, j, i - 1, j + 1);
		} else {
			swap(f_grid, i, j, i + 1, j + 1);
		}
	} else if f_grid[i - 1][j + 1].element == Element::Air {
		swap(f_grid, i, j, i - 1, j + 1);
	} else if f_grid[i + 1][j + 1].element == Element::Air {
		swap(f_grid, i, j, i + 1, j + 1);
	}
	
	false
}

pub fn apply_velocity(current_grid: &Box<[[Cell; ROWS]; COLS]>, f_grid: &mut Box<[[Cell; ROWS]; COLS]>, i: usize, j: usize) -> bool {
	let dist = (current_grid[i][j].velocity.x.powf(2.) + current_grid[i][j].velocity.y.powf(2.)).sqrt();

	if dist <= 0. {
		return false;
	}
	
	let (force_x, force_y) = (current_grid[i][j].velocity.x / dist, current_grid[i][j].velocity.y / dist);

	let (mut dx, mut dy) = (i as i32, j as i32);
	for m in 1..=dist as i32 {
		let (x, y) = ((i as f32 + (force_x * m as f32)) as i32, (j as f32 + (force_y * m as f32)) as i32);

		if !(x >= 0 && y >= 0 && x < COLS as i32 && y < ROWS as i32) {
			return false;
		}
		if f_grid[x as usize][y as usize].element == Element::Air {
			swap(f_grid, dx as usize, dy as usize, x as usize, y as usize);
		} else {
			return false;
		}
		(dx, dy) = (x, y);
	}

	true
}

pub fn apply_gravity(future_grid: &mut Box<[[Cell; ROWS]; COLS]>, i: usize, j: usize) {
	if future_grid[i][j + 1].element == Element::Air {
		if future_grid[i][j].velocity.y <= 10. {
			future_grid[i][j].velocity.y += 1.;
		}
	} else {
		future_grid[i][j].velocity.y = 0.;
	}
}

pub fn swap(grid: &mut Box<[[Cell; ROWS]; COLS]>, i1: usize, j1: usize, i2: usize, j2: usize) {
	let temp = grid[i1][j1].clone();
	grid[i1][j1] = grid[i2][j2].clone();
	grid[i2][j2] = temp;
}