mod grid;
mod element;
pub mod movement;

use element::*;
use grid::*;
use notan::draw::*;
use notan::egui::{EguiConfig, EguiPluginSugar, Window, Slider};
use notan::prelude::*;

#[derive(AppState)]
struct State {
    grid: Grid,
    selected_element: Cell,
    modify: bool,
    editor_open: bool,
    brush_size: i32
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(init)
        .add_config(WindowConfig::new().size(1280, 720).vsync(true).title("arms sandbox"))
        .add_config(DrawConfig)
        .add_config(EguiConfig)
        .update(update)
        .draw(draw)
        .build()
}

fn init(_app: &mut App, gfx: &mut Graphics) -> State {
    State {
        grid: Grid::new(0., 0., gfx),
        selected_element: sand_element(),
        modify: true,
        editor_open: true,
        brush_size: 32
    }
}

fn update(app: &mut App, state: &mut State) {
    state.grid.update();
    
    let mouse = state.grid.mouse_in_sim(app);
    if app.mouse.left_is_down() && state.modify {
        state.grid.modify_elements(mouse.0, mouse.1, state.brush_size, &state.selected_element);
    }

    if app.mouse.right_is_down() && state.modify {
        state.grid.explode(mouse.0, mouse.1, state.brush_size * 2, 4.);
    }
    
    if app.keyboard.was_pressed(KeyCode::E) {
        state.editor_open = !state.editor_open;
    }
}

fn draw(app: &mut App, gfx: &mut Graphics, plugins: &mut Plugins, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    state.grid.render(gfx, &mut draw); 
    draw.ellipse((app.mouse.x, app.mouse.y), (state.brush_size as f32 * 1.5, state.brush_size as f32 * 1.5)).stroke_color(Color::WHITE).fill_color(Color::from_rgba(0., 0., 0., 0.)).stroke(1.);

    gfx.render(&draw);

    let output = plugins.egui(|ctx| {
        Window::new("Editor").resizable(false).collapsible(false).open(&mut state.editor_open).show(ctx, |ui| {
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
            });
            ui.add_space(5.);

            let brush_slider = Slider::new(&mut state.brush_size, 2..=200);
            ui.add(brush_slider);
        });
        
        if !state.editor_open {
            state.modify = true;
        }
    });

    gfx.render(&output);
}
