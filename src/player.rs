use crate::{
    collidable::Collidable, constants::*, graphics, keyboard, na, util::Util, Context, KeyCode,
};

pub struct Controls {
    up_key: KeyCode,
    down_key: KeyCode,
}

impl Controls {
    pub fn new(up_key: KeyCode, down_key: KeyCode) -> Self {
        Controls { up_key, down_key }
    }
}

enum Direction {
    Up,
    Down,
}

pub struct Player {
    score: i32,
    controls: Controls,
    pub paddle: Paddle,
    name: String,
}

impl Player {
    pub fn new(controls: Controls, paddle: Paddle, name: String) -> Self {
        Self {
            score: 0,
            controls,
            paddle,
            name,
        }
    }

    pub fn update(&mut self, ctx: &mut Context, dt: f32) {
        if keyboard::is_key_pressed(ctx, self.controls.up_key) {
            self.paddle.move_direction(ctx, Direction::Up, dt);
        }

        if keyboard::is_key_pressed(ctx, self.controls.down_key) {
            self.paddle.move_direction(ctx, Direction::Down, dt);
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        let mut draw_param = graphics::DrawParam::default();
        draw_param.dest = self.paddle.position.into();

        graphics::draw(ctx, self.paddle.get_mesh(), draw_param).unwrap();
    }

    pub fn increment_score(&mut self) {
        self.score += 1;
    }

    pub fn get_score(&self) -> i32 {
        self.score
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
}

pub struct Paddle {
    pub position: na::Point2<f32>,
    mesh: graphics::Mesh,
}

impl Paddle {
    pub fn new(ctx: &mut Context, x: f32, y: f32) -> Self {
        let rect = graphics::Rect::new(
            -PADDLE_WIDTH_HALF,
            -PADDLE_HEIGHT_HALF,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
        );
        let mesh =
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::WHITE)
                .unwrap();

        Self {
            position: na::Point2::new(x, y),
            mesh,
        }
    }

    fn move_direction(&mut self, ctx: &mut Context, direction: Direction, dt: f32) {
        match direction {
            Direction::Up => self.position.y -= PADDLE_SPEED * dt,
            Direction::Down => self.position.y += PADDLE_SPEED * dt,
        };

        let scr_height = graphics::drawable_size(ctx).1;
        Util::clamp(
            &mut self.position.y,
            PADDLE_HEIGHT_HALF,
            scr_height - PADDLE_HEIGHT_HALF,
        );
    }

    pub fn get_mesh(&self) -> &graphics::Mesh {
        &self.mesh
    }
}

impl Collidable for Paddle {
    fn get_position(&self) -> na::Point2<f32> {
        self.position
    }

    fn get_size(&self) -> (f32, f32) {
        (PADDLE_WIDTH, PADDLE_HEIGHT)
    }
}
