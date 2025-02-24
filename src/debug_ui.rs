use std::time::Duration;

use memory_stats::memory_stats;
use notan::{
    app::App,
    egui::{epaint::Shadow, Color32, Context, RichText, Slider, Visuals, Window, ComboBox},
    input::keyboard::KeyCode,
};

use crate::{phys_world::chunk_manager::ChunkManager, phys_world::{element::*, rapier_world_handler::{RapierHandler, SelectBody}, all_physics_manager::PhysicsManager}};

pub struct DebugInfo {
    pub set_visuals: bool,
    pub editor_open: bool,
    pub debug_window: bool,
    pub debug_chunk_bounds: bool,
    pub debug_chunk_coords: bool,
    pub debug_dirty_rects: bool,
    pub debug_chunk_edges: bool,
    pub debug_rapier2d: bool,
    pub debug_rs_body_edges: bool,
    pub debug_metrics: bool,
    pub longest_update_time: Duration,
    pub debug_mem_usage: bool,
    pub sky_color: [u8; 3],
    pub sky_editor: bool,
}

impl Default for DebugInfo {
    fn default() -> Self {
        Self {
            set_visuals: false,
            editor_open: true,
            debug_window: true,
            debug_chunk_bounds: false,
            debug_chunk_coords: true,
            debug_dirty_rects: false,
            debug_chunk_edges: false,
            debug_rapier2d: true,
            debug_rs_body_edges: false,
            longest_update_time: Duration::ZERO,
            debug_metrics: false,
            debug_mem_usage: false,
            sky_color: [70, 35, 70],
            sky_editor: false,
        }
    }
}

pub fn debug_update(app: &App, debug_info: &mut DebugInfo, physics_manager: &mut PhysicsManager) {
    if app.keyboard.was_pressed(KeyCode::R) {
        debug_info.editor_open = !debug_info.editor_open;
    }
    if app.keyboard.was_pressed(KeyCode::Y) {
        debug_info.sky_editor = !debug_info.sky_editor;
    }
    if app.keyboard.was_pressed(KeyCode::Space) {
        physics_manager.pause_all_phys = !physics_manager.pause_all_phys;
    }
    if app.keyboard.was_pressed(KeyCode::F) {
        debug_info.debug_chunk_bounds = !debug_info.debug_chunk_bounds;
    }
    if app.keyboard.was_pressed(KeyCode::T) {
        debug_info.debug_window = !debug_info.debug_window;
    }
    if app.keyboard.was_pressed(KeyCode::M) {
        debug_info.debug_metrics = !debug_info.debug_metrics;
    }
    if app.keyboard.was_pressed(KeyCode::U) {
        debug_info.debug_rapier2d = !debug_info.debug_rapier2d;
    }
}

pub fn debug_ui(
    ctx: &Context,
    app: &App,
    debug_info: &mut DebugInfo,
    physics_manager: &mut PhysicsManager
) {
    if !debug_info.set_visuals {
        let mut visuals = Visuals::dark();
        visuals.window_shadow = Shadow::NONE;
        ctx.set_visuals(visuals);
        debug_info.set_visuals = true;
    }

    debug_update(app, debug_info, physics_manager);
    debug_editor(ctx, app, debug_info, physics_manager);
    debug_render(ctx, debug_info, &mut physics_manager.chunk_manager);
    debug_metrics(ctx, app, debug_info, &mut physics_manager.chunk_manager);
    debug_mem_usage(ctx, debug_info);
    debug_sky_editor(ctx, debug_info);
    debug_rapier_window(ctx, debug_info, &mut physics_manager.rapier_handler);
}

