use std::{collections::HashMap, time::{Instant, Duration}, thread};
use ahash::RandomState;

use notan::{prelude::*, draw::*, random::rand::{prelude::SliceRandom, thread_rng}};

use crate::{camera::Camera2D, input_manager::get_mouse_in_world, phys_world::element::{Cell, sand_element}, phys_world::chunk::{Chunk, self, COLS, UPSCALE_FACTOR, ROWS}, DebugInfo};

const CHUNK_UPDATE_DELTA: f32 = 0.016; // INFO: The chunks update at 60 FPS

pub type WorldChunks = HashMap<(i32, i32), Chunk, RandomState>;

pub struct ChunkManager {
	chunks: WorldChunks,
    pub selected_element: Cell,
    pub modify: bool,
    pub brush_size: i32,
	pub update_chunks: bool,
	pub range_x: (i32, i32),
	pub range_y: (i32, i32),
	pub hovering_cell: Cell,
	pub update_time: f32,
	pub replace_air: bool,
	pub chunks_update_time: Duration,
	pub chunks_render_time: Duration,
	pub	num_of_threads: [usize; 4],
	font: Font,
	pub chunk_frame_count: u128
}

impl ChunkManager {
	pub fn new(gfx: &mut Graphics) -> Self {
		let range_x = (-2, 2);
		let range_y = (-2, 2);

		let mut chunks: WorldChunks = HashMap::default();
		for i in range_x.0..=range_x.1 {
			for j in range_y.0..=range_y.1 {
				chunks.insert((i, j), Chunk::new(i, j, gfx));
			}
		}
		
		Self {
			chunks,
	        selected_element: sand_element(),
	        modify: true,
			brush_size: 32,
			update_chunks: true,
			range_x,
			range_y,
			hovering_cell: sand_element(),
			update_time: 0.,
			replace_air: true,
			chunks_update_time: Duration::default(),
			chunks_render_time: Duration::default(),
			num_of_threads: [0; 4],
			font: gfx.create_font(include_bytes!("../assets/UbuntuMono.ttf")).unwrap(),
			chunk_frame_count: 0
		}
	}

