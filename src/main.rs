mod chunk;
mod element;
pub mod movement;
pub mod camera;
pub mod input_manager;
pub mod chunk_manager;

use camera::Camera2D;
use chunk::{UPSCALE_FACTOR, ROWS, COLS};
use chunk_manager::ChunkManager;
use element::*;
use notan::draw::*;
use notan::egui::epaint::Shadow;
use notan::egui::{EguiConfig, EguiPluginSugar, Window, Slider, Visuals};
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
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(init)
        .add_config(WindowConfig::new().size(1920, 1080).vsync(false).title("arm'st sandbox").resizable(true).multisampling(0))
        .add_config(DrawConfig)
        .add_config(EguiConfig)
        .update(update)
        .draw(draw)
        .build()
}

fn init(app: &mut App, gfx: &mut Graphics) -> State {
    State {
        chunk_manager: ChunkManager::new(gfx),
        editor_open: true,
        camera: Camera2D::new(COLS as f32 / 2. * UPSCALE_FACTOR, ROWS as f32 / 2. * UPSCALE_FACTOR, app.window().width() as f32, app.window().height() as f32),
        camera_zoom: 1.0,
        sky_gradient: gfx.create_texture().from_image(include_bytes!("assets/sky_gradient.png")).with_filter(TextureFilter::Linear, TextureFilter::Linear).build().unwrap(),
        debug_window: false,
        debug_render: false,
        debug_chunk_coords: true
    }
}

fn update(app: &mut App, state: &mut State) {
    state.chunk_manager.update(app, &state.camera);

    if app.keyboard.was_pressed(KeyCode::R) {
        state.editor_open = !state.editor_open;
    }
    if app.keyboard.was_pressed(KeyCode::Space) {
        state.chunk_manager.update_chunks = !state.chunk_manager.update_chunks;
    }
    if app.keyboard.was_pressed(KeyCode::F) {
        state.debug_render = !state.debug_render;
    }

    if app.keyboard.was_pressed(KeyCode::Escape) {
        app.exit();
    }

    input_manager::camera_control(app, &mut state.camera, &mut state.camera_zoom);
}

fn draw(app: &mut App, gfx: &mut Graphics, plugins: &mut Plugins, state: &mut State) {
    let mut render_draw = gfx.create_draw();
    render_draw.clear(Color::BLACK);

    render_draw.image(&state.sky_gradient).position(0., 0.).size(app.window().width() as f32, app.window().height() as f32);

    state.camera.apply(&mut render_draw);

    state.chunk_manager.render(gfx, &mut render_draw, state.debug_render, state.debug_chunk_coords);

    render_draw.transform().pop();

    render_draw.ellipse((app.mouse.x, app.mouse.y), (state.chunk_manager.brush_size as f32 * state.camera_zoom * 0.5 * UPSCALE_FACTOR, state.chunk_manager.brush_size as f32 * state.camera_zoom * 0.5 * UPSCALE_FACTOR,))
        .stroke_color(Color::WHITE).fill_color(Color::from_rgba(0., 0., 0., 0.))
        .stroke(1.);

    gfx.render(&render_draw);

    let output = plugins.egui(|ctx| {
        let mut visuals = Visuals::dark();
        visuals.window_shadow = Shadow::NONE;
        ctx.set_visuals(visuals);

        Window::new("Editor").resizable(false).collapsible(false).title_bar(true).open(&mut state.editor_open).show(ctx, |ui| {
            state.chunk_manager.modify = !ctx.is_pointer_over_area();

            ui.label(format!("fps: {}", app.timer.fps().round()));
            ui.add_space(5.);

            ui.horizontal(|ui| {
                if ui.button("Air").clicked() {
                    state.chunk_manager.selected_element = air_element();
                }
                if ui.button("Solid").clicked() {
                    state.chunk_manager.selected_element = solid_element();
                }
                if ui.button("Sand").clicked() {
                    state.chunk_manager.selected_element = sand_element();
                }
                if ui.button("SawDust").clicked() {
                    state.chunk_manager.selected_element = sawdust_element();
                }
                if ui.button("Water").clicked() {
                    state.chunk_manager.selected_element = water_element();
                }
                if ui.button("Smoke").clicked() {
                    state.chunk_manager.selected_element = smoke_element();
                }
                if ui.button("Steam").clicked() {
                    state.chunk_manager.selected_element = steam_element();
                }
            });
            ui.add_space(5.);

            let brush_slider = Slider::new(&mut state.chunk_manager.brush_size, 2..=200);
            ui.add(brush_slider);

            ui.checkbox(&mut state.debug_window, "Debug window");

            Window::new("Debug window").resizable(false).collapsible(false).title_bar(true).open(&mut state.debug_window).show(ctx, |ui| {
                ui.checkbox(&mut state.chunk_manager.update_chunks, "Update");
                ui.checkbox(&mut state.debug_render, "Chunk borders");
                ui.checkbox(&mut state.debug_chunk_coords, "Chunk indices");

                ui.label(format!("{:?}", state.chunk_manager.hovering_cell));
            });
        });
        
        if !state.editor_open {
            state.chunk_manager.modify = true;
        }
    });

    gfx.render(&output);
}
