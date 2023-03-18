use notan::{graphics::Texture, draw::{Draw, DrawImages, DrawShapes}, prelude::{Graphics, App, Color}, math::Vec2};

use crate::{element::*, movement::{downward, downward_sides, apply_velocity, apply_gravity, upward, sideways_gas}};

pub const COLS: usize = 350;
pub const ROWS: usize = 350;
pub const UPSCALE_FACTOR: f32 = 2.;

pub struct Grid {
	pos: (f32, f32),
	grid: Box<[[Cell; ROWS]; COLS]>,
	future_grid: Box<[[Cell; ROWS]; COLS]>,
	texture: Texture,
	bytes: Vec<u8>,
	// current_rect: DirtyRect,
	// working_rect: DirtyRect
}

impl Grid {
	pub fn new(x: f32, y: f32, gfx: &mut Graphics) -> Self {
		let bytes = vec![0; COLS * ROWS * 4];

		let texture = gfx
			.create_texture()
			.from_bytes(&bytes, COLS as i32, ROWS as i32)
			.build()
			.unwrap();

		let mut grid = create_cells_array();

		for i in 0..COLS {
			grid[i][0] = solid_element();
			grid[i][ROWS - 1] = solid_element();
		}
		for j in 0..ROWS {
			grid[0][j] = solid_element();
			grid[COLS - 1][j] = solid_element();
		}

		let future_grid = grid.clone();
		
		Self {
			pos: (x, y),
			grid,
			future_grid,
			texture,
			bytes,
			// current_rect: DirtyRect::new(COLS, ROWS, 0, 0),
			// working_rect: DirtyRect::new(COLS, ROWS, 0, 0)
		}
	}

	pub fn update(&mut self) {
		self.future_grid = self.grid.clone();

		let flip_x = fastrand::bool();
		for mut i in 0..COLS {
			let flip_y = fastrand::bool();
			for mut j in 0..ROWS {
				if flip_x {
					i = COLS - i - 1;
				}
				if flip_y {
					j = ROWS - j - 1;
				}
				if self.grid[i][j].element == self.future_grid[i][j].element {
					match self.grid[i][j].element {
						Element::Sand => {
							apply_gravity(&mut self.future_grid, i, j);

							if !apply_velocity(&mut self.future_grid, i, j) {
								if !downward(&mut self.future_grid, i, j) {
									if !downward_sides(&mut self.future_grid, i, j) {
										self.future_grid[i][j].velocity = Vec2::ZERO;
									}
								}
							}
						},
						Element::SawDust => {
							apply_gravity(&mut self.future_grid, i, j);
							
							if !apply_velocity(&mut self.future_grid, i, j) {
								if !downward(&mut self.future_grid, i, j) {
									if !downward_sides(&mut self.future_grid, i, j) {
										self.future_grid[i][j].velocity = Vec2::ZERO;
									}
								}
							}
						},

						Element::Water => {
							apply_gravity(&mut self.future_grid, i, j);
							
							if !apply_velocity(&mut self.future_grid, i, j) {
								if !downward(&mut self.future_grid, i, j) {
									self.future_grid[i][j].velocity = Vec2::ZERO;

									let mut dir = 0.;

									if fastrand::bool() {
										if self.future_grid[i - 1][j].density <= self.future_grid[i][j].density {
											dir = -1.;
										} else if self.future_grid[i + 1][j].density <= self.future_grid[i][j].density {
											dir = 1.;
										}
									} else {
										if self.future_grid[i + 1][j].density <= self.future_grid[i][j].density {
											dir = 1.;
										} else if self.future_grid[i - 1][j].density <= self.future_grid[i][j].density {
											dir = -1.;
										}	
									}

									
									if dir != 0. {	
										self.future_grid[i][j].velocity.x += 5. * dir;
									}
								}
							}
						},
						Element::Smoke => {
							if !upward(&mut self.future_grid, i, j) {
								sideways_gas(&mut self.future_grid, i, j, 10);
							}
						}
						_ => ()
					}
				}
			}
		}
		self.grid = self.future_grid.clone();
	}

	pub fn render(&mut self, gfx: &mut Graphics, draw: &mut Draw) {
		self.update_bytes();

		gfx.update_texture(&mut self.texture)
        	.with_data(&self.bytes)
        	.update()
        	.unwrap();
		
		draw.image(&self.texture).size(COLS as f32 * UPSCALE_FACTOR, ROWS as f32 * UPSCALE_FACTOR).position(self.pos.0, self.pos.1);

		// let pos = (self.current_rect.x as f32, self.current_rect.y as f32);
		// draw.rect((pos.0 * 2., pos.1 * 2.) , ((pos.0 + self.current_rect.w as f32) * 2., (pos.1 + self.current_rect.h as f32) * 2.))
		// 	.fill_color(Color::from_rgba(0., 0., 0., 0.))
		// 	.stroke_color(Color::RED)
		// 	.stroke(1.);
	}

