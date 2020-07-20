use crate::{na, Context, GameResult, graphics, constants::*, util::Util, collidable::Collidable};

pub struct Ball {
    pub position: na::Point2<f32>,
    pub velocity: na::Vector2<f32>,
    mesh: graphics::Mesh,
}

impl Ball {

    pub fn new(ctx: &mut Context, x: f32, y: f32) -> Self {
        let rect = graphics::Rect::new(-BALL_SIZE_HALF, -BALL_SIZE_HALF, BALL_SIZE, BALL_SIZE);
        let mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::WHITE).unwrap();

        let mut velocity = na::Vector2::new(0., 0.);
        Util::randomize_vec(&mut velocity, BALL_SPEED, BALL_SPEED);
        
        Self {
            position: na::Point2::new(x, y),
            velocity,
            mesh,
        }
    }

    pub fn get_mesh(&self) -> &graphics::Mesh {
        &self.mesh
    }

    pub fn reset(&mut self, ctx: &mut Context) {
        let (scr_w, scr_h) = Util::get_bounds(ctx);
        self.position.x = scr_w * 0.5;
        self.position.y = scr_h * 0.5;
        Util::randomize_vec(&mut self.velocity, BALL_SPEED, BALL_SPEED);
    }

    pub fn update(&mut self, ctx: &mut Context, dt: f32) -> GameResult { 
        self.position += self.velocity * dt;

        let (scr_w, scr_h) = Util::get_bounds(ctx);

        if self.position.x < 0.0 || self.position.x > scr_w {
            self.reset(ctx);
        }

        // Ceiling bounce 
        if self.position.y < BALL_SIZE_HALF {
            self.position.y = BALL_SIZE_HALF;
            self.velocity.y = self.velocity.y.abs();
        } else if self.position.y > scr_h - BALL_SIZE_HALF {
            self.position.y = scr_h - BALL_SIZE_HALF;
            self.velocity.y = -self.velocity.y.abs();
        }

        Ok(())
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let mut draw_param = graphics::DrawParam::default();
        draw_param.dest = self.position.into(); 

        graphics::draw(ctx, self.get_mesh(), draw_param)
    }
}

impl Collidable for Ball {

    fn get_position(&self) -> na::Point2<f32> {
        self.position
    }

    fn get_size(&self) -> (f32, f32) {
        (BALL_SIZE, BALL_SIZE)
    }
}