use crate::{Context, graphics, KeyCode, Player, Controls, Paddle, Ball, constants::*};

pub struct World {
    pub player_1: Player,
    pub player_2: Player,
    pub ball: Ball,
    left_score: i32,
    right_score: i32,
    dt: f32,
}

impl World {
    pub fn new(ctx: &mut Context) -> Self {

        let (scr_width, scr_height) = graphics::drawable_size(ctx);
        let (scr_width_half, screen_height_half) = (scr_width * 0.5, scr_height * 0.5);

        // Setup player 1
        let player_1 = Player::new(
            Controls::new(KeyCode::W, KeyCode::S),
            Paddle::new(ctx, PADDLE_WIDTH_HALF + PADDING, screen_height_half)
        );

        // Setup player 2
        let player_2 = Player::new(
            Controls::new(KeyCode::Up, KeyCode::Down),
            Paddle::new(ctx, scr_width - PADDLE_WIDTH_HALF - PADDING, screen_height_half),
        );

        // Setup ball
        let ball = Ball::new(ctx, scr_width_half, screen_height_half);
        
        Self {
            player_1,
            player_2,
            ball,
            left_score: 0,
            right_score: 0,
            dt: 0.,
        }
    }

    pub fn get_dt(&self) -> f32 {
        self.dt
    }

    pub fn update_delta_time(&mut self, new_value: f32) {
        self.dt = new_value;
    }
}