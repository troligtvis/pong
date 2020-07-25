use crate::{
    collidable::Collidable, constants, graphics, na, Context, DrawParam, KeyCode, Scene, Text,
    World,
};

pub struct GameScene {}

impl GameScene {
    pub fn new(_ctx: &mut Context) -> Self {
        Self {}
    }
}

impl Scene<World> for GameScene {
    fn update(&mut self, ctx: &mut Context, world: &mut World) -> Option<Box<dyn Scene<World>>> {
        let dt = world.get_dt();
        world.player_1.update(ctx, dt);
        world.player_2.update(ctx, dt);

        world.ball.update(ctx, dt);

        // Collision check
        if world.player_1.paddle.check_collision(&world.ball) {
            world.ball.velocity.x = world.ball.velocity.x.abs();
        }

        if world.player_2.paddle.check_collision(&world.ball) {
            world.ball.velocity.x = -world.ball.velocity.x.abs();
        }

        if let Some(winner) = world.check_score(ctx) {
            println!("{} is the winner!", winner);
            // todo: present win screen
        }

        None
    }

    fn draw(&mut self, ctx: &mut Context, world: &mut World) {
        world.player_1.draw(ctx);
        world.player_2.draw(ctx);
        world.ball.draw(ctx);

        // Draw score UI
        let score_text = Text::new(format!(
            "{}      {}",
            world.player_1.get_score(),
            world.player_2.get_score()
        ));

        let scr_width = graphics::drawable_size(ctx).0;
        let scr_width_half = scr_width * 0.5;
        let score_position = na::Point2::new(scr_width_half, constants::PADDING);

        let mut draw_param = DrawParam::default();
        draw_param.dest = score_position.into();
        graphics::draw(ctx, &score_text, draw_param).unwrap();
    }

    fn input(&mut self, _world: &mut World, _keycode: KeyCode, _pressed: bool, _repeat: bool) {}

    fn name(&self) -> &str {
        "Game Scene"
    }
}
