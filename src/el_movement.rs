use std::collections::HashMap;

use crate::{base_movement::*, chunk::*};

pub fn falling_sand(f_grid: &mut Grid, i: usize, j: usize, chunks: &mut HashMap<(i32, i32), Chunk>, index: (i32, i32), keep_active: &mut bool) -> bool {
	apply_gravity(f_grid, i, j, chunks, index);
	if !downward(f_grid, i, j, chunks, index) {
		if !apply_velocity(f_grid, i, j, chunks, index) {
			if !downward_sides(f_grid, i, j, chunks, index) {
				return false;
			}
		}
	}
	*keep_active = true;
	true
}

pub fn liquid_movement(f_grid: &mut Grid, i: usize, j: usize, chunks: &mut HashMap<(i32, i32), Chunk>, index: (i32, i32)) -> bool {
	apply_gravity(f_grid, i, j, chunks, index);

	if !downward(f_grid, i, j, chunks, index) {				
		if !apply_velocity(f_grid, i, j, chunks, index) {
			let mut dir = 0.;

			if f_grid[i][j].velocity.x == 0. {
				let left_element = get(i as i32 - 1, j as i32, f_grid, chunks, index);
				let right_element = get(i as i32 + 1, j as i32, f_grid, chunks, index);
				if left_element.density < f_grid[i][j].density && right_element.density < f_grid[i][j].density {
					if fastrand::bool() {
						dir = -1.;
					} else {
						dir = 1.;
					}
				} else if left_element.density < f_grid[i][j].density {
					dir = -1.;
				} else if right_element.density < f_grid[i][j].density{
					dir = 1.;
				}
			}
			
			if dir != 0. {	
				f_grid[i][j].velocity.x += 5.5 * dir;
				f_grid[i][j].velocity.y += 0.5;
			} else {
				f_grid[i][j].velocity.x = 0.;
				return false;
			}
		}
	}
	true
}

pub fn gas_movement(f_grid: &mut Grid, i: usize, j: usize, chunks: &mut HashMap<(i32, i32), Chunk>, index: (i32, i32)) -> bool {
	if !upward(f_grid, i, j, chunks, index) {
		if !sideways_gas(f_grid, i, j, 4, chunks, index) {
			return false;
		}
	}
	true
}