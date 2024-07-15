use ahash::RandomState;
use std::{
    collections::HashMap,
    thread,
    time::{Duration, Instant},
};

use notan::{
    draw::*,
    prelude::*,
    random::rand::{prelude::SliceRandom, thread_rng},
};

use crate::{
    camera::Camera2D,
    input_manager::get_mouse_in_world,
    phys_world::chunk::{self, Chunk, COLS, ROWS, UPSCALE_FACTOR},
    phys_world::element::{sand_element, Cell},
    DebugInfo,
};

use super::element_texture_handler::ElementTexHandler;

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
    pub hovering_cell: (Cell, (i32, i32), (i32, i32)),
    pub update_time: f32,
    pub replace_air: bool,
    pub chunks_update_time: Duration,
    pub chunks_render_time: Duration,
    pub num_of_threads: [usize; 4],
    font: Font,
    pub chunk_frame_count: u128,
    pub tex_handler: ElementTexHandler
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
            hovering_cell: (sand_element(), (0, 0), (0, 0)),
            update_time: 0.,
            replace_air: true,
            chunks_update_time: Duration::default(),
            chunks_render_time: Duration::default(),
            num_of_threads: [0; 4],
            font: gfx
                .create_font(include_bytes!("../assets/UbuntuMono.ttf"))
                .unwrap(),
            chunk_frame_count: 0,
            tex_handler: ElementTexHandler::new()
        }
    }

    pub fn update(&mut self, app: &mut App, camera: &Camera2D) {
        let mouse_world = get_mouse_in_world(
            &(app.mouse.x, app.mouse.y),
            (app.window().width() as i32, app.window().height() as i32),
            camera,
        );

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
                    let mouse = chunk::mouse_in_chunk(chunk.pos, mouse_world);

                    if app.mouse.left_is_down() && self.modify {
                        chunk::modify_chunk_elements(
                            chunk,
                            mouse.0,
                            mouse.1,
                            self.brush_size,
                            &self.selected_element,
                            self.replace_air,
                            &self.tex_handler
                        );
                    }

                    if app.mouse.right_is_down() && self.modify {
                        chunk::explode_chunk(
                            chunk,
                            mouse.0,
                            mouse.1,
                            self.brush_size * 2,
                            4. * app.timer.delta_f32() * 90.,
                        );
                    }

                    if let Some(c) = chunk::get_chunk_cell(chunk, mouse.0, mouse.1) {
                        self.hovering_cell.0 = c.to_owned();
                        self.hovering_cell.1 = chunk.index;
                        self.hovering_cell.2 = mouse;
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
        if chunks_to_update.len() > 1 {
            // INFO: Create threads only if there are multiple chunks to update
            let mut thread_handles = vec![];

            // Separate chunks into pools for optimized use of threads
            // INFO: Here the chunks are sorted in increasing size by area
            let mut ordered_chunks_to_update = vec![];
            for chunk_index in chunks_to_update {
                let mut inserted = false;
                for i in 0..(ordered_chunks_to_update.len()) {
                    if self.chunks.get(chunk_index).unwrap().dirty_rect.get_area()
                        < self
                            .chunks
                            .get(&ordered_chunks_to_update[i])
                            .unwrap()
                            .dirty_rect
                            .get_area()
                    {
                        ordered_chunks_to_update.insert(i, chunk_index.clone());
                        inserted = true;
                        break;
                    }
                }
                if !inserted {
                    ordered_chunks_to_update
                        .insert(ordered_chunks_to_update.len(), chunk_index.clone());
                }
            }

            // INFO: Here the chunks are separated into threadable pools
            let mut chunk_pools_to_update = vec![];
            let mut chunk_pool = vec![];
            let mut sum = 0;
            while !ordered_chunks_to_update.is_empty() {
                if sum
                    + self
                        .chunks
                        .get(&ordered_chunks_to_update[0])
                        .unwrap()
                        .dirty_rect
                        .get_area()
                    > (COLS * ROWS) as u32
                {
                    chunk_pools_to_update.push(chunk_pool.clone());
                    chunk_pool.clear();
                    sum = 0;
                }
                sum += self
                    .chunks
                    .get(&ordered_chunks_to_update[0])
                    .unwrap()
                    .dirty_rect
                    .get_area();
                chunk_pool.push(ordered_chunks_to_update.remove(0));
            }
            if !chunk_pool.is_empty() {
                chunk_pools_to_update.push(chunk_pool);
            }

            // Update the chunks
            if chunk_pools_to_update.len() > 1 {
                // INFO: Only create threads if there is more than one pool to update
                for chunk_pool_indices in chunk_pools_to_update {
                    let mut chunk_pool = vec![];
                    for chunk_index in &chunk_pool_indices {
                        chunk_pool.push(self.chunks.remove(chunk_index).unwrap());
                    }

                    let ptr = RawPtrHolder {
                        ptr: &mut self.chunks as *mut WorldChunks,
                    };

                    let frame_count = self.chunk_frame_count;
                    let handle = thread::spawn(move || {
                        let world_chunks_ptr = ptr;
                        unsafe {
                            for i in 0..chunk_pool.len() {
                                chunk::update_chunk(&mut chunk_pool[i], &mut *world_chunks_ptr.ptr, frame_count);
                            }
                        }
                        chunk_pool
                    });

                    thread_handles.push(handle);
                }

                self.num_of_threads[index] = thread_handles.len();

                for handle in thread_handles {
                    let mut chunk_pool = handle.join().unwrap();
                    while !chunk_pool.is_empty() {
                        let chunk = chunk_pool.remove(0);
                        self.chunks.insert(chunk.index, chunk);
                    }
                }
            } else {
                // INFO: Update single pool
                for i in 0..chunk_pools_to_update[0].len() {
                    let mut chunk = self.chunks.remove(&chunk_pools_to_update[0][i]).unwrap();
                    chunk::update_chunk(&mut chunk, &mut self.chunks, self.chunk_frame_count);
                    self.chunks.insert(chunk.index, chunk);
                }
            }
        } else {
            // INFO: Update single chunk
            let mut chunk = self.chunks.remove(&chunks_to_update[0]).unwrap();
            chunk::update_chunk(&mut chunk, &mut self.chunks, self.chunk_frame_count);
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
                draw.text(&self.font, &format!("{}, {}", index.0, index.1))
                    .position(
                        (COLS as f32 / 2. + (index.0 as f32 * COLS as f32)) * UPSCALE_FACTOR,
                        (ROWS as f32 / 2. + (index.1 as f32 * ROWS as f32)) * UPSCALE_FACTOR,
                    )
                    .h_align_center()
                    .v_align_middle();
            }
        }

        if debug_info.debug_chunk_bounds {
            for index in self.chunks.keys() {
                draw.rect(
                    (
                        index.0 as f32 * COLS as f32 * UPSCALE_FACTOR + 0.5,
                        index.1 as f32 * ROWS as f32 * UPSCALE_FACTOR + 0.5,
                    ),
                    (
                        COLS as f32 * UPSCALE_FACTOR - 1.,
                        ROWS as f32 * UPSCALE_FACTOR - 1.,
                    ),
                )
                .fill_color(Color::from_rgba(0., 0., 0., 0.))
                .stroke_color(if self.chunks.get(index).unwrap().active {
                    Color::GREEN
                } else {
                    Color::RED
                })
                .stroke(1.);
            }
        }

        if debug_info.debug_dirty_rects {
            for (index, chunk) in self.chunks.iter() {
                if chunk.active {
                    draw.rect(
                        (
                            (index.0 as f32 * COLS as f32 + chunk.dirty_rect.min_xy.0 as f32)
                                * UPSCALE_FACTOR,
                            (index.1 as f32 * ROWS as f32 + chunk.dirty_rect.min_xy.1 as f32)
                                * UPSCALE_FACTOR,
                        ),
                        (
                            (chunk.dirty_rect.max_xy.0 - chunk.dirty_rect.min_xy.0) as f32
                                * UPSCALE_FACTOR,
                            (chunk.dirty_rect.max_xy.1 - chunk.dirty_rect.min_xy.1) as f32
                                * UPSCALE_FACTOR,
                        ),
                    )
                    .fill_color(Color::from_rgba(0., 0., 0., 0.))
                    .stroke_color(Color::BLUE)
                    .stroke(1.);
                }
            }
        }
    }
}

struct RawPtrHolder {
    ptr: *mut WorldChunks,
}

unsafe impl Send for RawPtrHolder {}
unsafe impl Sync for RawPtrHolder {}
