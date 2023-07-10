use std::{collections::HashMap};

use notan::{prelude::*, draw::*};

use crate::{camera::Camera2D, input_manager::get_mouse_in_world, element::{Cell, sand_element}, chunk::{Chunk, self, COLS, UPSCALE_FACTOR, ROWS}};

const CHUNK_UPDATE_FPS: f32 = 60.;

pub struct ChunkManager {
	chunks: HashMap<(i32, i32), Chunk>,
    pub selected_element: Cell,
    pub modify: bool,
    pub brush_size: i32,
	pub update_chunks: bool,
	pub hovering_cell: Cell,
	pub update_time: f32,
	pub replace_air: bool,
	font: Font
}

impl ChunkManager {
	pub fn new(gfx: &mut Graphics) -> Self {
		let mut chunks = HashMap::new();
		for i in -2..=2 {
			for j in -2..=2 {
				chunks.insert((i, j), Chunk::new(i, j, gfx));
			}
		}
		
		Self {
			chunks,
	        selected_element: sand_element(),
	        modify: true,
			brush_size: 32,
			update_chunks: true,
			hovering_cell: sand_element(),
			update_time: 0.,
			replace_air: true,
			font: gfx.create_font(include_bytes!("assets/UbuntuMono.ttf")).unwrap()
		}
	}

	pub fn update(&mut self, app: &mut App, camera: &Camera2D) {
	    let mouse_world = get_mouse_in_world(&(app.mouse.x, app.mouse.y), (app.window().width(), app.window().height()), &camera);

		let mut keys = Vec::new();
		for (key, _) in self.chunks.iter() {
			keys.push(key.to_owned());
		}

		for key in keys.iter() {
			let chunk = self.chunks.get_mut(key).unwrap();
		    let mouse = chunk::mouse_in_chunk(chunk, mouse_world);

            if app.mouse.left_is_down() && self.modify {
                chunk::modify_chunk_elements(chunk, mouse.0, mouse.1, self.brush_size, &self.selected_element, self.replace_air);
            }

			if app.mouse.right_is_down() && self.modify {
                chunk::explode_chunk(chunk, mouse.0, mouse.1, self.brush_size * 2, 4. * app.timer.delta_f32() * 90.);
            }

			if let Some(c) = chunk::get_chunk_cell(chunk, mouse.0, mouse.1) {
				self.hovering_cell = c.to_owned();
			}
		}

		self.update_chunks(app, keys);
	}

	fn update_chunks(&mut self, app: &mut App, keys: Vec<(i32, i32)>) {
		self.update_time += app.timer.delta_f32();
		if self.update_time >= 1. / CHUNK_UPDATE_FPS && self.update_chunks {
			for key in &keys {
				let mut chunk = self.chunks.remove(key).unwrap();
				chunk::update_chunk(&mut chunk, &mut self.chunks);
				self.chunks.insert(key.to_owned(), chunk);
			}
			self.update_time = 0.;
		}
	}

	pub fn render(&mut self, gfx: &mut Graphics, draw: &mut Draw, debug_render_boundaries: bool, debug_chunk_coords: bool) {
		if debug_chunk_coords {
			for index in self.chunks.keys() {
				draw.text(&self.font, &format!("{:?}", index))
					.position((COLS as f32 / 2. + (index.0 as f32 * COLS as f32)) * UPSCALE_FACTOR, (ROWS as f32 / 2. + (index.1 as f32 * ROWS as f32)) * UPSCALE_FACTOR)
					.h_align_center()
					.v_align_middle();
			}
		}

		if debug_render_boundaries {
			for index in self.chunks.keys() {
				draw.rect((index.0 as f32 * COLS as f32 * UPSCALE_FACTOR + 0.5, index.1 as f32 * ROWS as f32 * UPSCALE_FACTOR + 0.5), (COLS as f32 * UPSCALE_FACTOR - 1., ROWS as f32 * UPSCALE_FACTOR - 1.))
					.fill_color(Color::from_rgba(0., 0., 0., 0.))
					.stroke_color(if self.chunks.get(index).unwrap().active { Color::GREEN } else { Color::RED })
					.stroke(1.);
			}
		}

		for chunk in self.chunks.values_mut() {
			chunk::render_chunk(chunk, gfx, draw);
		}
	}
}
