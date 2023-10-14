use crate::{base_movement::*, chunk::*, chunk_manager::WorldChunks, element_actions::{set_action, is_flammable}, element::{Action, air_element}};

pub fn falling_sand(f_grid: &mut Grid, i: usize, j: usize, chunks: &mut WorldChunks, index: (i32, i32), keep_active: &mut bool, dirty_rect: &mut DirtyRect) -> bool {
	apply_gravity(f_grid, i, j, chunks, index);
	if !downward(f_grid, i, j, chunks, index, dirty_rect) {
		if !apply_velocity(f_grid, i, j, chunks, index, dirty_rect) {
			if !downward_sides(f_grid, i, j, chunks, index, dirty_rect) {
				return false;
			}
		}
	}
	*keep_active = true;
	true
}

pub fn liquid_movement(f_grid: &mut Grid, i: usize, j: usize, chunks: &mut WorldChunks, index: (i32, i32), keep_active: &mut bool, dirty_rect: &mut DirtyRect) -> bool {
	apply_gravity(f_grid, i, j, chunks, index);

	if !downward(f_grid, i, j, chunks, index, dirty_rect) {				
		if !apply_velocity(f_grid, i, j, chunks, index, dirty_rect) {
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
				dirty_rect.set_temp(i, j);
			} else {
				f_grid[i][j].velocity.x = 0.;
				return false;
			}
		}
	}
	*keep_active = true;
	true
}

pub fn gas_movement(f_grid: &mut Grid, i: usize, j: usize, chunks: &mut WorldChunks, index: (i32, i32), keep_active: &mut bool, dirty_rect: &mut DirtyRect) -> bool {
	if !upward(f_grid, i, j, chunks, index, dirty_rect) {
		if !sideways_gas(f_grid, i, j, 4, chunks, index, dirty_rect) {
			return false;
		}
	}
	*keep_active = true;
	true
}

pub fn fire_movement(f_grid: &mut Grid, i: usize, j: usize, chunks: &mut WorldChunks, index: (i32, i32), keep_active: &mut bool, dirty_rect: &mut DirtyRect) -> bool {
	let rand =  fastrand::i32(2..8);
	f_grid[i][j].lifetime -= rand;

	if f_grid[i][j].lifetime <= 0 {
		f_grid[i][j] = air_element();
		*keep_active = true;
		dirty_rect.set_temp(i, j);
		return false;
	}

	if f_grid[i][j].velocity.y >= -4. {
		f_grid[i][j].velocity.y += -0.5;
	}
	f_grid[i][j].velocity.x += (f_grid[i][j].lifetime as f32).sin() * 1.1;

	f_grid[i][j].color[1] = (f_grid[i][j].color[1] as f32 - (rand as f32).powf(2.) * 0.3).clamp(0., 200.) as u8;
	f_grid[i][j].color[3] = (f_grid[i][j].color[3] as f32 - (rand as f32).powf(2.)).clamp(220., 255.) as u8;

	if is_flammable(&get(i as i32, j as i32 - 1, f_grid, chunks, index)) {
		set_action(i as i32, j as i32 - 1, f_grid, chunks, index, Some(Action::Burn));
	}
	if is_flammable(&get(i as i32, j as i32 + 1, f_grid, chunks, index)) {
		set_action(i as i32, j as i32 + 1, f_grid, chunks, index, Some(Action::Burn));
	}
	if is_flammable(&get(i as i32 - 1, j as i32, f_grid, chunks, index)) {
		set_action(i as i32 - 1, j as i32, f_grid, chunks, index, Some(Action::Burn));
	}
	if is_flammable(&get(i as i32 + 1, j as i32, f_grid, chunks, index)) {
		set_action(i as i32 + 1, j as i32, f_grid, chunks, index, Some(Action::Burn));
	}
	
	if apply_velocity(f_grid, i, j, chunks, index, dirty_rect) {
		*keep_active = true;
	}
	return true;
}