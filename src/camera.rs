use notan::{math::{Vec2, vec2, Mat3, Mat4}, draw::Draw};

pub struct Camera2D {
    pub work_size: Vec2,
    pub pos: Vec2,
    scale: Vec2,
    transform: Mat3,
    dirty: bool,
}

impl Camera2D {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        let size = vec2(width, height);
        let pos = Vec2::new(x, y);
        let scale = Vec2::splat(1.0);
        let transform = Mat3::IDENTITY;

        Self {
            work_size: size,
            pos,
            scale,
            transform,
            dirty: true,
        }
    }

    pub fn apply(&mut self, draw: &mut Draw) {
        if self.dirty {
            self.dirty = false;
            self.calculate_transform();
        }

    	let projection = Mat4::orthographic_rh_gl(0., self.work_size.x, self.work_size.y, 0., -1., 1.);
        draw.set_projection(Some(projection));
        draw.transform().push(self.transform);
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        let pos = vec2(x, y);
        if self.pos != pos {
            self.dirty = true;
            self.pos = pos;
        }
    }

    pub fn position(&self) -> Vec2 {
        self.pos
    }

    pub fn pos_add_x(&mut self, x: f32) {
        self.dirty = true;
        self.pos.x += x;
    }

    pub fn pos_add_y(&mut self, y: f32) {
        self.dirty = true;
        self.pos.y += y;
    }

    pub fn set_position_to_center(&mut self) {
        self.set_position(self.work_size.x * 0.5, self.work_size. y * 0.5);
    }

    pub fn set_scale(&mut self, x: f32, y: f32) {
        let scale = vec2(x, y);
        if self.scale != scale {
            self.dirty = true;
            self.scale = scale;
        }
    }

    pub fn scale(&self) -> Vec2 {
        self.scale
    }

    pub fn set_zoom(&mut self, factor: f32) {
        self.set_scale(factor, factor);
    }

    fn calculate_transform(&mut self) {
        let pos = self.pos - self.work_size * 0.5 / self.scale;
        let translate = Mat3::from_translation(pos * -1.0);
        let scale = Mat3::from_scale(self.scale);
        self.transform = scale * translate;
    }
}

