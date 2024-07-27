use notan::prelude::{App, KeyCode};

use crate::camera::Camera2D;

pub fn camera_control(app: &mut App, camera: &mut Camera2D, camera_zoom: &mut f32) {
    let mut speed = 5. * app.timer.delta_f32() * 60.;
    if app.keyboard.shift() {
        speed = 10.* app.timer.delta_f32() * 60.;
    }
    if app.keyboard.is_down(KeyCode::D) {
        camera.pos_add_x(speed);
    }
    if app.keyboard.is_down(KeyCode::A) {
        camera.pos_add_x(-speed);
    }
    if app.keyboard.is_down(KeyCode::S) {
        camera.pos_add_y(speed);
    }
    if app.keyboard.is_down(KeyCode::W) {
        camera.pos_add_y(-speed);
    }
    camera.set_zoom(*camera_zoom);

    if app.keyboard.is_down(KeyCode::Q) {
        *camera_zoom -= *camera_zoom * app.timer.delta_f32();
    }
    if app.keyboard.is_down(KeyCode::E) {
        *camera_zoom += *camera_zoom * app.timer.delta_f32();
    }
}

pub fn get_mouse_in_world(mouse_pos: &(f32, f32), window_size: (i32, i32), camera: &Camera2D) -> (f32, f32) {
    let mouse_x = map(mouse_pos.0, 0.0, window_size.0 as f32, 0.0, camera.work_size.x / camera.scale().x);
    let mouse_y = map(mouse_pos.1, 0.0, window_size.1 as f32, 0.0, camera.work_size.y / camera.scale().y);

    (camera.pos.x - camera.work_size.x / 2.0 / camera.scale().x + mouse_x, camera.pos.y - camera.work_size.y / 2.0 / camera.scale().y + mouse_y)
}

pub fn map(value: f32, begin: f32, end: f32, new_begin: f32, new_end: f32) -> f32 {
    new_begin + (new_end - new_begin) * ((value - begin) / (end - begin))
}
