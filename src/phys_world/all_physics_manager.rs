use notan::{prelude::{Graphics, App}, draw::Draw, math::{Mat3, Vec2, Vec3}};

use crate::{camera::Camera2D, debug_ui::DebugInfo, input_manager::get_mouse_in_world};

use super::{chunk_manager::ChunkManager, rapier_world_handler::{RapierHandler, PHYS_SCALE, SelectBody}, chunk::{UPSCALE_FACTOR, ROWS, COLS, self}, element::{air_element, Cell}, rigid_sand_body::ElInWorldInfo};

const PHYSICS_UPDATE_DELTA: f64 = 0.016; // INFO: The physics sims update at 60 FPS

pub struct PhysicsManager {
    pub chunk_manager: ChunkManager,
	pub rapier_handler: RapierHandler,
    pub update_time: f64,
	pub pause_all_phys: bool
}

impl PhysicsManager {
	pub fn new(gfx: &mut Graphics) -> Self {
		Self {
			chunk_manager: ChunkManager::new(gfx),
			rapier_handler: RapierHandler::new(),
            update_time: 0.,
			pause_all_phys: false
		}
	}

	pub fn update(&mut self, app: &mut App, camera: &Camera2D) {
		self.chunk_manager.update(app, camera);
		self.rapier_handler.update(app, camera);

		// INFO: Update the physics simulations at 60 FPS
        self.update_time += app.timer.delta_f32() as f64;
        if self.update_time >= PHYSICS_UPDATE_DELTA {
            self.update_time = 0.;

			if self.pause_all_phys {
				return;
			}

			self.rsbodies_to_chunks();
			self.chunk_manager.update_chunks_fixed();
			self.retrieve_els_to_rsbodies();
			
			self.rapier_handler.create_chunk_colliders(&mut self.chunk_manager.chunks);
			self.rapier_handler.update_fixed();
		}
	}

	pub fn render(&mut self, app: &mut App, gfx: &mut Graphics, render_draw: &mut Draw, debug_info: &DebugInfo, camera: &Camera2D) {
	    self.chunk_manager.render(gfx, render_draw);
		
		for rsbody_index in 0..self.rapier_handler.rigid_sand_bodies.len() {
			let rsbody = &mut self.rapier_handler.rigid_sand_bodies[rsbody_index];
			for el_info in &rsbody.body_elements_in_chunks {
				if let Some(chunk) = self.chunk_manager.chunks.get_mut(&el_info.chunk) {
					chunk::update_byte(&mut chunk.bytes, el_info.index_chunk.0, el_info.index_chunk.1, &chunk.grid[el_info.index_chunk.0][el_info.index_chunk.1].color);
				}
			}
		}

		for rsbody_index in 0..self.rapier_handler.rigid_sand_bodies.len() {
			let rs_body = &mut self.rapier_handler.rigid_sand_bodies[rsbody_index];

			rs_body.update_texture(gfx);
		}
		
	    self.chunk_manager.debug_render(render_draw, debug_info);
		self.rapier_handler.debug_render(render_draw, debug_info);

		// TODO:  This is not a good place for this
        let mouse_world = get_mouse_in_world(
            &(app.mouse.x, app.mouse.y),
            (app.window().width() as i32, app.window().height() as i32),
            camera,
        );
		if app.mouse.middle_was_released() {
			match self.rapier_handler.select_body {
			    SelectBody::Ball => self.rapier_handler.add_ball(mouse_world),
			    SelectBody::SandBody => self.rapier_handler.add_sand_body(mouse_world, gfx, &self.chunk_manager.tex_handler),
			}
		}
	}

