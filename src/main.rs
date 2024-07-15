pub mod camera;
mod debug_ui;
pub mod input_manager;
pub mod phys_world;

use camera::Camera2D;
use debug_ui::DebugInfo;
use notan::draw::*;
use notan::egui::{EguiConfig, EguiPluginSugar};
use notan::prelude::*;
use phys_world::chunk::{COLS, ROWS, UPSCALE_FACTOR};
use phys_world::chunk_manager::ChunkManager;

#[derive(AppState)]
struct State {
    chunk_manager: ChunkManager,
    camera: Camera2D,
    camera_zoom: f32,
    sky_gradient: Texture,
    debug_info: DebugInfo,
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(init)
        .add_config(
            WindowConfig::new()
                .set_size(1920, 1080)
                .set_fullscreen(false)
                .set_vsync(false)
                .set_title("arm'st sandbox")
                .set_resizable(false)
                .set_multisampling(0)
                .set_high_dpi(false),
        )
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
    app.window().set_position(
        (((screen_size.0 as f64 - window_size.0 as f64) / 2.) * dpi) as i32,
        (((screen_size.1 as f64 - window_size.1 as f64) / 2.) * dpi) as i32 - 80,
    );

    State {
        chunk_manager: ChunkManager::new(gfx),
        camera: Camera2D::new(
            COLS as f32 / 2. * UPSCALE_FACTOR,
            ROWS as f32 / 2. * UPSCALE_FACTOR,
            app.window().width() as f32,
            app.window().height() as f32,
        ),
        camera_zoom: 1.0,
        sky_gradient: gfx
            .create_texture()
            .from_image(include_bytes!("assets/sky_gradient.png"))
            .with_filter(TextureFilter::Linear, TextureFilter::Linear)
            .build()
            .unwrap(),
        debug_info: DebugInfo::default(),
    }
}

fn update(app: &mut App, state: &mut State) {
    state.chunk_manager.update(app, &state.camera);

    if app.keyboard.was_pressed(KeyCode::Escape) {
        app.exit();
    }

    input_manager::camera_control(app, &mut state.camera, &mut state.camera_zoom);
}

fn draw(app: &mut App, gfx: &mut Graphics, plugins: &mut Plugins, state: &mut State) {
    let mut render_draw = gfx.create_draw();
    render_draw.clear(Color::BLACK);

    render_draw
        .image(&state.sky_gradient)
        .position(0., 0.)
        .size(app.window().width() as f32, app.window().height() as f32)
        .color(Color::from_bytes(
            state.debug_info.sky_color[0],
            state.debug_info.sky_color[1],
            state.debug_info.sky_color[2],
            255,
        ));

    state.camera.apply(&mut render_draw);

    state.chunk_manager.render(gfx, &mut render_draw);
    state.chunk_manager.debug_render(&mut render_draw, &state.debug_info);

    render_draw.transform().pop();

    render_draw
        .ellipse(
            (app.mouse.x, app.mouse.y),
            (
                state.chunk_manager.brush_size as f32 * state.camera_zoom * 0.5 * UPSCALE_FACTOR,
                state.chunk_manager.brush_size as f32 * state.camera_zoom * 0.5 * UPSCALE_FACTOR,
            ),
        )
        .stroke_color(Color::WHITE)
        .fill_color(Color::from_rgba(0., 0., 0., 0.))
        .stroke(1.);

    gfx.render(&render_draw);

    let output = plugins.egui(|ctx| {
        state.chunk_manager.modify = !ctx.is_pointer_over_area();

        debug_ui::debug_ui(ctx, app, &mut state.debug_info, &mut state.chunk_manager);
    });

    gfx.render(&output);
}
