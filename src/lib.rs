use rand::prelude::*;
use wasm_bindgen::prelude::*;

mod ball;
mod paddle;
mod utils;

use ball::Ball;
use paddle::Paddle;

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 600.0;
const PADDLE_VEL: f32 = 0.8;

#[wasm_bindgen]
struct Game {
    left_paddle: Paddle,
    right_paddle: Paddle,
    ball: Ball,
    holding_up: bool,
    holding_down: bool,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        let mut game = Game {
            left_paddle: Paddle::new(),
            right_paddle: Paddle::new(),
            ball: Ball::new(),
            holding_up: false,
            holding_down: false,
        };

        game.ball.x = WIDTH / 2.0 - 5.0;
        game.ball.y = HEIGHT / 2.0 - 5.0;

        game.left_paddle.x = 20.0;
        game.left_paddle.y = 260.0;

        game.right_paddle.x = 770.0;
        game.right_paddle.y = 260.0;

        let mut rng = rand::thread_rng();
        let y: f32 = rng.gen();
        let x: f32 = rng.gen();

        let x = if x == 0.0 { -1.0 } else { 1.0 };

        let y = if y == 0.0 { -1.0 } else { 1.0 };

        game.ball.set_velocity(x, y);

        game
    }

    pub fn get_ball_x(&self) -> f32 {
        self.ball.x
    }

    pub fn get_ball_y(&self) -> f32 {
        self.ball.y
    }

    pub fn get_left_paddle_x(&self) -> f32 {
        self.left_paddle.x
    }

    pub fn get_left_paddle_y(&self) -> f32 {
        self.left_paddle.y
    }

    pub fn get_right_paddle_x(&self) -> f32 {
        self.right_paddle.x
    }

    pub fn get_right_paddle_y(&self) -> f32 {
        self.right_paddle.y
    }

    pub fn set_holding_down(&mut self, is_holding: bool) {
        self.holding_down = is_holding;
    }

    pub fn set_holding_up(&mut self, is_holding: bool) {
        self.holding_up = is_holding;
    }

    pub fn update(&mut self, delta: f32) {
        self.ball.move_ball(delta);

        if self.holding_down {
            self.left_paddle.y += PADDLE_VEL * delta;
        }
        if self.holding_up {
            self.left_paddle.y += -PADDLE_VEL * delta;
        }

        if self.left_paddle.y < 0.0 {
            self.left_paddle.y = 0.0;
        } else if self.left_paddle.y + 60.0 > HEIGHT {
            self.left_paddle.y = HEIGHT - 60.0;
        }

        if self.ball.x < 0.0 {
            self.ball.x = 0.0;
            self.ball.velocity_x *= -1.0;
        } else if self.ball.x > WIDTH {
            self.ball.x = WIDTH;
            self.ball.velocity_x *= -1.0;
        }

        if self.ball.y < 0.0 {
            self.ball.y = 0.0;
            self.ball.velocity_y *= -1.0;
        } else if self.ball.y > HEIGHT {
            self.ball.y = HEIGHT;
            self.ball.velocity_y *= -1.0;
        }

        let ball_box = (
            self.ball.x,
            self.ball.y,
            self.ball.x + 10.0,
            self.ball.y + 10.0,
        );
        let left_paddle_box = (
            self.left_paddle.x,
            self.left_paddle.y,
            self.left_paddle.x + 10.0,
            self.left_paddle.y + 60.0,
        );

        let x_overlaps = (ball_box.0 >= left_paddle_box.0 && ball_box.0 <= left_paddle_box.2)
            || (ball_box.0 <= left_paddle_box.0 && ball_box.2 >= left_paddle_box.0);
        let y_overlaps = (ball_box.1 >= left_paddle_box.1 && ball_box.1 <= left_paddle_box.3)
            || (ball_box.1 <= left_paddle_box.1 && ball_box.3 >= left_paddle_box.1);

        if x_overlaps && y_overlaps {
            self.ball.velocity_x *= -1.0;
        }
    }
}