	fn rsbodies_to_chunks(&mut self) {
		for rsbody_index in 0..self.rapier_handler.rigid_sand_bodies.len() {
			let rsbody = &mut self.rapier_handler.rigid_sand_bodies[rsbody_index];
			rsbody.body_elements_in_chunks.clear();

			let body_angle = self.rapier_handler.rigid_body_set[rsbody.rigid_body_handle].rotation().angle();
			let (body_els_rotated, off_x, off_y) = rotate_arbitrary(&rsbody.body_elements, -body_angle);

			let body_pos = &self.rapier_handler.rigid_body_set[rsbody.rigid_body_handle].translation();
			let body_world = Vec2::new(body_pos.x, body_pos.y) * PHYS_SCALE;

			let body_world_x = body_world.x.round() as i32 as f32 - off_x as f32;
			let body_world_y = body_world.y.round() as i32 as f32 - off_y as f32;

			let translation = Mat3::from_translation(Vec2::new(body_world_x, body_world_y));
			let rotation = Mat3::from_angle(body_angle);
			let matrix = translation * rotation;

			// TODO: rotations are probably not quite fixed yet, also use variables instead of set values
			let body_world = 
				matrix * 
				Vec3::new(rsbody.body_elements.len() as f32 * UPSCALE_FACTOR / 2., rsbody.body_elements[0].len() as f32 * UPSCALE_FACTOR / 2., 1.);

			
			for i in 0..body_els_rotated.len() {
				for j in 0..body_els_rotated[0].len() {

					// TODO: Elements are still incorrectly placed

					let el_world = (
						(body_world.x + (i as f32 * UPSCALE_FACTOR)) as i32,
						(body_world.y + (j as f32 * UPSCALE_FACTOR)) as i32
					);

                    let el_chunk_x = 
                        (el_world.0 / (COLS as f32 * UPSCALE_FACTOR) as i32) as i32 - if el_world.0 < 0 { 1 } else { 0 };
                    let el_chunk_y = 
                        (el_world.1 / (ROWS as f32 * UPSCALE_FACTOR) as i32) as i32 - if el_world.1 < 0 { 1 } else { 0 };

                    let mut cell_index_x =
                        ((el_world.0 as f32 + el_chunk_x as f32 * UPSCALE_FACTOR) / UPSCALE_FACTOR).round() as i32 % COLS as i32 - el_chunk_x;
                    if cell_index_x < 0 {
                        cell_index_x = COLS as i32 + cell_index_x - 1;
                    }
                    let mut cell_index_y =
                        ((el_world.1 as f32 + el_chunk_y as f32 * UPSCALE_FACTOR) / UPSCALE_FACTOR).round() as i32 % ROWS as i32 - el_chunk_y;
                    if cell_index_y < 0 {
                        cell_index_y = ROWS as i32 + cell_index_y - 1;
                    }

					if let Some(chunk) = self.chunk_manager.chunks.get_mut(&(el_chunk_x, el_chunk_y)) {

						// chunk::update_byte(&mut chunk.bytes, cell_index_x as usize, cell_index_y as usize, &[255, 255, 255, 255]);
						
						if let Some(element) = body_els_rotated[i][j] {
							chunk.grid[cell_index_x as usize][cell_index_y as usize] = element.0;
							chunk::update_byte(&mut chunk.bytes, cell_index_x as usize, cell_index_y as usize, &chunk.grid[cell_index_x as usize][cell_index_y as usize].color);

							if !chunk.active {
								chunk::activate(chunk);
							}
							chunk.dirty_rect.set_temp(cell_index_x as usize, cell_index_y as usize);
							
							rsbody.body_elements_in_chunks.push(
								ElInWorldInfo {
									chunk: (el_chunk_x, el_chunk_y),
									index_chunk: (cell_index_x as usize, cell_index_y as usize),
									index_body: (element.1, element.2)
								}
							);
						}
					}
				}
			}
		}
	}

	fn retrieve_els_to_rsbodies(&mut self) {
		for rsbody_index in 0..self.rapier_handler.rigid_sand_bodies.len() {
			let rsbody = &mut self.rapier_handler.rigid_sand_bodies[rsbody_index];

			for el_info in &rsbody.body_elements_in_chunks {
				if let Some(chunk) = self.chunk_manager.chunks.get_mut(&el_info.chunk) {

					// TODO: This will work when elements are properly placed
					let _retrieved_element = chunk.grid[el_info.index_chunk.0][el_info.index_chunk.1];
					// rsbody.body_elements[el_info.index_body.0][el_info.index_body.1] = Some(retrieved_element);

					chunk.grid[el_info.index_chunk.0][el_info.index_chunk.1] = air_element();
				}
			}
		}
	}
}

fn rotate_arbitrary(body_elements: &Vec<Vec<Option<Cell>>>, angle_radians: f32) -> (Vec<Vec<Option<(Cell, usize, usize)>>>, i32, i32) {
    let height = body_elements.len() as f32;
    let width = body_elements[0].len() as f32;
    let cos_angle = angle_radians.cos();
    let sin_angle = angle_radians.sin();

    // Determine new image dimensions
    let new_width = ((width * cos_angle.abs()) + (height * sin_angle.abs())).ceil() as usize;
    let new_height = ((width * sin_angle.abs()) + (height * cos_angle.abs())).ceil() as usize;

    let mut rotated = vec![vec![None; new_width]; new_height];

    let x_center = width / 2.0;
    let y_center = height / 2.0;
    let new_x_center = new_width as f32 / 2.0;
    let new_y_center = new_height as f32 / 2.0;

    for y in 0..new_height {
        for x in 0..new_width {
            let new_x = x as f32 - new_x_center;
            let new_y = y as f32 - new_y_center;

            let old_x = cos_angle * new_x + sin_angle * new_y + x_center;
            let old_y = -sin_angle * new_x + cos_angle * new_y + y_center;

            if old_x >= 0.0 && old_x < width && old_y >= 0.0 && old_y < height {
                let old_x_int = old_x.floor() as usize;
                let old_y_int = old_y.floor() as usize;

				if let Some(element) = body_elements[old_y_int][old_x_int] {
	                rotated[y][x] = Some((element, old_y_int, old_x_int));
				}
            }
        }
    }
	
    (rotated, new_width as i32, new_height as i32)
}
