mod grid;
mod element;
pub mod movement;
pub mod camera;
pub mod input_manager;

use camera::Camera2D;
use element::*;
use grid::*;
use input_manager::get_mouse_in_world;
use notan::draw::*;
use notan::egui::{EguiConfig, EguiPluginSugar, Window, Slider};
use notan::prelude::*;

#[derive(AppState)]
struct State {
    grid: Grid,
    selected_element: Cell,
    modify: bool,
    editor_open: bool,
    brush_size: i32,
    pause: bool,
    camera: Camera2D,
    camera_zoom: f32,
    sky_gradient: Texture
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(init)
        .add_config(WindowConfig::new().size(1920, 1080).vsync(true).title("arm's sandbox"))
        .add_config(DrawConfig)
        .add_config(EguiConfig)
        .update(update)
        .draw(draw)
        .build()
}

fn init(app: &mut App, gfx: &mut Graphics) -> State {
    State {
        grid: Grid::new(350., 350., gfx),
        selected_element: sand_element(),
        modify: true,
        editor_open: true,
        brush_size: 32,
        pause: false,
        camera: Camera2D::new(app.window().width() as f32 / 2., app.window().height() as f32 / 2., app.window().width() as f32, app.window().height() as f32),
        camera_zoom: 1.0,
        sky_gradient: gfx.create_texture().from_image(include_bytes!("assets/sky_gradient.png")).with_filter(TextureFilter::Linear, TextureFilter::Linear).build().unwrap()
    }
}

fn update(app: &mut App, state: &mut State) {
    if !state.pause {
        state.grid.update();
    }

    let mouse_world = get_mouse_in_world(&(app.mouse.x, app.mouse.y), (app.window().width(), app.window().height()), &state.camera);
    let mouse = state.grid.mouse_in_sim(mouse_world, app);

    match mouse {
        Some(mouse) => {
            if app.mouse.left_is_down() && state.modify {
                state.grid.modify_elements(mouse.0, mouse.1, state.brush_size, &state.selected_element);
            }

            if app.mouse.right_is_down() && state.modify {
                state.grid.explode(mouse.0, mouse.1, state.brush_size * 2, 4.);
            }
        },
        _ => ()
    }
        
    if app.keyboard.was_pressed(KeyCode::R) {
        state.editor_open = !state.editor_open;
    }
    if app.keyboard.was_pressed(KeyCode::Space) {
        state.pause = !state.pause;
        
    }

    if app.keyboard.was_pressed(KeyCode::Escape) {
        app.exit();
    }

    input_manager::camera_control(app, &mut state.camera, &mut state.camera_zoom);
}

fn draw(app: &mut App, gfx: &mut Graphics, plugins: &mut Plugins, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    draw.image(&state.sky_gradient).position(0., 0.).size(app.window().width() as f32, app.window().height() as f32);

    state.camera.apply(&mut draw);
    state.grid.render(gfx, &mut draw);

    draw.transform().pop();
    draw.ellipse((app.mouse.x, app.mouse.y), (state.brush_size as f32 * state.camera_zoom, state.brush_size as f32 * state.camera_zoom)).stroke_color(Color::WHITE).fill_color(Color::from_rgba(0., 0., 0., 0.)).stroke(1.);

    gfx.render(&draw);

    let output = plugins.egui(|ctx| {
        Window::new("Editor").resizable(false).collapsible(false).title_bar(false).open(&mut state.editor_open).show(ctx, |ui| {
            state.modify = !ctx.is_pointer_over_area();

            ui.label(format!("fps: {}", app.timer.fps().round()));
            ui.add_space(5.);

            ui.horizontal(|ui| {
                if ui.button("Air").clicked() {
                    state.selected_element = air_element();
                }
                if ui.button("Solid").clicked() {
                    state.selected_element = solid_element();
                }
                if ui.button("Sand").clicked() {
                    state.selected_element = sand_element();
                }
                if ui.button("SawDust").clicked() {
                    state.selected_element = sawdust_element();
                }
                if ui.button("Water").clicked() {
                    state.selected_element = water_element();
                }
                if ui.button("Smoke").clicked() {
                    state.selected_element = smoke_element();
                }
                if ui.button("Steam").clicked() {
                    state.selected_element = steam_element();
                }
            });
            ui.add_space(5.);

            let brush_slider = Slider::new(&mut state.brush_size, 2..=200);
            ui.add(brush_slider);

            ui.checkbox(&mut state.pause, "Pause");
        });
        
        if !state.editor_open {
            state.modify = true;
        }
    });

    gfx.render(&output);
}
