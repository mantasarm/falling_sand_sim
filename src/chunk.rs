use std::collections::HashMap;

use notan::{graphics::Texture, draw::{Draw, DrawImages}, prelude::Graphics, math::Vec2};

use crate::{element::*, el_movement::*};

pub const COLS: usize = 256 / 2;
pub const ROWS: usize = 144 / 2;
pub const UPSCALE_FACTOR: f32 = 2.;

pub struct Chunk {
	pub pos: (f32, f32),
	pub index: (i32, i32),
	pub grid: Box<[[Cell; ROWS]; COLS]>,
	pub future_grid: Box<[[Cell; ROWS]; COLS]>,
	pub active: bool,
	pub dirty_tex: bool,
	pub dirty_rect: DirtyRect,
	bytes: Vec<u8>,
	texture: Texture,
}

impl Chunk {
	pub fn new(i: i32, j: i32, gfx: &mut Graphics) -> Self {
		let bytes = vec![0; COLS * ROWS * 4];

		let texture = gfx
			.create_texture()
			.from_bytes(&bytes, COLS as i32, ROWS as i32)
			.build()
			.unwrap();

		let grid = create_cells_array();
		let future_grid = grid.clone();
		
		Self {
			pos: (i as f32 * COLS as f32 * UPSCALE_FACTOR, j as f32 * ROWS as f32 * UPSCALE_FACTOR),
			index: (i, j),
			grid,
			future_grid,
			active: true,
			dirty_tex: true,
			dirty_rect: DirtyRect::new(),
			bytes,
			texture
		}
	}
}

fn create_cells_array() -> Box<[[Cell; ROWS]; COLS]> {
    let mut data = std::mem::ManuallyDrop::new(vec![air_element(); ROWS * COLS]);
    unsafe {
        Box::from_raw(data.as_mut_ptr() as *mut [[Cell; ROWS]; COLS])
    }
}

pub fn update_chunk(chunk: &mut Chunk, chunks: &mut HashMap<(i32, i32), Chunk>) {
	if !chunk.active {
		return;
	}

	chunk.future_grid = chunk.grid.clone();

	let mut keep_active = false;
	
	let flip_x = fastrand::bool();
	for i_loop in 0..COLS	/*chunk.dirty_rect.min_xy.0..=chunk.dirty_rect.max_xy.0*/ {
		let flip_y = fastrand::bool();
		for j_loop in 0..ROWS /*chunk.dirty_rect.min_xy.1..=chunk.dirty_rect.max_xy.1*/ {

			let i = if flip_x { COLS - i_loop - 1 } else { i_loop };
			let j = if flip_y { ROWS - j_loop - 1 } else { j_loop };

			if chunk.grid[i][j].element == chunk.future_grid[i][j].element {
				match chunk.grid[i][j].element {
					Element::Sand | Element::SawDust | Element::Dirt => {
						if falling_sand(&mut chunk.future_grid, i, j, chunks, chunk.index) {
							keep_active = true;
							chunk.dirty_rect.set_temp(i, j);	
						}
					},
					Element::Water => {
						if liquid_movement(&mut chunk.future_grid, i, j, chunks, chunk.index) {
							keep_active = true;
						};
					},
					Element::Smoke => {
						if gas_movement(&mut chunk.future_grid, i, j, chunks, chunk.index) {
							keep_active = true;
						};
					},
					Element::Steam => {
						if gas_movement(&mut chunk.future_grid, i, j, chunks, chunk.index) {
							keep_active = true;
						};
					},
					_ => ()
				}
			}
		}
	}

	chunk.dirty_rect.set_min_max();
	if keep_active {
		chunk.dirty_tex = true;
		chunk.active = true;
	} else {
		chunk.active = false;
		chunk.dirty_tex = false;
	}
	chunk.grid = chunk.future_grid.clone();
}

pub fn activate(chunk: &mut Chunk) {
	chunk.active = true;
	chunk.dirty_tex = true;
	chunk.dirty_rect.reset();
}

pub fn modify_chunk_elements(chunk: &mut Chunk, i: i32, j: i32, brush_size: i32, cell: &Cell, empty_only: bool) {
	if brush_size != 1 {
		for x in -brush_size / 2..=brush_size / 2 {
			for y in -brush_size / 2..brush_size / 2 {
				if (((i as f32 + 0.5) - (i as f32 - x as f32)).powf(2.) + ((j as f32 + 0.5) - (j as f32 - y as f32)).powf(2.)) <= (brush_size as f32 / 2.).powf(2.) {
					if empty_only && cell.element != Element::Air {
						if in_bound(i as i32 - x, j as i32 - y) {
							if chunk.grid[(i as i32 - x) as usize][(j as i32 - y) as usize].element == Element::Air {
								modify_chunk_element(chunk, i as i32 - x, j as i32 - y, cell);
							}
						}
					} else {
						modify_chunk_element(chunk, i as i32 - x, j as i32 - y, cell);
					}
				}
			}
		}
	} else {
		if in_bound(i as i32, j as i32) {
			if empty_only && cell.element != Element::Air {
				if chunk.grid[i as usize][j as usize].element == Element::Air {
					modify_chunk_element(chunk, i as i32, j as i32, cell);
				}
			} else {
				modify_chunk_element(chunk, i as i32, j as i32, cell);
			}
		}
	}
}

