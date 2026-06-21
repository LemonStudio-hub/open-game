use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

type AnimationFrameClosure = Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>>;

use opengame_engine::color::Color;
use opengame_engine::input::{keys::KeyCode, InputManager};
use opengame_engine::renderer::{Camera2D, GlBackend, ShapeRenderer};
use opengame_engine::time::Time;

const PADDLE_WIDTH: f32 = 12.0;
const PADDLE_HEIGHT: f32 = 80.0;
const PADDLE_SPEED: f32 = 400.0;
const PADDLE_MARGIN: f32 = 30.0;
const BALL_SIZE: f32 = 10.0;
const BALL_SPEED: f32 = 350.0;
const BALL_SPEED_INCREMENT: f32 = 15.0;
const WIN_SCORE: i32 = 7;

struct PongGame {
    gl: GlBackend,
    shapes: ShapeRenderer,
    camera: Camera2D,
    input: InputManager,
    time: Time,
    left_paddle_y: f32,
    right_paddle_y: f32,
    ball_x: f32,
    ball_y: f32,
    ball_vx: f32,
    ball_vy: f32,
    left_score: i32,
    right_score: i32,
    game_over: bool,
    winner: &'static str,
}

impl PongGame {
    fn new() -> Result<Self, String> {
        opengame_engine::log::init();

        let gl = GlBackend::new("game-canvas")?;
        let camera = Camera2D::new(gl.width() as f32, gl.height() as f32);
        let shapes = ShapeRenderer::new(gl.gl())?;
        let input = InputManager::new()?;

        let window = web_sys::window().ok_or("No window")?;
        let performance = window.performance().ok_or("No performance")?;
        let time = Time::new(performance);

        let width = gl.width() as f32;
        let height = gl.height() as f32;

        Ok(Self {
            gl,
            shapes,
            camera,
            input,
            time,
            left_paddle_y: height / 2.0,
            right_paddle_y: height / 2.0,
            ball_x: width / 2.0,
            ball_y: height / 2.0,
            ball_vx: BALL_SPEED,
            ball_vy: BALL_SPEED * 0.5,
            left_score: 0,
            right_score: 0,
            game_over: false,
            winner: "",
        })
    }

    fn reset_ball(&mut self) {
        let width = self.gl.width() as f32;
        let height = self.gl.height() as f32;
        self.ball_x = width / 2.0;
        self.ball_y = height / 2.0;
        self.ball_vx = if self.ball_vx > 0.0 {
            -BALL_SPEED
        } else {
            BALL_SPEED
        };
        self.ball_vy = BALL_SPEED * 0.5 * if rand_bool() { 1.0 } else { -1.0 };
    }

