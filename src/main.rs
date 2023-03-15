mod grid;
mod element;
pub mod movement;

use element::*;
use grid::*;
use notan::draw::*;
use notan::prelude::*;

#[derive(AppState)]
struct State {
    grid: Grid,
    selected_element: Cell
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(init)
        .add_config(WindowConfig::new().size(1280, 720).vsync(true))
        .add_config(DrawConfig)
        .update(update)
        .draw(draw)
        .build()
}

fn init(_app: &mut App, gfx: &mut Graphics) -> State {
    State {
        grid: Grid::new(0., 0., gfx),
        selected_element: sand_element()
    }
}

fn update(app: &mut App, state: &mut State) {
    state.grid.update();
    
    let mouse = state.grid.mouse_in_sim(app);
    if app.mouse.left_is_down() {
        state.grid.modify_elements(mouse.0, mouse.1, 32, &state.selected_element);
    }

    if app.mouse.right_is_down() {
        state.grid.explode(mouse.0, mouse.1, 100, 4.);
    }

    if app.keyboard.was_pressed(KeyCode::Key1) {
        state.selected_element = air_element();
    } else if app.keyboard.was_pressed(KeyCode::Key2) {
        state.selected_element = solid_element();
    } else if app.keyboard.was_pressed(KeyCode::Key3) {
        state.selected_element = sand_element();
        println!("{}", app.timer.fps());
    } else if app.keyboard.was_pressed(KeyCode::Key4) {
        state.selected_element = sawdust_element();
    } else if app.keyboard.was_pressed(KeyCode::Key5) {
        state.selected_element = water_element();
    } else if app.keyboard.was_pressed(KeyCode::Key6) {
        state.selected_element = smoke_element();
    }
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    state.grid.render(gfx, &mut draw); 

    draw.ellipse((app.mouse.x, app.mouse.y), (32., 32.)).stroke_color(Color::WHITE).fill_color(Color::from_rgba(0., 0., 0., 0.)).stroke(1.);
    
    gfx.render(&draw);
}
