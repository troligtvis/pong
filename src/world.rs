use crate::{
    graphics, na,
    player::{Player, PADDLE_WIDTH_HALF},
    Ball, Context, Controls, KeyCode,
};

pub const PADDING: f32 = 40.;

pub struct World {
    pub player_1: Player,
    pub player_2: Player,
    pub ball: Ball,
    max_score: i32,
    dt: f32,
}

impl World {
    pub fn new(ctx: &mut Context, max_score: i32) -> Self {
        let (scr_width, scr_height) = graphics::drawable_size(ctx);
        let (scr_width_half, screen_height_half) = (scr_width * 0.5, scr_height * 0.5);

        // Setup player 1
        let player_1 = Player::new(
            ctx,
            Controls::new(KeyCode::W, KeyCode::S),
            na::Vector2::new(PADDLE_WIDTH_HALF + PADDING, scr_height * 0.5),
            String::from("Player 1"),
        );

        // Setup player 2
        let player_2 = Player::new(
            ctx,
            Controls::new(KeyCode::Up, KeyCode::Down),
            na::Vector2::new(scr_width - PADDLE_WIDTH_HALF - PADDING, screen_height_half),
            String::from("Player 2"),
        );

        // Setup ball
        let ball = Ball::new(ctx, scr_width_half, screen_height_half);

        Self {
            player_1,
            player_2,
            ball,
            max_score,
            dt: 0.,
        }
    }

    pub fn get_dt(&self) -> f32 {
        self.dt
    }

    pub fn update_delta_time(&mut self, new_value: f32) {
        self.dt = new_value;
    }

    pub fn check_score(&mut self, ctx: &mut Context) -> Option<&str> {
        let scr_width = graphics::drawable_size(ctx).0;

        // Check which side scored
        if self.ball.position.x < 0.0 {
            self.player_2.increment_score();
            self.ball.reset(ctx);
        }
        if self.ball.position.x > scr_width {
            self.player_1.increment_score();
            self.ball.reset(ctx);
        }

        if self.player_1.get_score() >= self.max_score {
            self.reset(ctx);
            return Some(self.player_1.get_name());
        }

        if self.player_2.get_score() >= self.max_score {
            self.reset(ctx);
            return Some(self.player_2.get_name());
        }

        None
    }

    // Reset both player 1 and player 2 to origin position and 0 score
    // FIXME: - remove duplicate code for init players
    pub fn reset(&mut self, ctx: &mut Context) {
        let (scr_width, scr_height) = graphics::drawable_size(ctx);
        let screen_height_half = scr_height * 0.5;

        // Setup player 1
        let player_1 = Player::new(
            ctx,
            Controls::new(KeyCode::W, KeyCode::S),
            na::Vector2::new(PADDLE_WIDTH_HALF + PADDING, scr_height * 0.5),
            String::from("Player 1"),
        );

        // Setup player 2
        let player_2 = Player::new(
            ctx,
            Controls::new(KeyCode::Up, KeyCode::Down),
            na::Vector2::new(scr_width - PADDLE_WIDTH_HALF - PADDING, screen_height_half),
            String::from("Player 2"),
        );

        self.player_1 = player_1;
        self.player_2 = player_2;
    }
}
