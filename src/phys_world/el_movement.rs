use notan::math::Vec2;

use crate::{phys_world::base_movement::*, phys_world::chunk::{Grid, MovData, self}};

use super::element::{air_element, Element, firework_ember_element, fire_element};

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

	if down_density >= f_grid[i][j].density && f_grid[i][j].velocity.x.abs() <= 7. {
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

		let acc = 3. * f_grid[i][j].drag;
		if right {
			if f_grid[i][j].velocity.x < 0. {
				f_grid[i][j].velocity.x = 0.;
			}
			f_grid[i][j].velocity.x += acc;
		} else if left {
			if f_grid[i][j].velocity.x > 0. {
				f_grid[i][j].velocity.x = 0.;
			}
			f_grid[i][j].velocity.x -= acc;
		}
	}

	if apply_velocity(f_grid, i, j, mov_dt) {
		*mov_dt.keep_active = true;

		return true;
	}

	false
}

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
	let rand = fastrand::i32(2..8);
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
	chunk::update_byte(mov_dt.bytes, i, j, &f_grid[i][j].color);

	spread_fire(f_grid, i, j, mov_dt);
	
	if !apply_velocity(f_grid, i, j, mov_dt) {
		mov_dt.dirty_rect.set_temp(i, j);
	}

	true
}

#[inline]
pub fn firework_shell_movement(f_grid: &mut Grid, i: usize, j: usize, mov_dt: &mut MovData) -> bool {
	*mov_dt.keep_active = true;

	let rand = fastrand::i32(2..7);
	if f_grid[i][j].lifetime <= 0 {
		f_grid[i][j] = air_element();
		chunk::update_byte(mov_dt.bytes, i, j, &[0, 0, 0, 0]);

		let size = fastrand::i32(30..=70);
		let density = fastrand::i32(4..=8) as f32;
		for x in -size..=size {
			for y in -size..=size {
				let dist = Vec2::new(x as f32, y as f32).length();
				if dist > size as f32 || dist < 15. {
					continue;
				}

				let vel = Vec2::new(x as f32, y as f32).normalize() * 5.;
				let mut temp_el = get(i as i32 + x, j as i32 + y, f_grid, mov_dt);
				temp_el.velocity = vel.clone();
				
				set(i as i32 + x, j as i32 + y, f_grid, mov_dt, temp_el);
				
				if fastrand::i32(1..=4) == 4 {
					let check_el = get(i as i32 + x, j as i32 + y, f_grid, mov_dt);
					if check_el.element == Element::Air {
						let mut firework = firework_ember_element();
						firework.velocity = vel.clone();
						firework.density = density;
						firework.lifetime = 130;
					
						set(i as i32 + x, j as i32 + y, f_grid, mov_dt, firework);
					}
				}
			}
		}

		return true;
	}

	if f_grid[i][j].velocity.y >= -7. {
		f_grid[i][j].velocity.y += -0.75;
	}

	let mut fire_trail = fire_element();
	fire_trail.lifetime = 30;
	set(i as i32, j as i32 + 1, f_grid, mov_dt, fire_trail);

	f_grid[i][j].lifetime -= rand;
	f_grid[i][j].velocity.x = (f_grid[i][j].lifetime as f32 / 8.).sin() * 2.;

	if !apply_velocity(f_grid, i, j, mov_dt) {
		mov_dt.dirty_rect.set_temp(i, j);
	}

	true
}

#[inline]
pub fn firework_ember_movement(f_grid: &mut Grid, i: usize, j: usize, mov_dt: &mut MovData) -> bool {
	let rand = fastrand::i32(2..8);
	f_grid[i][j].lifetime -= rand;

	*mov_dt.keep_active = true;

	if f_grid[i][j].lifetime <= 0 {
		f_grid[i][j] = air_element();
		chunk::update_byte(mov_dt.bytes, i, j, &[0, 0, 0, 0]);
		return true;
	}

	match f_grid[i][j].density {
		4. => {
			f_grid[i][j].color = lerp_rgb([255, 255, 255, 180], [14, 8, 184, 255], f_grid[i][j].lifetime as f32 / 100.);
		}
		5. => {
			f_grid[i][j].color = lerp_rgb([255, 255, 255, 180], [206, 32, 41, 255], f_grid[i][j].lifetime as f32 / 100.);
		}
		6. => {
			f_grid[i][j].color = lerp_rgb([255, 255, 0, 120], [255, 204, 0, 255], f_grid[i][j].lifetime as f32 / 100.);
		}
		7. => {
			f_grid[i][j].color = lerp_rgb([255, 255, 255, 180], [11, 217, 118, 255], f_grid[i][j].lifetime as f32 / 100.);
		}
		8. => {
			f_grid[i][j].color = lerp_rgb([255, 255, 255, 180], [159, 16, 140, 255], f_grid[i][j].lifetime as f32 / 100.);
		}
		_ => ()
	}
	chunk::update_byte(mov_dt.bytes, i, j, &f_grid[i][j].color);

	spread_fire(f_grid, i, j, mov_dt);
	
	apply_velocity(f_grid, i, j, mov_dt);
	mov_dt.dirty_rect.set_temp(i, j);

	true
}

pub fn lerp_rgb(color1: [u8; 4], color2: [u8; 4], t: f32) -> [u8; 4] {
    let t = t.clamp(0.0, 1.0);

    let r = (color1[0] as f32 + t * (color2[0] as f32 - color1[0] as f32)).round() as u8;
    let g = (color1[1] as f32 + t * (color2[1] as f32 - color1[1] as f32)).round() as u8;
    let b = (color1[2] as f32 + t * (color2[2] as f32 - color1[2] as f32)).round() as u8;
    let a = (color1[3] as f32 + t * (color2[3] as f32 - color1[3] as f32)).round() as u8;

    [r, g, b, a]
}
