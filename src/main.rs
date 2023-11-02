mod chunk;
mod element;
pub mod base_movement;
pub mod camera;
pub mod input_manager;
pub mod chunk_manager;
pub mod el_movement;
pub mod element_actions;

use camera::Camera2D;
use chunk::{UPSCALE_FACTOR, ROWS, COLS};
use chunk_manager::ChunkManager;
use element::*;
use memory_stats::memory_stats;
use notan::draw::*;
use notan::egui::epaint::Shadow;
use notan::egui::{EguiConfig, EguiPluginSugar, Window, Slider, Visuals, RichText, Color32};
use notan::prelude::*;

#[derive(AppState)]
struct State {
    chunk_manager: ChunkManager,
    editor_open: bool,
    camera: Camera2D,
    camera_zoom: f32,
    sky_gradient: Texture,
    debug_window: bool,
    debug_render: bool,
    debug_chunk_coords: bool,
    debug_dirty_rects: bool,
    debug_metrics: bool,
    sky_color: [u8; 3],
    sky_editor: bool
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(init)
        .add_config(WindowConfig::new()
                    .set_size(1920, 1080).set_fullscreen(false)
                    .set_vsync(false) // TODO: Bug: sim runs slower with vsync enabled
                    .set_title("arm'st sandbox")
                    .set_resizable(false)
                    .set_multisampling(0)
                    .set_high_dpi(true))
        .add_config(DrawConfig)
        .add_config(EguiConfig)
        .update(update)
        .draw(draw)
        .build()
}

fn init(app: &mut App, gfx: &mut Graphics) -> State {
    let screen_size = app.window().container_size();
    let window_size = app.window().size();
    let dpi = app.window().dpi();
    app.window().set_position((((screen_size.0 as f64 - window_size.0 as f64) / 2.) * dpi) as i32, (((screen_size.1 as f64 - window_size.1 as f64) / 2.) * dpi) as i32);

    State {
        chunk_manager: ChunkManager::new(gfx),
        editor_open: true,
        camera: Camera2D::new(COLS as f32 / 2. * UPSCALE_FACTOR, ROWS as f32 / 2. * UPSCALE_FACTOR, app.window().width() as f32, app.window().height() as f32),
        camera_zoom: 1.0,
        sky_gradient: gfx.create_texture().from_image(include_bytes!("assets/sky_gradient.png")).with_filter(TextureFilter::Linear, TextureFilter::Linear).build().unwrap(),
        debug_window: false,
        debug_render: false,
        debug_chunk_coords: true,
        debug_dirty_rects: false,
        debug_metrics: false,
        sky_color: [70, 35, 70],
        sky_editor: false
    }
}

fn update(app: &mut App, state: &mut State) {
    state.chunk_manager.update(app, &state.camera);

    if app.keyboard.was_pressed(KeyCode::R) {
        state.editor_open = !state.editor_open;
    }
    if app.keyboard.was_pressed(KeyCode::Y) {
        state.sky_editor = !state.sky_editor;
    }
    if app.keyboard.was_pressed(KeyCode::Space) {
        state.chunk_manager.update_chunks = !state.chunk_manager.update_chunks;
    }
    if app.keyboard.was_pressed(KeyCode::F) {
        state.debug_render = !state.debug_render;
    }
    if app.keyboard.was_pressed(KeyCode::T) {
        state.debug_window = !state.debug_window;
    }
    if app.keyboard.was_pressed(KeyCode::M) {
        state.debug_metrics = !state.debug_metrics;
    }

    if app.keyboard.was_pressed(KeyCode::Escape) {
        app.exit();
    }

    input_manager::camera_control(app, &mut state.camera, &mut state.camera_zoom);
}

