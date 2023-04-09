use std::collections::HashMap;

use notan::{prelude::*, draw::*};

use crate::{grid::{Grid, UPSCALE_FACTOR, COLS, ROWS}, camera::Camera2D, input_manager::get_mouse_in_world, element::{Cell, sand_element}};

pub struct ChunkManager {
	chunks: HashMap<(i32, i32), Grid>,
    pub selected_element: Cell,
    pub modify: bool,
    pub brush_size: i32,
	pub update_chunks: bool,
	pub hovering_cell: Cell,
	font: Font
}

impl ChunkManager {
	pub fn new(gfx: &mut Graphics) -> Self {
		let mut chunks = HashMap::new();
		for i in -1..=1 {
			for j in -1..=1 {
				chunks.insert((i, j), Grid::new(i, j, gfx));
			}
		}
		
		Self {
			chunks,
	        selected_element: sand_element(),
	        modify: true,
			brush_size: 32,
			update_chunks: true,
			hovering_cell: sand_element(),
			font: gfx.create_font(include_bytes!("assets/Ubuntu-B.ttf")).unwrap()
		}
	}

	pub fn update(&mut self, app: &mut App, camera: &Camera2D) {
	    let mouse_world = get_mouse_in_world(&(app.mouse.x, app.mouse.y), (app.window().width(), app.window().height()), &camera);

		let mut grid_map = HashMap::new();
		for (key, chunk) in self.chunks.iter() {
			grid_map.insert(key.to_owned(), chunk.grid.clone());
		}

		let mut keys = Vec::new();
		for (key, _) in self.chunks.iter() {
			keys.push(key.to_owned());
		}
		
		for key in &keys {
			let chunk = self.chunks.get_mut(key).unwrap();
			
		    let mouse = chunk.mouse_in_sim(mouse_world);

            if app.mouse.left_is_down() && self.modify {
                chunk.modify_elements(mouse.0, mouse.1, self.brush_size, &self.selected_element);
            }

            if app.mouse.right_is_down() && self.modify {
                chunk.explode(mouse.0, mouse.1, self.brush_size * 2, 4.);
            }

			match chunk.get_cell(mouse.0, mouse.1) {
				Some(c) => self.hovering_cell = c.to_owned(),
				_ => ()
			}

			if self.update_chunks {
				for swap in &chunk.update(&grid_map) {
					if self.chunks.contains_key(&(swap.0, swap.1)) {
						self.chunks.get_mut(&(swap.0, swap.1)).unwrap().grid[swap.2][swap.3] = swap.4;
					}
				}
			}
		}
	}

	pub fn render(&mut self, gfx: &mut Graphics, draw: &mut Draw, debug_render: bool) {
		for index in self.chunks.keys() {
			draw.text(&self.font, &format!("{:?}", index))
				.position((COLS as f32 / 2. + (index.0 as f32 * COLS as f32)) * UPSCALE_FACTOR, (ROWS as f32 / 2. + (index.1 as f32 * ROWS as f32)) * UPSCALE_FACTOR)
				.h_align_center()
				.v_align_middle();
		}

		for chunk in self.chunks.values_mut() {
			chunk.render(gfx, draw, debug_render);
		}
	}
}