pub fn debug_editor(
    ctx: &Context,
    app: &App,
    debug_info: &mut DebugInfo,
    physics_manager: &mut PhysicsManager
) {
    Window::new("Editor")
        .resizable(false)
        .collapsible(true)
        .title_bar(true)
        .open(&mut debug_info.editor_open)
        .show(ctx, |ui| {
            if !debug_info.debug_metrics {
                ui.label(format!("fps: {}", app.timer.fps().round()));
            }

            ui.add_space(5.);

            ui.horizontal_wrapped(|ui| {
                if ui.button(RichText::new("Air").color(Color32::from_rgb(255, 255, 255))).clicked() {
                    physics_manager.chunk_manager.selected_element = air_element();
                }
                if ui.button(RichText::new("Solid").color(Color32::from_rgb(169, 162, 166))).clicked() {
                    physics_manager.chunk_manager.selected_element = solid_element();
                }
                if ui.button(RichText::new("Sand").color(Color32::from_rgb(243, 239, 118))).clicked() {
                    physics_manager.chunk_manager.selected_element = sand_element();
                }
                if ui.button(RichText::new("Dirt").color(Color32::from_rgb(136, 107, 82))).clicked() {
                    physics_manager.chunk_manager.selected_element = dirt_element();
                }
                if ui.button(RichText::new("Coal").color(Color32::from_rgb(130, 130, 130))).clicked() {
                    physics_manager.chunk_manager.selected_element = coal_element();
                }
                if ui.button(RichText::new("Wood").color(Color32::from_rgb(111, 83, 57))).clicked() {
                    physics_manager.chunk_manager.selected_element = wood_element();
                }
                if ui.button(RichText::new("SawDust").color(Color32::from_rgb(181, 137, 100))).clicked() {
                    physics_manager.chunk_manager.selected_element = sawdust_element();
                }
                if ui.button(RichText::new("Water").color(Color32::from_rgb(75, 66, 249))).clicked() {
                    physics_manager.chunk_manager.selected_element = water_element();
                }
                if ui.button(RichText::new("Smoke").color(Color32::from_rgb(142, 142, 142))).clicked() {
                    physics_manager.chunk_manager.selected_element = smoke_element();
                }
                if ui.button(RichText::new("Steam").color(Color32::from_rgb(143, 159, 234))).clicked() {
                    physics_manager.chunk_manager.selected_element = steam_element();
                }
                if ui.button(RichText::new("Petrol").color(Color32::from_rgb(0, 95, 106))).clicked() {
                    physics_manager.chunk_manager.selected_element = petrol_element();
                }
                if ui.button(RichText::new("Methane").color(Color32::from_rgb(130, 171, 41))).clicked() {
                    physics_manager.chunk_manager.selected_element = methane_element();
                }
                if ui.button(RichText::new("Fire").color(Color32::from_rgb(255, 0, 0))).clicked() {
                    let mut element = fire_element();
                    element.lifetime = 150;
                    physics_manager.chunk_manager.selected_element = element;
                }
                if ui.button(RichText::new("Lava").color(Color32::from_rgb(234, 46, 56))).clicked() {
                    physics_manager.chunk_manager.selected_element = lava_element();
                }
                if ui.button(RichText::new("Source").color(Color32::from_rgb(252, 186, 3))).clicked() {
                    physics_manager.chunk_manager.selected_element = source_element();
                }
                if ui.button(RichText::new("Gravel").color(Color32::from_rgb(83, 84, 78))).clicked() {
                    physics_manager.chunk_manager.selected_element = gravel_element();
                }
                if ui.button(RichText::new("Solid Dirt").color(Color32::from_rgb(136, 107, 82))).clicked() {
                    physics_manager.chunk_manager.selected_element = soliddirt_element();
                }
                if ui.button(RichText::new("Grass").color(Color32::from_rgb(19, 109, 21))).clicked() {
                    physics_manager.chunk_manager.selected_element = grass_element();
                }
                if ui.button(RichText::new("Brick").color(Color32::from_rgb(156, 89, 89))).clicked() {
                    physics_manager.chunk_manager.selected_element = brick_element();
                }
                if ui.button(RichText::new("Snow").color(Color32::from_rgb(200, 200, 200))).clicked() {
                    physics_manager.chunk_manager.selected_element = snow_element();
                }
                if ui.button(RichText::new("Ice").color(Color32::from_rgb(154, 176, 221))).clicked() {
                    physics_manager.chunk_manager.selected_element = ice_element();
                }
                if ui.button(RichText::new("Fireworks").color(Color32::from_rgb(243, 46, 161))).clicked() {
                    physics_manager.chunk_manager.selected_element = firework_shell_element();
                }
            });
            ui.add_space(5.);

            let brush_slider = Slider::new(&mut physics_manager.chunk_manager.brush_size, 1..=200).clamp_to_range(false);
            ui.add(brush_slider);
            ui.checkbox(&mut physics_manager.chunk_manager.replace_air, "Replace only air");
            ui.checkbox(&mut physics_manager.chunk_manager.edit_body_elements, "Editable body elements");
            ui.checkbox(&mut physics_manager.pause_all_phys, "Pause all");

            if ui.button("Next phys step").clicked() {
                if physics_manager.pause_all_phys {
                    physics_manager.next_step = true;
                }
            }

            ui.label("Press Y to modify sky color");
            ui.label("Press T for debug info");
            ui.label("Press M for metrics");
            ui.label("Press U for rapier2D info");
        });
}

