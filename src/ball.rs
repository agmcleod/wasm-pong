use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub velocity_x: f32,
    pub velocity_y: f32,
}

const VEL: f32 = 0.4;

impl Ball {
    pub fn new() -> Self {
        Ball {
            x: 0.0,
            y: 0.0,
            velocity_x: 0.0,
            velocity_y: 0.0,
        }
    }

    pub fn set_velocity(&mut self, x: f32, y: f32) {
        self.velocity_x = x;
        self.velocity_y = y;
    }

    pub fn move_ball(&mut self, delta: f32) {
        self.x += self.velocity_x * VEL * delta;
        self.y += self.velocity_y * VEL * delta;
    }
}
