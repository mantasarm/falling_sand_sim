use notan::{prelude::{Graphics, App}, draw::Draw};

use crate::{camera::Camera2D, debug_ui::DebugInfo, input_manager::get_mouse_in_world};

use super::{chunk_manager::ChunkManager, rapier_world_handler::RapierHandler};

const PHYSICS_UPDATE_DELTA: f32 = 0.016; // INFO: The chunks update at 60 FPS

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

        let mouse_world = get_mouse_in_world(
            &(app.mouse.x, app.mouse.y),
            (app.window().width() as i32, app.window().height() as i32),
            camera,
        );
		if app.mouse.middle_was_released() {
			self.rapier_handler.add_ball(mouse_world);
		}
		
		self.rapier_handler.update(app);

		if self.pause_all_phys {
			return;
		}
		
        self.update_time += app.timer.delta_f32();
        if self.update_time >= PHYSICS_UPDATE_DELTA {
            self.update_time = 0.;

			self.chunk_manager.update_chunks_fixed();
			
			self.rapier_handler.create_chunk_colliders(&mut self.chunk_manager.chunks);
			self.rapier_handler.update_fixed();
		}
	}

	pub fn render(&mut self, gfx: &mut Graphics, render_draw: &mut Draw, debug_info: &DebugInfo) {
	    self.chunk_manager.render(gfx, render_draw);
	    self.chunk_manager.debug_render(render_draw, debug_info);

		self.rapier_handler.render(render_draw);
	}
}
