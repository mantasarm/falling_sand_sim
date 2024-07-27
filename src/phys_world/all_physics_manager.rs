use notan::{prelude::{Graphics, App, Color}, draw::{Draw, DrawImages, CreateDraw}, math::{Mat4, Mat3, Vec2}};

use crate::{camera::Camera2D, debug_ui::DebugInfo, input_manager::get_mouse_in_world};

use super::{chunk_manager::ChunkManager, rapier_world_handler::{RapierHandler, PHYS_SCALE, SelectBody}, chunk::{UPSCALE_FACTOR, ROWS, COLS}};

const PHYSICS_UPDATE_DELTA: f32 = 0.016; // INFO: The physics sims update at 60 FPS

pub struct PhysicsManager {
    pub chunk_manager: ChunkManager,
	pub rapier_handler: RapierHandler,
    pub update_time: f32,
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

		if self.pause_all_phys {
			return;
		}
		
		// INFO: Update the physics simulations at 60 FPS
        self.update_time += app.timer.delta_f32();
        if self.update_time >= PHYSICS_UPDATE_DELTA {
            self.update_time = 0.;

			self.chunk_manager.update_chunks_fixed();
			
			self.rapier_handler.create_chunk_colliders(&mut self.chunk_manager.chunks);
			self.rapier_handler.update_fixed();
		}
	}

	pub fn render(&mut self, app: &mut App, gfx: &mut Graphics, render_draw: &mut Draw, debug_info: &DebugInfo, camera: &Camera2D) {
	    self.chunk_manager.render(gfx, render_draw);
	    self.chunk_manager.debug_render(render_draw, debug_info);

		// TODO: optimize, bodies are rendered in chunks that they don't even appear in, and move into seperate function
        for i in self.chunk_manager.range_x.0..=self.chunk_manager.range_x.1 {
            for j in self.chunk_manager.range_y.0..=self.chunk_manager.range_y.1 {
				if let Some(chunk) = self.chunk_manager.chunks.get(&(i, j)) {
					let mut rt_draw = gfx.create_draw();
					rt_draw.clear(Color::TRANSPARENT);
					let projection = Mat4::orthographic_rh_gl(0., COLS as f32, ROWS as f32, 0., -1., 1.);
					rt_draw.set_projection(Some(projection));

					for rs_body in &self.rapier_handler.rigid_sand_bodies {
						let body = &self.rapier_handler.rigid_body_set[rs_body.rigid_body_handle];
						let pos = ((body.translation().x * PHYS_SCALE - chunk.pos.0) / UPSCALE_FACTOR, (body.translation().y * PHYS_SCALE  - chunk.pos.1) / UPSCALE_FACTOR);

						let translation = Mat3::from_translation(Vec2::new(pos.0, pos.1));
						let rotation = Mat3::from_angle(body.rotation().angle());
						let matrix = translation * rotation;

						rt_draw.transform().push(matrix);
						
						rt_draw.image(&rs_body.texture);

						rt_draw.transform().pop();
					}

					gfx.render_to(&chunk.render_texture, &rt_draw);
					
					render_draw.image(&chunk.render_texture.texture())
						.size(COLS as f32 * UPSCALE_FACTOR, ROWS as f32 * UPSCALE_FACTOR)
						.position(chunk.pos.0, chunk.pos.1);
				}
            }
        }
		//
		
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
}
