use crate::{collidable::Collidable, graphics, keyboard, na, util::Util, Context, KeyCode};

// Paddle data
const PADDLE_HEIGHT: f32 = 100.0;
const PADDLE_WIDTH: f32 = 20.0;
const PADDLE_HEIGHT_HALF: f32 = PADDLE_HEIGHT * 0.5;
pub const PADDLE_WIDTH_HALF: f32 = PADDLE_WIDTH * 0.5;
const PADDLE_SPEED: f32 = 600.0;

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
    pub fn new(
        ctx: &mut Context,
        controls: Controls,
        position: na::Vector2<f32>,
        name: String,
    ) -> Self {
        let rect = graphics::Rect::new(
            -PADDLE_WIDTH_HALF,
            -PADDLE_HEIGHT_HALF,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
        );

        let paddle = Paddle::new(ctx, position, rect, PADDLE_SPEED);

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
        draw_param.dest = self.paddle.get_position().into();

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
    position: na::Point2<f32>,
    pub size: (f32, f32),
    mesh: graphics::Mesh,
    speed: f32,
}

impl Paddle {
    pub fn new(
        ctx: &mut Context,
        coord: na::Vector2<f32>,
        rect: graphics::Rect,
        speed: f32,
    ) -> Self {
        let mesh =
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::WHITE)
                .unwrap();

        Self {
            position: na::Point2::new(coord.x, coord.y),
            size: (rect.w, rect.h),
            mesh,
            speed,
        }
    }

    fn move_direction(&mut self, ctx: &mut Context, direction: Direction, dt: f32) {
        match direction {
            Direction::Up => self.position.y -= self.speed * dt,
            Direction::Down => self.position.y += self.speed * dt,
        };

        let scr_height = graphics::drawable_size(ctx).1;
        let height_half = self.size.1 * 0.5;
        Util::clamp(&mut self.position.y, height_half, scr_height - height_half);
    }

    pub fn get_mesh(&self) -> &graphics::Mesh {
        &self.mesh
    }

    pub fn set_position(&mut self, position: na::Point2<f32>) {
        self.position = position;
    }

    pub fn get_position(&self) -> na::Point2<f32> {
        self.position
    }
}

impl Collidable for Paddle {
    fn get_position(&self) -> na::Point2<f32> {
        self.position
    }

    fn get_size(&self) -> (f32, f32) {
        self.size
    }
}
