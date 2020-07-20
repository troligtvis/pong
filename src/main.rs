use ggez;
use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::event;
use ggez::input::keyboard::{self, KeyCode};

mod constants;
mod util;
mod player;
mod ball;
mod collidable;

use collidable::Collidable;
use constants::*;
use player::{Player, Paddle, Controls};
use ball::Ball;
use util::Util;

pub trait Scene<W> {
    fn update(&mut self, ctx: &mut Context, world: &mut W) -> Option<Box<dyn Scene<W>>>;
    fn draw(&mut self, ctx: &mut Context, world: &mut W);
    fn input(&mut self, world: &mut W, keycode: KeyCode, pressed: bool, repeat: bool);
    fn name(&self) -> &str;
}

struct MainState {
    player_1: Player,
    player_2: Player,
    ball: Ball,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> Self {
        let (scr_w, screen_h) = Util::get_bounds(ctx);
        let (scr_w_half, screen_h_half) = (scr_w * 0.5, screen_h * 0.5);

        // Setup player 1
        let player_1 = Player::new(
            Controls::new(KeyCode::W, KeyCode::S),
            Paddle::new(ctx, PADDLE_WIDTH_HALF + PADDING, screen_h_half)
        );

        // Setup player 2
        let player_2 = Player::new(
            Controls::new(KeyCode::Up, KeyCode::Down),
            Paddle::new(ctx, scr_w - PADDLE_WIDTH_HALF - PADDING, screen_h_half),
        );

        // Setup ball
        let ball = Ball::new(ctx, scr_w_half, screen_h_half);

        MainState {
            player_1,
            player_2,
            ball,
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ggez::timer::delta(ctx).as_secs_f32();

        self.player_1.update(ctx, dt)?;
        self.player_2.update(ctx, dt)?;

        self.ball.update(ctx, dt)?;

        // Collision check
        if self.player_1.paddle.check_collision(&self.ball) {
            self.ball.velocity.x = self.ball.velocity.x.abs();
        }

        if self.player_2.paddle.check_collision(&self.ball) {
            self.ball.velocity.x = -self.ball.velocity.x.abs();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        self.player_1.draw(ctx)?;
        self.player_2.draw(ctx)?;

        self.ball.draw(ctx)?;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Pong", "Troligtvis");
    let (ctx, event_loop) = &mut cb.build()?;
    graphics::set_window_title(ctx, "PONG");

    let mut state = MainState::new(ctx);
    event::run(ctx, event_loop, &mut state)
}
