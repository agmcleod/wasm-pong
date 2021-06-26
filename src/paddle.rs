use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Paddle {
    pub x: f32,
    pub y: f32,
}

impl Paddle {
    pub fn new() -> Self {
        Paddle { x: 0.0, y: 0.0 }
    }
}
