use ahash::HashMap;
use notan::{draw::{Draw, DrawShapes, DrawTransform}, prelude::{App, KeyCode}};
use rapier2d::prelude::*;

use crate::phys_world::chunk::{COLS, ROWS};

use super::{chunk_manager::WorldChunks, chunk::UPSCALE_FACTOR};

pub const PHYS_SCALE: f32 = 50.0;
pub const GRAVITY: f32 = 9.81;

pub struct RapierHandler {
	pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    gravity: Vector<f32>,
    integration_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: DefaultBroadPhase,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    physics_hooks: (),
    event_handler: (),
	chunk_colliders: HashMap<(i32, i32), Vec<ColliderHandle>>,
	pub update_phys: bool,
	
	ball_body_handles: Vec<RigidBodyHandle>,
}

impl RapierHandler {
	pub fn new() -> Self {
		let rigid_body_set = RigidBodySet::new();
	    let collider_set = ColliderSet::new();

		let ball_body_handles = vec![];

		let gravity = vector![0.0, GRAVITY];
	    let integration_parameters = IntegrationParameters::default();
	    let physics_pipeline = PhysicsPipeline::new();
	    let island_manager = IslandManager::new();
	    let broad_phase = DefaultBroadPhase::new();
	    let narrow_phase = NarrowPhase::new();
	    let impulse_joint_set = ImpulseJointSet::new();
	    let multibody_joint_set = MultibodyJointSet::new();
	    let ccd_solver = CCDSolver::new();
	    let physics_hooks = ();
	    let event_handler = ();

	    Self {
	        rigid_body_set,
	        collider_set,
	        gravity,
	        integration_parameters,
	        physics_pipeline,
	        island_manager,
	        broad_phase,
	        narrow_phase,
	        impulse_joint_set,
	        multibody_joint_set,
	        ccd_solver,
	        physics_hooks,
	        event_handler,
			chunk_colliders: HashMap::default(),
			update_phys: true,
			
			ball_body_handles,
	    }
	}

	pub fn update(&mut self, app: &App) {
		if app.keyboard.was_pressed(KeyCode::Right) {
	        self.rigid_body_set[self.ball_body_handles[0]].apply_torque_impulse(2., true);
	    }
	    if app.keyboard.was_pressed(KeyCode::Left) {
	        self.rigid_body_set[self.ball_body_handles[0]].apply_torque_impulse(-2., true);
	    }
	}

	pub fn update_fixed(&mut self) {
		if !self.update_phys {
			return;
		}
		
		self.physics_pipeline.step(
			&self.gravity,
			&self.integration_parameters,
			&mut self.island_manager, 
			&mut self.broad_phase,
			&mut self.narrow_phase, 
			&mut self.rigid_body_set,
			&mut self.collider_set,
			&mut self.impulse_joint_set,
			&mut self.multibody_joint_set,
			&mut self.ccd_solver,
			None,
			&self.physics_hooks,
			&self.event_handler
		);
	}

	pub fn add_ball(&mut self, mouse: (f32, f32)) {
		let rigid_body = RigidBodyBuilder::dynamic().translation(vector![mouse.0 / PHYS_SCALE, mouse.1 / PHYS_SCALE]).build();
		let collider = ColliderBuilder::ball(8. / PHYS_SCALE).restitution(0.7).build();
		self.ball_body_handles.push(self.rigid_body_set.insert(rigid_body));
		self.collider_set.insert_with_parent(collider, self.ball_body_handles[self.ball_body_handles.len() - 1], &mut self.rigid_body_set);
	}

	pub fn remove_balls(&mut self) {
		for i in 0..self.ball_body_handles.len() {
			self.rigid_body_set.remove(self.ball_body_handles[i], &mut self.island_manager, &mut self.collider_set, &mut self.impulse_joint_set, &mut self.multibody_joint_set, true);
		}
		self.ball_body_handles.clear();
	}

	pub fn create_chunk_colliders(&mut self, chunks: &mut WorldChunks) {
		for (index, chunk) in chunks.iter_mut() {
			if chunk.colliders_dirty {
			    // edges_from_chunk(chunk);
				
				if let Some(collider_handles) = self.chunk_colliders.get(index) {
					for i in 0..collider_handles.len() {
						self.collider_set.remove(collider_handles[i], &mut self.island_manager, &mut self.rigid_body_set, false);
					}
				}

				let mut collider_handles = vec![];
				for edge in &chunk.edges {
					collider_handles.push(
						self.collider_set.insert(
							ColliderBuilder::polyline(edge.to_owned(), None)
							.translation(vector![index.0 as f32 * COLS as f32 / (PHYS_SCALE / UPSCALE_FACTOR), index.1 as f32 * ROWS as f32 / (PHYS_SCALE / UPSCALE_FACTOR)])
							.build()
						)
					);
				}

				self.chunk_colliders.insert(index.to_owned(), collider_handles);
				chunk.colliders_dirty = false;
			}
		}
	}

	pub fn render(&self, render_draw: &mut Draw) {
		for i in 0..self.ball_body_handles.len() {
			let ball_body = &self.rigid_body_set[self.ball_body_handles[i]];
			let pos = ball_body.translation();
			render_draw.circle(8.).translate(pos.x * PHYS_SCALE, pos.y * PHYS_SCALE);
		}
	}
}
