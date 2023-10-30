use crate::{base_movement::*, chunk::*, chunk_manager::WorldChunks, element_actions::{set_action, is_flammable}, element::{Action, air_element}};

#[inline]
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

#[inline]
pub fn liquid_movement(f_grid: &mut Grid, i: usize, j: usize, chunks: &mut WorldChunks, index: (i32, i32), keep_active: &mut bool, dirty_rect: &mut DirtyRect) -> bool {
	apply_gravity(f_grid, i, j, chunks, index);

	let down_density = get(i as i32, j as i32 + 1, f_grid, chunks, index).density;

	if down_density >= f_grid[i][j].density && f_grid[i][j].velocity.x.abs() <= 5.0 {
		let mut left = f_grid[i][j].velocity.x < 0.;
		let mut right = f_grid[i][j].velocity.x > 0.;

		if !left && !right {
			left = get(i as i32 - 1, j as i32, f_grid, chunks, index).density < f_grid[i][j].density;
			right = get(i as i32 + 1, j as i32, f_grid, chunks, index).density < f_grid[i][j].density;
			
			if left && right {
				let rand = fastrand::bool();
				left = rand;
				right = !rand;
			}
		}

		if right {
			f_grid[i][j].velocity.x += 1.0;
		} else if left {
			f_grid[i][j].velocity.x -= 1.0;
		}
	}

	if apply_velocity(f_grid, i, j, chunks, index, dirty_rect) {
		*keep_active = true;

		return true;
	}

	false
}

// TODO: fix weird interaction between different gasses
// TODO: improve gas interaction with itself
#[inline]
pub fn gas_movement(f_grid: &mut Grid, i: usize, j: usize, chunks: &mut WorldChunks, index: (i32, i32), keep_active: &mut bool, dirty_rect: &mut DirtyRect) -> bool {
	let up_density = get(i as i32, j as i32 - 1, f_grid, chunks, index).density;

	if f_grid[i][j].velocity.y > -1.75 && up_density < f_grid[i][j].density {
		f_grid[i][j].velocity.y += -0.5;
	} else if up_density >= f_grid[i][j].density && f_grid[i][j].velocity.x.abs() <= 2.5 {
		let mut left = f_grid[i][j].velocity.x < 0.;
		let mut right = f_grid[i][j].velocity.x > 0.;

		if !left && !right {
			left = get(i as i32 - 1, j as i32, f_grid, chunks, index).density < f_grid[i][j].density;
			right = get(i as i32 + 1, j as i32, f_grid, chunks, index).density < f_grid[i][j].density;
			
			if left && right {
				let rand = fastrand::bool();
				left = rand;
				right = !rand;
			}
		}

		if right {
			f_grid[i][j].velocity.x += 0.5;
		} else if left {
			f_grid[i][j].velocity.x -= 0.5;
		}
	}

	if apply_velocity(f_grid, i, j, chunks, index, dirty_rect) {
		*keep_active = true;

		return true;
	}

	false
}

#[inline]
pub fn fire_movement(f_grid: &mut Grid, i: usize, j: usize, chunks: &mut WorldChunks, index: (i32, i32), keep_active: &mut bool, dirty_rect: &mut DirtyRect) -> bool {
	let rand =  fastrand::i32(2..8);
	f_grid[i][j].lifetime -= rand;

	*keep_active = true;

	if f_grid[i][j].lifetime <= 0 {
		f_grid[i][j] = air_element();
		return true;
	}

	if f_grid[i][j].velocity.y >= -4. {
		f_grid[i][j].velocity.y += -0.5;
	}
	f_grid[i][j].velocity.x += ((f_grid[i][j].lifetime as f32).sin() * 1.075).clamp(-1.5, 1.5);

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
	
	apply_velocity(f_grid, i, j, chunks, index, dirty_rect);

	true
}