pub fn modify_chunk_element(chunk: &mut Chunk, i: i32, j: i32, cell: &Cell) {
	if in_bound(i, j) {
		let mut c_cell = cell.to_owned();

		let amount = 40;
		let mut c = fastrand::u8(0..=amount);
		if c_cell.color[0] < c || c_cell.color[1] < c || c_cell.color[2] < c {
			c = 0;
		}
		
		c_cell.color = [cell.color[0] - c, cell.color[1] - c, cell.color[2] - c, cell.color[3]];
		chunk.grid[i as usize][j as usize] = c_cell;

		activate(chunk);
	}
}

pub fn explode_chunk(chunk: &mut Chunk, i: i32, j: i32, radius: i32, force: f32) {
	for x in -radius / 2..=radius / 2 {
		for y in -radius / 2..radius / 2 {
			if ((i as i32 - (i as i32 - x)).pow(2) + (j as i32 - (j as i32 - y)).pow(2)) <= (radius / 2).pow(2)  {
				if in_bound(i as i32 - x, j as i32 - y) {
					if chunk.grid[(i as i32 - x) as usize][(j as i32 - y) as usize].state != State::Solid && chunk.grid[(i as i32 - x) as usize][(j as i32 - y) as usize].element != Element::Air {
						let mut angle = Vec2::new(x as f32, y as f32);
						angle = angle.normalize_or_zero() * force * -1.;
						chunk.grid[(i as i32 - x) as usize][(j as i32 - y) as usize].velocity += angle;
						if angle.x.abs() > 0.5 && angle.y.abs() > 0.5 {
							activate(chunk)
						}
					}
				}
			} 
		}
	}
}

pub fn get_chunk_cell(chunk: &Chunk, i: i32, j: i32) -> Option<&Cell> {
	if in_bound(i, j) {
		return Some(&chunk.grid[i as usize][j as usize]);
	}
	None
}

pub fn mouse_in_chunk(chunk: &Chunk, mouse_world: (f32, f32)) -> (i32, i32) {
	let mut mouse_pos = (0, 0);
	mouse_pos.0 = ((mouse_world.0 - chunk.pos.0) / UPSCALE_FACTOR) as i32;
	mouse_pos.1 = ((mouse_world.1 - chunk.pos.1) / UPSCALE_FACTOR) as i32;

	return mouse_pos
}


pub fn in_bound(i: i32, j: i32) -> bool {
	return i >= 0 && j >= 0 && i < COLS as i32 && j < ROWS as i32
}

pub fn render_chunk(chunk: &mut Chunk, gfx: &mut Graphics, draw: &mut Draw) {
	update_chunk_tex_data(chunk, gfx);

	draw.image(&chunk.texture)
		.size(COLS as f32 * UPSCALE_FACTOR, ROWS as f32 * UPSCALE_FACTOR)
		.position(chunk.pos.0, chunk.pos.1);
}

fn update_chunk_tex_data(chunk: &mut Chunk, gfx: &mut Graphics) {
	if chunk.dirty_tex {
		update_bytes(chunk);
		gfx.update_texture(&mut chunk.texture)
    		.with_data(&chunk.bytes)
    		.update()
    		.unwrap();

			chunk.dirty_tex = false;
	}
}

fn update_bytes(chunk: &mut Chunk) {
	for i in 0..chunk.bytes.len() / 4 {
		chunk.bytes[i * 4..i * 4 + 4].copy_from_slice(&chunk.grid[i % COLS][i / COLS].color);
	}
}

pub struct DirtyRect {
	pub min_xy: (usize, usize),
	pub max_xy: (usize, usize),
	pub temp_min_xy: (usize, usize),
	pub temp_max_xy: (usize, usize),
}

impl DirtyRect {
	pub fn new() -> Self {
		Self {
			min_xy: (0, 0),
			max_xy: (COLS - 1, ROWS - 1),
			temp_min_xy: (COLS - 1, ROWS - 1),
			temp_max_xy: (0, 0)
		}
	}

	pub fn set_temp(&mut self, i: usize, j: usize) {
		self.temp_min_xy.0 = (i - 20).min(self.temp_min_xy.0).clamp(0, COLS - 1);
		self.temp_min_xy.1 = (j - 20).min(self.temp_min_xy.1).clamp(0, ROWS - 1);
		self.temp_max_xy.0 = (i + 20).max(self.temp_max_xy.0).clamp(0, COLS - 1);
		self.temp_max_xy.1 = (j + 20).max(self.temp_max_xy.1).clamp(0, ROWS - 1);
	}

	pub fn set_min_max(&mut self) {
		self.min_xy = self.temp_min_xy;
		self.max_xy = self.temp_max_xy;
		self.temp_min_xy = (COLS - 1, ROWS - 1);
		self.temp_max_xy = (0, 0);
	}

	pub fn reset(&mut self) {
		self.min_xy = (0, 0);
		self.max_xy = (COLS - 1, ROWS - 1);
		self.temp_min_xy = (COLS - 1, ROWS - 1);
		self.temp_max_xy = (0, 0);
	}
}