pub fn debug_render(ctx: &Context, debug_info: &mut DebugInfo, chunk_manager: &mut ChunkManager) {
    Window::new("Debug window")
        .resizable(false)
        .collapsible(true)
        .title_bar(true)
        .open(&mut debug_info.debug_window)
        .show(ctx, |ui| {
            ui.checkbox(&mut chunk_manager.update_chunks, "Update falling sand sim");
            ui.add_space(5.);
            ui.checkbox(&mut debug_info.debug_chunk_bounds, "Chunk borders");
            ui.checkbox(&mut debug_info.debug_chunk_coords, "Chunk indices");
            ui.checkbox(&mut debug_info.debug_dirty_rects, "Dirty rects");

            ui.label(format!("Chunk hovered: {:?}", chunk_manager.hovering_cell.1));
            ui.label(format!("Index hovered: {:?}", chunk_manager.hovering_cell.2));

            ui.label(RichText::new("Mouse is on: ").color(Color32::from_rgb(180, 180, 180)));
            ui.label("Cell {");
            ui.label(format!(
                "    element: {:?} ",
                chunk_manager.hovering_cell.0.element
            ));
            ui.label(format!(
                "    action: {:?} ",
                chunk_manager.hovering_cell.0.action
            ));
            ui.label(format!(
                "    state: {:?}",
                chunk_manager.hovering_cell.0.state
            ));
            ui.label(format!(
                "    velocity: Vec2({:.2}, {:.2})",
                chunk_manager.hovering_cell.0.velocity.x, chunk_manager.hovering_cell.0.velocity.y
            ));
            ui.label(format!(
                "    density: {:?}",
                chunk_manager.hovering_cell.0.density
            ));
            ui.label(format!("    drag: {:?}", chunk_manager.hovering_cell.0.drag));
            ui.label(format!(
                "    color: {:?}",
                chunk_manager.hovering_cell.0.color
            ));
            ui.label(format!(
                "    lifetime: {:?}",
                chunk_manager.hovering_cell.0.lifetime
            ));
            ui.label("}");
        });
}