    fn update(&mut self, dt: f32) {
        if self.game_over {
            if self.input.is_key_pressed(KeyCode::Space) {
                self.left_score = 0;
                self.right_score = 0;
                self.game_over = false;
                self.winner = "";
                self.reset_ball();
            }
            return;
        }

        let height = self.gl.height() as f32;
        let half_paddle = PADDLE_HEIGHT / 2.0;

        if self.input.is_key_down(KeyCode::KeyW) {
            self.left_paddle_y += PADDLE_SPEED * dt;
        }
        if self.input.is_key_down(KeyCode::KeyS) {
            self.left_paddle_y -= PADDLE_SPEED * dt;
        }
        self.left_paddle_y = self.left_paddle_y.clamp(half_paddle, height - half_paddle);

        if self.input.is_key_down(KeyCode::ArrowUp) {
            self.right_paddle_y += PADDLE_SPEED * dt;
        }
        if self.input.is_key_down(KeyCode::ArrowDown) {
            self.right_paddle_y -= PADDLE_SPEED * dt;
        }
        self.right_paddle_y = self.right_paddle_y.clamp(half_paddle, height - half_paddle);

        self.ball_x += self.ball_vx * dt;
        self.ball_y += self.ball_vy * dt;

        let half_ball = BALL_SIZE / 2.0;
        if self.ball_y - half_ball <= 0.0 {
            self.ball_y = half_ball;
            self.ball_vy = self.ball_vy.abs();
        }
        if self.ball_y + half_ball >= height {
            self.ball_y = height - half_ball;
            self.ball_vy = -self.ball_vy.abs();
        }

        let left_paddle_x = PADDLE_MARGIN + PADDLE_WIDTH / 2.0;
        if self.ball_x - half_ball <= left_paddle_x + PADDLE_WIDTH / 2.0
            && self.ball_x + half_ball >= left_paddle_x - PADDLE_WIDTH / 2.0
            && self.ball_y + half_ball >= self.left_paddle_y - half_paddle
            && self.ball_y - half_ball <= self.left_paddle_y + half_paddle
            && self.ball_vx < 0.0
        {
            self.ball_x = left_paddle_x + PADDLE_WIDTH / 2.0 + half_ball;
            self.ball_vx = -self.ball_vx + BALL_SPEED_INCREMENT;
            let hit_pos = (self.ball_y - self.left_paddle_y) / half_paddle;
            self.ball_vy = hit_pos * BALL_SPEED;
        }

        let right_paddle_x = self.gl.width() as f32 - PADDLE_MARGIN - PADDLE_WIDTH / 2.0;
        if self.ball_x + half_ball >= right_paddle_x - PADDLE_WIDTH / 2.0
            && self.ball_x - half_ball <= right_paddle_x + PADDLE_WIDTH / 2.0
            && self.ball_y + half_ball >= self.right_paddle_y - half_paddle
            && self.ball_y - half_ball <= self.right_paddle_y + half_paddle
            && self.ball_vx > 0.0
        {
            self.ball_x = right_paddle_x - PADDLE_WIDTH / 2.0 - half_ball;
            self.ball_vx = -self.ball_vx - BALL_SPEED_INCREMENT;
            let hit_pos = (self.ball_y - self.right_paddle_y) / half_paddle;
            self.ball_vy = hit_pos * BALL_SPEED;
        }

        if self.ball_x < -BALL_SIZE {
            self.right_score += 1;
            if self.right_score >= WIN_SCORE {
                self.game_over = true;
                self.winner = "Right Player";
            } else {
                self.reset_ball();
            }
        }
        if self.ball_x > self.gl.width() as f32 + BALL_SIZE {
            self.left_score += 1;
            if self.left_score >= WIN_SCORE {
                self.game_over = true;
                self.winner = "Left Player";
            } else {
                self.reset_ball();
            }
        }
    }

    fn render(&mut self, _alpha: f32) {
        self.gl.resize();
        let width = self.gl.width() as f32;
        let height = self.gl.height() as f32;
        self.camera.set_viewport(width, height);

        self.gl.clear(0.1, 0.1, 0.15, 1.0);
        self.gl.enable_blend();

        let vp = self.camera.view_projection();

        self.shapes.begin();

        self.shapes.set_color(Color::new(0.3, 0.3, 0.35, 1.0));
        let dash_height = 15.0;
        let dash_gap = 10.0;
        let mut y = 0.0;
        while y < height {
            self.shapes
                .draw_rect(width / 2.0 - 2.0, y, 4.0, dash_height);
            y += dash_height + dash_gap;
        }

        self.shapes.set_color(Color::WHITE);
        self.shapes.draw_rect(
            PADDLE_WIDTH / 2.0,
            self.left_paddle_y - PADDLE_HEIGHT / 2.0,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
        );

        self.shapes.draw_rect(
            width - PADDLE_MARGIN - PADDLE_WIDTH / 2.0,
            self.right_paddle_y - PADDLE_HEIGHT / 2.0,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
        );

        self.shapes.set_color(Color::new(1.0, 0.85, 0.2, 1.0));
        self.shapes.draw_rect(
            self.ball_x - BALL_SIZE / 2.0,
            self.ball_y - BALL_SIZE / 2.0,
            BALL_SIZE,
            BALL_SIZE,
        );

        self.shapes.flush(self.gl.gl(), &vp);
    }
}

fn rand_bool() -> bool {
    js_sys::Math::random() > 0.5
}

#[wasm_bindgen(start)]
pub fn main() {
    let mut game = PongGame::new().expect("Failed to create Pong game");
    game.time.init();

    let game = Rc::new(RefCell::new(game));
    let f: AnimationFrameClosure = Rc::new(RefCell::new(None));
    let g = f.clone();
    let game_clone = game.clone();

    let mut last_time = 0.0_f64;

    *g.borrow_mut() = Some(Closure::new(move |timestamp: f64| {
        let dt = if last_time == 0.0 {
            1.0 / 60.0
        } else {
            ((timestamp - last_time) / 1000.0).min(0.05)
        };
        last_time = timestamp;

        let mut game = game_clone.borrow_mut();
        game.time.update();
        game.input.update();
        game.update(dt as f32);
        game.render(1.0);
        drop(game);

        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(g.borrow().as_ref().unwrap());
}

fn request_animation_frame(f: &Closure<dyn FnMut(f64)>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap();
}
