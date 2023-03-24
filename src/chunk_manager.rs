use notan::{prelude::*, draw::*};

use crate::{grid::Grid, camera::Camera2D, input_manager::get_mouse_in_world, element::{Cell, sand_element}};

pub struct ChunkManager {
	grid: Grid,
    pub selected_element: Cell,
    pub modify: bool,
    pub brush_size: i32
}

impl ChunkManager {
	pub fn new(gfx: &mut Graphics) -> Self {
		Self {
			grid: Grid::new(50., 50., gfx),
	        selected_element: sand_element(),
	        modify: true,
			brush_size: 32,
		}
	}

	pub fn update(&mut self, app: &mut App, camera: &Camera2D) {
	    let mouse_world = get_mouse_in_world(&(app.mouse.x, app.mouse.y), (app.window().width(), app.window().height()), &camera);
	    let mouse = self.grid.mouse_in_sim(mouse_world);

	    match mouse {
	        Some(mouse) => {
	            if app.mouse.left_is_down() && self.modify {
	                self.grid.modify_elements(mouse.0, mouse.1, self.brush_size, &self.selected_element);
	            }

	            if app.mouse.right_is_down() && self.modify {
	                self.grid.explode(mouse.0, mouse.1, self.brush_size * 2, 4.);
	            }
	        },
	        _ => ()
	    }
		
		self.grid.update();
	}

	pub fn render(&mut self, gfx: &mut Graphics, draw: &mut Draw) {
		self.grid.render(gfx, draw);
	}
}
