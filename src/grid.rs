use std::collections::HashMap;

use notan::{graphics::Texture, draw::{Draw, DrawImages, DrawShapes}, prelude::{Graphics, Color}, math::Vec2};

use crate::{element::*, movement::*};

pub const COLS: usize = 200;
pub const ROWS: usize = 150;
pub const UPSCALE_FACTOR: f32 = 2.;

pub struct Chunk {
	pub pos: (f32, f32),
	pub index: (i32, i32),
	pub grid: Box<[[Cell; ROWS]; COLS]>,
	pub future_grid: Box<[[Cell; ROWS]; COLS]>,
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

pub fn update_chunk(chunk: &mut Chunk, chunks: &HashMap<(i32, i32), Box<[[Cell; ROWS]; COLS]>>) -> Vec<(i32, i32, usize, usize, Cell)> {
	chunk.future_grid = chunk.grid.clone();
	let mut chunk_swaps = Vec::new();

	for mut i in 0..COLS {
		let flip_x = fastrand::bool();
		let flip_y = fastrand::bool();
		for mut j in 0..ROWS {
			if flip_x {
				i = COLS - i - 1;
			}
			if flip_y {
				j = ROWS - j - 1;
			}
			if chunk.grid[i][j].element == chunk.future_grid[i][j].element {
				match chunk.grid[i][j].element {
					Element::Sand => {
						apply_gravity(&mut chunk.future_grid, i, j, &chunks, chunk.index);
						if !apply_velocity(&mut chunk.future_grid, i, j, &chunks, chunk.index, &mut chunk_swaps) {
							if !downward(&mut chunk.future_grid, i, j, &chunks, chunk.index, &mut chunk_swaps) {
								if !downward_sides(&mut chunk.future_grid, i, j, &chunks, chunk.index, &mut chunk_swaps) {
									chunk.future_grid[i][j].velocity = Vec2::ZERO;
								}
							}
						}
					},
					Element::SawDust => {
						apply_gravity(&mut chunk.future_grid, i, j, &chunks, chunk.index);
						
						if !apply_velocity(&mut chunk.future_grid, i, j, &chunks, chunk.index, &mut chunk_swaps) {
							if !downward(&mut chunk.future_grid, i, j, &chunks, chunk.index, &mut chunk_swaps) {
								if !downward_sides(&mut chunk.future_grid, i, j, &chunks, chunk.index, &mut chunk_swaps) {
									chunk.future_grid[i][j].velocity = Vec2::ZERO;
								}
							}
						}
					},
					Element::Water => {
						apply_gravity(&mut chunk.future_grid, i, j, &chunks, chunk.index);
						
						if !apply_velocity(&mut chunk.future_grid, i, j, &chunks, chunk.index, &mut chunk_swaps) {
							if !downward(&mut chunk.future_grid, i, j, &chunks, chunk.index, &mut chunk_swaps) {
								let mut dir = 0.;
								let left_element = get(i, j, i as i32 - 1, j as i32, &mut chunk.future_grid, &chunks, chunk.index);
								let right_element = get(i, j, i as i32 + 1, j as i32, &mut chunk.future_grid, &chunks, chunk.index);
								if left_element.density <= chunk.future_grid[i][j].density && right_element.density <= chunk.future_grid[i][j].density {
									if fastrand::bool() {
										dir = -1.;
									} else {
										dir = 1.;
									}
								} else if left_element.density <= chunk.future_grid[i][j].density {
									dir = -1.;
								} else if right_element.density <= chunk.future_grid[i][j].density{
									dir = 1.;
								}
								
								if dir != 0. {	
									chunk.future_grid[i][j].velocity.x += 5.5 * dir;
								} else {
									chunk.future_grid[i][j].velocity = Vec2::ZERO;
								}
							}
						}
					},
					Element::Smoke => {
						if !upward(&mut chunk.future_grid, i, j, &chunks, chunk.index, &mut chunk_swaps) {
							sideways_gas(&mut chunk.future_grid, i, j, 10, &chunks, chunk.index, &mut chunk_swaps);
						}
					},
					Element::Steam => {
						if !upward(&mut chunk.future_grid, i, j, &chunks, chunk.index, &mut chunk_swaps) {
							sideways_gas(&mut chunk.future_grid, i, j, 10, &chunks, chunk.index, &mut chunk_swaps);
						}
					},
					_ => ()
				}
			}
		}
	}
	chunk.grid = chunk.future_grid.clone();
	chunk_swaps
}

pub fn modify_chunk_elements(chunk: &mut Chunk, i: i32, j: i32, brush_size: i32, cell: &Cell) {
	for x in -brush_size / 2..=brush_size / 2 {
		for y in -brush_size / 2..brush_size / 2 {
			if ((i as i32 - (i as i32 - x)).pow(2) + (j as i32 - (j as i32 - y)).pow(2)) <= (brush_size / 2).pow(2)  {
				modify_chunk_element(chunk, i as i32 - x, j as i32 - y, cell);
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
	}
}

pub fn explode_chunk(chunk: &mut Chunk, i: i32, j: i32, radius: i32, force: f32) {
	for x in -radius / 2..=radius / 2 {
		for y in -radius / 2..radius / 2 {
			if ((i as i32 - (i as i32 - x)).pow(2) + (j as i32 - (j as i32 - y)).pow(2)) <= (radius / 2).pow(2)  {
				if in_bound(i as i32 - x, j as i32 - y) {
					let mut angle = Vec2::new(x as f32, y as f32);
					angle = angle.normalize_or_zero() * force * -1.;
					chunk.grid[(i as i32 - x) as usize][(j as i32 - y) as usize].velocity += angle;
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

pub fn render_chunk(chunk: &mut Chunk, gfx: &mut Graphics, draw: &mut Draw, debug_render: bool) {
	update_bytes(chunk);
	gfx.update_texture(&mut chunk.texture)
    	.with_data(&chunk.bytes)
    	.update()
    	.unwrap();
	
	draw.image(&chunk.texture).size(COLS as f32 * UPSCALE_FACTOR, ROWS as f32 * UPSCALE_FACTOR).position(chunk.pos.0, chunk.pos.1);
	if debug_render {
		draw.rect((chunk.pos.0, chunk.pos.1), (COLS as f32 * UPSCALE_FACTOR, ROWS as f32 * UPSCALE_FACTOR))
			.fill_color(Color::from_rgba(0., 0., 0., 0.))
			.stroke_color(Color::RED)
			.stroke(1.);
	}
}

fn update_bytes(chunk: &mut Chunk) {
	for i in 0..chunk.bytes.len() / 4 {
		chunk.bytes[i * 4..i * 4 + 4].copy_from_slice(&chunk.grid[i % COLS][i / COLS].color);
	}
}