fn draw(app: &mut App, gfx: &mut Graphics, plugins: &mut Plugins, state: &mut State) {
    let mut render_draw = gfx.create_draw();
    render_draw.clear(Color::BLACK);

    render_draw.image(&state.sky_gradient)
                .position(0., 0.)
                .size(app.window().width() as f32, app.window().height() as f32)
                .color(Color::from_bytes(state.sky_color[0], state.sky_color[1], state.sky_color[2], 255));

    state.camera.apply(&mut render_draw);

    state.chunk_manager.render(gfx, &mut render_draw, state.debug_render, state.debug_chunk_coords, state.debug_dirty_rects);

    render_draw.transform().pop();

    render_draw.ellipse((app.mouse.x, app.mouse.y), (state.chunk_manager.brush_size as f32 * state.camera_zoom * 0.5 * UPSCALE_FACTOR, state.chunk_manager.brush_size as f32 * state.camera_zoom * 0.5 * UPSCALE_FACTOR,))
        .stroke_color(Color::WHITE).fill_color(Color::from_rgba(0., 0., 0., 0.))
        .stroke(1.);

    gfx.render(&render_draw);

    let output = plugins.egui(|ctx| {
        let mut visuals = Visuals::dark();
        visuals.window_shadow = Shadow::NONE;
        ctx.set_visuals(visuals);

        state.chunk_manager.modify = !ctx.is_pointer_over_area();

        Window::new("Editor").resizable(false).collapsible(true).title_bar(true).open(&mut state.editor_open).show(ctx, |ui| {
            ui.label(format!("fps: {}", app.timer.fps().round()));

            ui.add_space(5.);

            ui.horizontal(|ui| {
                if ui.button(RichText::new("Air").color(Color32::from_rgb(255, 255, 255))).clicked() {
                    state.chunk_manager.selected_element = air_element();
                }
                if ui.button(RichText::new("Solid").color(Color32::from_rgb(169, 162, 166))).clicked() {
                    state.chunk_manager.selected_element = solid_element();
                }
                if ui.button(RichText::new("Sand").color(Color32::from_rgb(243, 239, 118))).clicked() {
                    state.chunk_manager.selected_element = sand_element();
                }
                if ui.button(RichText::new("Dirt").color(Color32::from_rgb(136, 107, 82))).clicked() {
                    state.chunk_manager.selected_element = dirt_element();
                }
                if ui.button(RichText::new("Coal").color(Color32::from_rgb(130, 130, 130))).clicked() {
                    state.chunk_manager.selected_element = coal_element();
                }
                if ui.button(RichText::new("Wood").color(Color32::from_rgb(111, 83, 57))).clicked() {
                    state.chunk_manager.selected_element = wood_element();
                }
                if ui.button(RichText::new("SawDust").color(Color32::from_rgb(181, 137, 100))).clicked() {
                    state.chunk_manager.selected_element = sawdust_element();
                }
            });
            ui.add_space(2.);
            ui.horizontal(|ui| {
                if ui.button(RichText::new("Water").color(Color32::from_rgb(75, 66, 249))).clicked() {
                    state.chunk_manager.selected_element = water_element();
                }
                if ui.button(RichText::new("Smoke").color(Color32::from_rgb(142, 142, 142))).clicked() {
                    state.chunk_manager.selected_element = smoke_element();
                }
                if ui.button(RichText::new("Steam").color(Color32::from_rgb(143, 159, 234))).clicked() {
                   state.chunk_manager.selected_element = steam_element();
                }
                if ui.button(RichText::new("Methane").color(Color32::from_rgb(130, 171, 41))).clicked() {
                    state.chunk_manager.selected_element = methane_element();
                 }
                if ui.button(RichText::new("Fire").color(Color32::from_rgb(255, 0, 0))).clicked() {
                   state.chunk_manager.selected_element = fire_element();
                }
            });
            ui.add_space(5.);

            let brush_slider = Slider::new(&mut state.chunk_manager.brush_size, 1..=200).clamp_to_range(false);
            ui.add(brush_slider);
            ui.checkbox(&mut state.chunk_manager.replace_air, "Replace only air");
            ui.checkbox(&mut state.chunk_manager.update_chunks, "Update");

            ui.label("Press Y to modify sky color");
            ui.label("Press T for debug info");
            ui.label("Press M for metrics");
        });

        Window::new("Debug window").resizable(false).collapsible(true).title_bar(true).open(&mut state.debug_window).show(ctx, |ui| {
            ui.checkbox(&mut state.debug_render, "Chunk borders");
            ui.checkbox(&mut state.debug_chunk_coords, "Chunk indices");
            ui.checkbox(&mut state.debug_dirty_rects, "Dirty rects");
            ui.add_space(5.);

            ui.label(RichText::new("Mouse is on: ").color(Color32::from_rgb(180, 180, 180)));
            ui.label("Cell {");
            ui.label(format!("    element: {:?} ", state.chunk_manager.hovering_cell.element));
            ui.label(format!("    action: {:?} ", state.chunk_manager.hovering_cell.action));
            ui.label(format!("    state: {:?}", state.chunk_manager.hovering_cell.state));
            ui.label(format!("    velocity: Vec2({:.2}, {:.2})", state.chunk_manager.hovering_cell.velocity.x, state.chunk_manager.hovering_cell.velocity.y));
            ui.label(format!("    density: {:?}", state.chunk_manager.hovering_cell.density));
            ui.label(format!("    drag: {:?}", state.chunk_manager.hovering_cell.drag));
            ui.label(format!("    color: {:?}", state.chunk_manager.hovering_cell.color));
            ui.label(format!("    lifetime: {:?}", state.chunk_manager.hovering_cell.lifetime));
            ui.label("}");
        });

        Window::new("Metrics").resizable(false).collapsible(true).open(&mut state.debug_metrics).show(ctx, |ui| {
            ui.label(format!("fps: {}", app.timer.fps().round()));

            ui.label(format!("Chunks update time: {:?}", state.chunk_manager.chunks_update_time));
            if let Some(usage) = memory_stats() {
                ui.label(format!("Physical mem usage: {} mb", usage.physical_mem / 1024 / 1024));
                ui.label(format!("Virtual mem usage: {} mb", usage.virtual_mem / 1024 / 1024));
            } else {
                ui.label("Memory usage is unknown");
            }

            for i in 0..state.chunk_manager.num_of_threads.len() {
                ui.label(format!("Pass {} num of threads: {}", i, state.chunk_manager.num_of_threads[i]));
            }
            ui.label("                                                            "); // This is necessery because egui is annoying and without this the window twitches
        });

        Window::new("Sky color").resizable(false).collapsible(true).open(&mut state.sky_editor).show(ctx, |ui| {
            ui.add(Slider::new(&mut state.sky_color[0], 0..=255).clamp_to_range(true).prefix("r: "));
            ui.add(Slider::new(&mut state.sky_color[1], 0..=255).clamp_to_range(true).prefix("g: "));
            ui.add(Slider::new(&mut state.sky_color[2], 0..=255).clamp_to_range(true).prefix("b: "));
        });
    });

    gfx.render(&output);
}