	pub fn update(&mut self, app: &mut App, camera: &Camera2D) {
	    let mouse_world = get_mouse_in_world(&(app.mouse.x, app.mouse.y), (app.window().width() as i32, app.window().height() as i32), camera);

		if app.mouse.is_scrolling() {
			self.brush_size += app.mouse.wheel_delta.y as i32 / 6;

			if self.brush_size <= 0 {
				self.brush_size = 1;
			}
		}

		for i in self.range_x.0..=self.range_x.1 {
			for j in (self.range_y.0..=self.range_y.1).rev() {
				let key = &(i, j);

				if let Some(chunk) = self.chunks.get_mut(key) {
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
			}
		}

		self.update_chunks(app);
	}

	fn update_chunks(&mut self, app: &mut App) {
		if self.update_chunks {
			self.update_time += app.timer.delta_f32();

			if self.update_time >= CHUNK_UPDATE_DELTA {
				let now = Instant::now();
				
				self.update_time = 0.;
				self.chunk_frame_count += 1;

				let mut all_chunks_to_update = vec![];

				// INFO: Separate chunks into updatable chunk pools
				let mut chunks_to_update = (vec![], vec![], vec![], vec![]);
				for j in self.range_y.0..=self.range_y.1 {
					if j % 2 == 0 {
						for i in self.range_x.0..=self.range_x.1 {
							if i % 2 == 0 {
								if let Some(chunk) = self.chunks.get(&(i, j)) {
							        if chunk.active {
								        chunks_to_update.0.push((i, j));
							        }
						        }
							} else {
								if let Some(chunk) = self.chunks.get(&(i, j)) {
							        if chunk.active {
								        chunks_to_update.1.push((i, j));
							        }
						        }
							}
						}
					} else {
						for i in self.range_x.0..=self.range_x.1 {
							if i % 2 == 0 {
								if let Some(chunk) = self.chunks.get(&(i, j)) {
							        if chunk.active {
								        chunks_to_update.2.push((i, j));
							        }
						        }
							} else {
								if let Some(chunk) = self.chunks.get(&(i, j)) {
							        if chunk.active {
								        chunks_to_update.3.push((i, j));
							        }
						        }
							}
						}
					}
				}
			
				if !chunks_to_update.0.is_empty() {
					all_chunks_to_update.push(chunks_to_update.0);
				}
				if !chunks_to_update.1.is_empty() {
					all_chunks_to_update.push(chunks_to_update.1);
				}
				if !chunks_to_update.2.is_empty() {
					all_chunks_to_update.push(chunks_to_update.2);
				}
				if !chunks_to_update.3.is_empty() {
					all_chunks_to_update.push(chunks_to_update.3);
				}

				if !all_chunks_to_update.is_empty() {
					self.num_of_threads = [0; 4];
					let mut order: Vec<usize> = (0..all_chunks_to_update.len()).collect();
					order.shuffle(&mut thread_rng());

					for i in order {
						self.update_select_chunks(&all_chunks_to_update[i], i);
					}
				
				} else if all_chunks_to_update.is_empty() {
					self.num_of_threads = [0; 4];
				}

				self.chunks_update_time = now.elapsed();
			}
		}
	}

	fn update_select_chunks(&mut self, chunks_to_update: &Vec<(i32, i32)>, index: usize) {
		self.num_of_threads[index] = 0;
		if chunks_to_update.len() > 1 { // INFO: Create threads only if there are multiple chunks to update
			let mut thread_handles = vec![];
			for chunk_index in chunks_to_update {
				let mut chunk = self.chunks.remove(chunk_index).unwrap();

				let ptr = RawPtrHolder {
					ptr: &mut self.chunks as *mut WorldChunks
				};

				let handle = thread::spawn(move || {
					let world_chunks_ptr = ptr;
					unsafe {
						chunk::update_chunk(&mut chunk, &mut *world_chunks_ptr.ptr);
					}
					chunk
				});

				thread_handles.push(handle);
			}

			self.num_of_threads[index] = thread_handles.len();

			for handle in thread_handles {
				let chunk = handle.join().unwrap();
				self.chunks.insert(chunk.index, chunk);
			}
		} else {
			let mut chunk = self.chunks.remove(&chunks_to_update[0]).unwrap();
			chunk::update_chunk(&mut chunk, &mut self.chunks);
			self.chunks.insert(chunks_to_update[0], chunk);
		}
	}

	pub fn render(&mut self, gfx: &mut Graphics, draw: &mut Draw) {
		let now = Instant::now();
		for chunk in self.chunks.values_mut() {
			chunk::render_chunk(chunk, gfx, draw, self.update_chunks);
		}
		self.chunks_render_time = now.elapsed();
	}

	pub fn debug_render(&mut self, draw: &mut Draw, debug_info: &DebugInfo) {
		if debug_info.debug_chunk_coords {
			for index in self.chunks.keys() {
				draw.text(&self.font, &format!("{:?}", index))
					.position((COLS as f32 / 2. + (index.0 as f32 * COLS as f32)) * UPSCALE_FACTOR, (ROWS as f32 / 2. + (index.1 as f32 * ROWS as f32)) * UPSCALE_FACTOR)
					.h_align_center()
					.v_align_middle();
			}
		}

		if debug_info.debug_chunk_bounds {
			for index in self.chunks.keys() {
				draw.rect((index.0 as f32 * COLS as f32 * UPSCALE_FACTOR + 0.5, index.1 as f32 * ROWS as f32 * UPSCALE_FACTOR + 0.5), (COLS as f32 * UPSCALE_FACTOR - 1., ROWS as f32 * UPSCALE_FACTOR - 1.))
					.fill_color(Color::from_rgba(0., 0., 0., 0.))
					.stroke_color(if self.chunks.get(index).unwrap().active { Color::GREEN } else { Color::RED })
					.stroke(1.);
			}
		}

		if debug_info.debug_dirty_rects {
			for (index, chunk) in self.chunks.iter() {
				if chunk.active {
					draw.rect(((index.0 as f32 * COLS as f32 + chunk.dirty_rect.min_xy.0 as f32) * UPSCALE_FACTOR,
										(index.1 as f32 * ROWS as f32 + chunk.dirty_rect.min_xy.1 as f32) * UPSCALE_FACTOR),
					              ((chunk.dirty_rect.max_xy.0 - chunk.dirty_rect.min_xy.0) as f32 * UPSCALE_FACTOR,
								        (chunk.dirty_rect.max_xy.1 - chunk.dirty_rect.min_xy.1) as f32 * UPSCALE_FACTOR))
						.fill_color(Color::from_rgba(0., 0., 0., 0.))
						.stroke_color(Color::BLUE)
						.stroke(1.);
				}
			}
		}
	}
}

struct RawPtrHolder {
	ptr: *mut WorldChunks
}

unsafe impl Send for RawPtrHolder {}
unsafe impl Sync for RawPtrHolder {}
