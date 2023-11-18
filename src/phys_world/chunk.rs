use notan::{graphics::Texture, draw::{Draw, DrawImages}, prelude::Graphics, math::Vec2};

use crate::{phys_world::element::*, phys_world::el_movement::*, phys_world::chunk_manager::WorldChunks, phys_world::element_actions::handle_actions};

pub const COLS: usize = 256;
pub const ROWS: usize = 144;
pub const UPSCALE_FACTOR: f32 = 2.;

pub type Grid = Box<[[Cell; ROWS]; COLS]>;

pub struct Chunk {
	pub pos: (f32, f32),
	pub index: (i32, i32),
	pub grid: Grid,
	pub future_grid: Grid,
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
			.from_bytes(&bytes, COLS as u32, ROWS as u32)
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
			dirty_rect: DirtyRect::default(),
			bytes,
			texture
		}
	}
}

fn create_cells_array() -> Grid {
    let mut data = std::mem::ManuallyDrop::new(vec![air_element(); ROWS * COLS]);
    unsafe {
        Box::from_raw(data.as_mut_ptr() as *mut [[Cell; ROWS]; COLS])
    }
}

pub struct MovData<'a> {
	pub chunks: &'a mut WorldChunks,
	pub index: (i32, i32),
	pub keep_active: &'a mut bool,
	pub dirty_rect: &'a mut DirtyRect,
	pub bytes: &'a mut Vec<u8>
}

