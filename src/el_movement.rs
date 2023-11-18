use crate::{base_movement::*, chunk::{Grid, MovData, self}, element_actions::{set_action, is_flammable}, element::{Action, air_element}};

#[inline]
pub fn falling_sand(f_grid: &mut Grid, i: usize, j: usize, mov_dt: &mut MovData) -> bool {
	apply_gravity(f_grid, i, j, mov_dt);
	if !downward(f_grid, i, j, mov_dt) {
		if !apply_velocity(f_grid, i, j, mov_dt) {
			if !downward_sides(f_grid, i, j, mov_dt) {
				return false;
			}
		}
	}

	*mov_dt.keep_active = true;
	true
}

#[inline]
pub fn liquid_movement(f_grid: &mut Grid, i: usize, j: usize, mov_dt: &mut MovData) -> bool {
	apply_gravity(f_grid, i, j, mov_dt);

	let down_density = get(i as i32, j as i32 + 1, f_grid, mov_dt).density;

	if down_density >= f_grid[i][j].density && f_grid[i][j].velocity.x.abs() <= 10. {
		let mut left = f_grid[i][j].velocity.x < 0.;
		let mut right = f_grid[i][j].velocity.x > 0.;

		if !left && !right {
			left = get(i as i32 - 1, j as i32, f_grid, mov_dt).density < f_grid[i][j].density;
			right = get(i as i32 + 1, j as i32, f_grid, mov_dt).density < f_grid[i][j].density;
			
			if left && right {
				let rand = fastrand::bool();
				left = rand;
				right = !rand;
			}
		}

		if right {
			if f_grid[i][j].velocity.x < 0. {
				f_grid[i][j].velocity.x = 0.;
			}
			f_grid[i][j].velocity.x += 1.4;
		} else if left {
			if f_grid[i][j].velocity.x > 0. {
				f_grid[i][j].velocity.x = 0.;
			}
			f_grid[i][j].velocity.x -= 1.4;
		}
	}

	if apply_velocity(f_grid, i, j, mov_dt) {
		*mov_dt.keep_active = true;

		return true;
	}

	false
}

// TODO: fix weird interaction between different gasses
// TODO: improve gas interaction with itself
#[inline]
pub fn gas_movement(f_grid: &mut Grid, i: usize, j: usize, mov_dt: &mut MovData) -> bool {
	let up_density = get(i as i32, j as i32 - 1, f_grid, mov_dt).density;

	if f_grid[i][j].velocity.y > -1.75 && up_density < f_grid[i][j].density {
		f_grid[i][j].velocity.y += -0.5;
	} else if up_density >= f_grid[i][j].density && f_grid[i][j].velocity.x.abs() <= 2.5 {
		let mut left = f_grid[i][j].velocity.x < 0.;
		let mut right = f_grid[i][j].velocity.x > 0.;

		if !left && !right {
			left = get(i as i32 - 1, j as i32, f_grid, mov_dt).density < f_grid[i][j].density;
			right = get(i as i32 + 1, j as i32, f_grid, mov_dt).density < f_grid[i][j].density;
			
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

	if apply_velocity(f_grid, i, j, mov_dt) {
		*mov_dt.keep_active = true;

		return true;
	}

	false
}

#[inline]
pub fn fire_movement(f_grid: &mut Grid, i: usize, j: usize, mov_dt: &mut MovData) -> bool {
	let rand =  fastrand::i32(2..8);
	f_grid[i][j].lifetime -= rand;

	*mov_dt.keep_active = true;

	if f_grid[i][j].lifetime <= 0 {
		f_grid[i][j] = air_element();
		chunk::update_byte(mov_dt.bytes, i, j, &[0, 0, 0, 0]);
		return true;
	}

	if f_grid[i][j].velocity.y >= -4. {
		f_grid[i][j].velocity.y += -0.5;
	}
	f_grid[i][j].velocity.x += ((f_grid[i][j].lifetime as f32).sin() * 1.075).clamp(-1.5, 1.5);

	f_grid[i][j].color[1] = (f_grid[i][j].color[1] as f32 - (rand as f32).powf(2.) * 0.3).clamp(0., 200.) as u8;
	f_grid[i][j].color[3] = (f_grid[i][j].color[3] as f32 - (rand as f32).powf(2.)).clamp(220., 255.) as u8;

	if is_flammable(&get(i as i32, j as i32 - 1, f_grid, mov_dt)) {
		set_action(i as i32, j as i32 - 1, f_grid, mov_dt, Some(Action::Burn));
	}
	if is_flammable(&get(i as i32, j as i32 + 1, f_grid, mov_dt)) {
		set_action(i as i32, j as i32 + 1, f_grid, mov_dt, Some(Action::Burn));
	}
	if is_flammable(&get(i as i32 - 1, j as i32, f_grid, mov_dt)) {
		set_action(i as i32 - 1, j as i32, f_grid, mov_dt, Some(Action::Burn));
	}
	if is_flammable(&get(i as i32 + 1, j as i32, f_grid, mov_dt)) {
		set_action(i as i32 + 1, j as i32, f_grid, mov_dt, Some(Action::Burn));
	}
	
	if !apply_velocity(f_grid, i, j, mov_dt) {
		mov_dt.dirty_rect.set_temp(i, j);
	}

	true
}