	fn update_bytes(&mut self) {
		for i in 0..self.bytes.len() / 4 {
			self.bytes[i * 4..i * 4 + 4].copy_from_slice(&self.grid[i % COLS][i / COLS].color);
		}
	}

	pub fn modify_elements(&mut self, i: usize, j: usize, brush_size: i32, cell: &Cell) {
		for x in -brush_size / 2..=brush_size / 2 {
			for y in -brush_size / 2..brush_size / 2 {
				if ((i as i32 - (i as i32 - x)).pow(2) + (j as i32 - (j as i32 - y)).pow(2)) <= (brush_size / 2).pow(2)  {
					self.modify_element((i as i32 - x) as usize, (j as i32 - y) as usize, cell);
				}
			}
		}
	}

	pub fn modify_element(&mut self, i: usize, j: usize, cell: &Cell) {
		if in_bound(i, j) {
			let mut c_cell = cell.to_owned();
			let amount = 40;
			let mut c = fastrand::u8(0..=amount);

			if c_cell.color[0] < c || c_cell.color[1] < c || c_cell.color[2] < c {
				c = 0;
			}
			
			c_cell.color = [cell.color[0] - c, cell.color[1] - c, cell.color[2] - c, cell.color[3]];
			self.grid[i][j] = c_cell;
		}
	}

	pub fn explode(&mut self, i: usize, j: usize, radius: i32, force: f32) {
		for x in -radius / 2..=radius / 2 {
			for y in -radius / 2..radius / 2 {
				if ((i as i32 - (i as i32 - x)).pow(2) + (j as i32 - (j as i32 - y)).pow(2)) <= (radius / 2).pow(2)  {
					if in_bound((i as i32 - x) as usize, (j as i32 - y) as usize) {
						let mut angle = Vec2::new(x as f32, y as f32);
						angle = angle.normalize_or_zero() * force * -1.;

						self.grid[(i as i32 - x) as usize][(j as i32 - y) as usize].velocity += angle;
					}
				} 
			}
		}
	}

	pub fn get_cell(&self, i: usize, j: usize) -> &Cell {
		&self.grid[i][j]
	}

	pub fn mouse_in_sim(&self, mouse_world: (f32, f32), app: &mut App) -> Option<(usize, usize)> {
		if mouse_world.0 > self.pos.0 && mouse_world.1 > self.pos.1 && mouse_world.0 < self.pos.0 + COLS as f32 * UPSCALE_FACTOR as f32 && mouse_world.1 < self.pos.1 + ROWS as f32 * UPSCALE_FACTOR as f32 {
			let mut mouse_pos = (0, 0);
			mouse_pos.0 = ((mouse_world.0 - self.pos.0) / UPSCALE_FACTOR) as usize;
			mouse_pos.1 = ((mouse_world.1 - self.pos.1) / UPSCALE_FACTOR) as usize;

			return Some(mouse_pos);
		}
		None
	}
}

// #[derive(Clone, Copy)]
// struct DirtyRect {
// 	pub x: usize,
// 	pub y: usize,
// 	pub w: usize,
// 	pub h: usize
// }

// impl DirtyRect {
// 	pub fn new(x: usize, y: usize, w: usize, h: usize) -> DirtyRect {
// 		DirtyRect { x, y, w, h }
// 	}

// 	pub fn update(&mut self, i: usize, j: usize) {
// 		if i < self.x {
// 			self.x = i - 1;
// 		}
// 		if j < self.y {
// 			self.y = j - 1;
// 		}
// 		if i > self.x + self.w {
// 			self.w = i - self.x + 1;
// 		}
// 		if j > self.y + self.h {
// 			self.h = j - self.y + 1;
// 		}
// 	}
// }

fn in_bound(i: usize, j: usize) -> bool {
	return i > 0 && j > 0 && i < COLS - 1 && j < ROWS - 1
}

pub fn create_cells_array() -> Box<[[Cell; ROWS]; COLS]> {
    let mut data = std::mem::ManuallyDrop::new(vec![air_element(); ROWS * COLS]);
    
    unsafe {
        Box::from_raw(data.as_mut_ptr() as *mut [[Cell; ROWS]; COLS])
    }
}