pub fn update_chunk(chunk: &mut Chunk, chunks: &mut WorldChunks) {
	if !chunk.active {
		return;
	}

	chunk.future_grid = chunk.grid.clone();

	let mut keep_active = false;

	let flip_x = fastrand::bool();
	for i_loop in chunk.dirty_rect.min_xy.0..=chunk.dirty_rect.max_xy.0 {
		let flip_y = fastrand::bool();
		for j_loop in chunk.dirty_rect.min_xy.1..=chunk.dirty_rect.max_xy.1 {

			let i = if flip_x { chunk.dirty_rect.max_xy.0 - (i_loop - chunk.dirty_rect.min_xy.0) } else { i_loop };
			let j = if flip_y { chunk.dirty_rect.max_xy.1 - (j_loop - chunk.dirty_rect.min_xy.1) } else { j_loop };

			/* TODO: Bug: Elements when moving into other chunk sometimes do not activate the chunk and get stuck.
      			This affects all elements, happens only sometimes, issue unknown */
			
			// TODO: Change the way the update_byte function is called

			let mut mov_dt = MovData {
				chunks,
				index: chunk.index,
				keep_active: &mut keep_active,
				dirty_rect: &mut chunk.dirty_rect,
				bytes: &mut chunk.bytes
			};

			if chunk.grid[i][j].element == chunk.future_grid[i][j].element {
				match chunk.grid[i][j].element {
					Element::Sand | Element::Dirt => {
						falling_sand(&mut chunk.future_grid, i, j, &mut mov_dt);
					},
					Element::SawDust => {
						handle_actions(&mut chunk.future_grid, i, j, &mut mov_dt);
						falling_sand(&mut chunk.future_grid, i, j, &mut mov_dt);
					},
					Element::Water | Element::Petrol => {
						handle_actions(&mut chunk.future_grid, i, j, &mut mov_dt);
						liquid_movement(&mut chunk.future_grid, i, j, &mut mov_dt);
					},
					Element::Steam | Element::Smoke => {
						gas_movement(&mut chunk.future_grid, i, j, &mut mov_dt);
					},
					Element::Methane => {
						handle_actions(&mut chunk.future_grid, i, j, &mut mov_dt);
						gas_movement(&mut chunk.future_grid, i, j, &mut mov_dt);
					},
					Element::Fire => {
						fire_movement(&mut chunk.future_grid, i, j, &mut mov_dt);
					},
					Element::Wood | Element::Coal => handle_actions(&mut chunk.future_grid, i, j, &mut mov_dt),
					_ => ()
				}
			}
		}
	}
	chunk.dirty_rect.set_min_max();

	chunk.active = keep_active;
	chunk.dirty_tex = true;

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
						if in_bound(i - x, j - y) {
							if chunk.grid[(i - x) as usize][(j - y) as usize].element == Element::Air {
								modify_chunk_element(chunk, i - x, j - y, cell);
							}
						}
					} else {
						modify_chunk_element(chunk, i - x, j - y, cell);
					}
				}
			}
		}
	} else {
		if in_bound(i, j) {
			if empty_only && cell.element != Element::Air {
				if chunk.grid[i as usize][j as usize].element == Element::Air {
					modify_chunk_element(chunk, i, j, cell);
				}
			} else {
				modify_chunk_element(chunk, i, j, cell);
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

		chunk.dirty_tex = true;

		if !chunk.active {
			activate(chunk);
		} else {
			chunk.dirty_rect.set_temp(i as usize, j as usize);
		}
	}
}

pub fn explode_chunk(chunk: &mut Chunk, i: i32, j: i32, radius: i32, force: f32) {
	for x in -radius / 2..=radius / 2 {
		for y in -radius / 2..radius / 2 {
			if ((i - (i - x)).pow(2) + (j - (j - y)).pow(2)) <= (radius / 2).pow(2)  {
				if in_bound(i - x, j - y) {
					if chunk.grid[(i - x) as usize][(j - y) as usize].state != State::Solid && chunk.grid[(i - x) as usize][(j - y) as usize].element != Element::Air {
						let mut angle = Vec2::new(x as f32, y as f32);
						angle = angle.normalize_or_zero() * force * -1.;
						chunk.grid[(i - x) as usize][(j - y) as usize].velocity += angle;
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

	mouse_pos
}


pub fn in_bound(i: i32, j: i32) -> bool {
	i >= 0 && j >= 0 && i < COLS as i32 && j < ROWS as i32
}

pub fn render_chunk(chunk: &mut Chunk, gfx: &mut Graphics, draw: &mut Draw, update_chunks: bool) {
	update_chunk_tex_data(chunk, gfx, update_chunks);

	draw.image(&chunk.texture)
		.size(COLS as f32 * UPSCALE_FACTOR, ROWS as f32 * UPSCALE_FACTOR)
		.position(chunk.pos.0, chunk.pos.1);
}

fn update_chunk_tex_data(chunk: &mut Chunk, gfx: &mut Graphics, update_chunks: bool) {
	if chunk.dirty_tex {
		update_bytes(chunk, update_chunks);
		gfx.update_texture(&mut chunk.texture)
    		.with_data(&chunk.bytes)
    		.update()
    		.unwrap();

		chunk.dirty_tex = false;
	}
}

pub fn update_byte(bytes: &mut Vec<u8>, i: usize, j: usize, color: &[u8; 4]) {
	let index = j * COLS + i;
	bytes[index * 4..index * 4 + 4].copy_from_slice(color);
}

fn update_bytes(chunk: &mut Chunk, update_chunks: bool) {
	if !update_chunks {
		for i in 0..chunk.bytes.len() / 4 {
			chunk.bytes[i * 4..i * 4 + 4].copy_from_slice(&chunk.grid[i % COLS][i / COLS].color);
		}
	} else {
		for i in chunk.dirty_rect.min_xy.0..=chunk.dirty_rect.max_xy.0 {
			for j in chunk.dirty_rect.min_xy.1..=chunk.dirty_rect.max_xy.1 {
				let index = j * COLS + i;

				chunk.bytes[index * 4..index * 4 + 4].copy_from_slice(&chunk.grid[i][j].color);
			
			}
		}
	}
}

pub struct DirtyRect {
	pub min_xy: (usize, usize),
	pub max_xy: (usize, usize),
	pub temp_min_xy: (usize, usize),
	pub temp_max_xy: (usize, usize),
}

impl DirtyRect {
	pub fn set_temp(&mut self, i: usize, j: usize) {
		// INFO: we have to get these in i32 format and the cast to usize because usize value flips to max value when index is smaller than amount
		let amount = 10;
		let (min_x, min_y) = ((i as i32 - amount).clamp(0, COLS as i32 - 1), (j as i32 - amount).clamp(0, ROWS as i32 - 1));
		let (max_x, max_y) = ((i as i32 + amount).clamp(0, COLS as i32 - 1), (j as i32 + amount).clamp(0, ROWS as i32 - 1));

		self.temp_min_xy.0 = (min_x as usize).min(self.temp_min_xy.0);
		self.temp_min_xy.1 = (min_y as usize).min(self.temp_min_xy.1);
		self.temp_max_xy.0 = (max_x as usize).max(self.temp_max_xy.0);
		self.temp_max_xy.1 = (max_y as usize).max(self.temp_max_xy.1);
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

impl Default for DirtyRect {
    fn default() -> Self {
        Self {
			min_xy: (0, 0),
			max_xy: (COLS - 1, ROWS - 1),
			temp_min_xy: (COLS - 1, ROWS - 1),
			temp_max_xy: (0, 0)
		}
    }
}