pub fn debug_metrics(
    ctx: &Context,
    app: &App,
    debug_info: &mut DebugInfo,
    chunk_manager: &ChunkManager,
) {
    Window::new("Metrics")
        .resizable(false)
        .collapsible(true)
        .open(&mut debug_info.debug_metrics)
        .show(ctx, |ui| {
            ui.label(format!("fps: {}", app.timer.fps().round()));
            ui.label(format!(
                "Chunk frame count: {:?}",
                chunk_manager.chunk_frame_count
            ));

            ui.label(format!(
                "Chunks render time: {:?}",
                chunk_manager.chunks_render_time
            ));

            // INFO: This is necessery because egui is annoying and without this the window twitches
            ui.label("                                                            ");

            ui.label(format!(
                "Chunks update time: {:?}",
                chunk_manager.chunks_update_time
            ));

            if chunk_manager.chunks_update_time > debug_info.longest_update_time {
                debug_info.longest_update_time = chunk_manager.chunks_update_time.clone();
            }

            ui.label(format!(
                "Biggest drop: {:?}",
                debug_info.longest_update_time
            ));
            if ui.button("Reset").clicked() {
                debug_info.longest_update_time = Duration::ZERO;
            }

            ui.add_space(5.);

            for i in 0..chunk_manager.num_of_threads.len() {
                ui.label(format!(
                    "Pass {} num of threads: {}",
                    i, chunk_manager.num_of_threads[i]
                ));
            }

            if ui.button("Show mem usage").clicked() {
                debug_info.debug_mem_usage = true;
            }
        });
}

pub fn debug_mem_usage(ctx: &Context, debug_info: &mut DebugInfo) {
    if !debug_info.debug_mem_usage {
        return;
    }

    Window::new("Mem usage")
        .resizable(false)
        .collapsible(true)
        .open(&mut debug_info.debug_mem_usage)
        .show(ctx, |ui| {
            if let Some(usage) = memory_stats() {
                ui.label(format!(
                    "Physical mem usage: {} mb",
                    usage.physical_mem / 1024 / 1024
                ));
                ui.label(format!(
                    "Virtual mem usage: {} mb",
                    usage.virtual_mem / 1024 / 1024
                ));
            } else {
                ui.label("Memory usage is unknown");
            }
        });
}

pub fn debug_sky_editor(ctx: &Context, debug_info: &mut DebugInfo) {
    Window::new("Sky color")
        .resizable(false)
        .collapsible(true)
        .open(&mut debug_info.sky_editor)
        .show(ctx, |ui| {
            ui.add(
                Slider::new(&mut debug_info.sky_color[0], 0..=255)
                    .clamp_to_range(true)
                    .prefix("r: "),
            );
            ui.add(
                Slider::new(&mut debug_info.sky_color[1], 0..=255)
                    .clamp_to_range(true)
                    .prefix("g: "),
            );
            ui.add(
                Slider::new(&mut debug_info.sky_color[2], 0..=255)
                    .clamp_to_range(true)
                    .prefix("b: "),
            );
        });
}

pub fn debug_rapier_window(ctx: &Context, debug_info: &mut DebugInfo, rapier_handler: &mut RapierHandler) {
    Window::new("Rapier2D").resizable(false).collapsible(true).open(&mut debug_info.debug_rapier2d).show(ctx, |ui| {
        ui.checkbox(&mut rapier_handler.update_phys, "Update rapier physics");
        ui.checkbox(&mut debug_info.debug_chunk_edges, "Show chunk colliders");
        ui.small("(Impacts performance)");
        ui.checkbox(&mut debug_info.debug_rs_body_edges, "Show rigid body edges");

        ui.add_space(5.);

        ComboBox::from_label("Rigid body type").selected_text(format!("{:?}", rapier_handler.select_body)).show_ui(ui, |ui| {
            ui.selectable_value(&mut rapier_handler.select_body, SelectBody::SandBodyBall, "SandBodyBall");
            ui.selectable_value(&mut rapier_handler.select_body, SelectBody::SandBodySquare, "SandBodySquare");
            ui.selectable_value(&mut rapier_handler.select_body, SelectBody::SandBodyRectangle, "SandBodyRectangle");
            ui.selectable_value(&mut rapier_handler.select_body, SelectBody::Ball, "Ball");
        });

        ui.add_space(5.);

        if ui.button("Clear balls").clicked() {
            rapier_handler.remove_balls();
        }
        if ui.button("Clear sand bodies").clicked() {
            rapier_handler.remove_sand_bodies();
        }
        
        ui.label(format!("Num of colliders: {}", rapier_handler.collider_set.len()));
        ui.label(format!("Num of rigid bodies: {}", rapier_handler.rigid_body_set.len()));
    });
}
