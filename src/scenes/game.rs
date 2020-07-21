use crate::{Context, Scene, KeyCode, World, collidable::Collidable};

pub struct GameScene {

}

impl GameScene {
    pub fn new(_ctx: &mut Context,) -> Self {
        Self {}
    }
}

impl Scene<World> for GameScene {
    fn update(
        &mut self, 
        ctx: &mut Context, 
        world: &mut World
    ) -> Option<Box<dyn Scene<World>>> { 

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

        None
    }

    fn draw(
        &mut self, 
        ctx: &mut Context, 
        world: &mut World
    ) {
        world.player_1.draw(ctx);
        world.player_2.draw(ctx);
        world.ball.draw(ctx);
    }

    fn input(
        &mut self, 
        _world: &mut World, 
        _keycode: KeyCode, 
        _pressed: bool, 
        _repeat: bool
    ) {

    }

    fn name(&self) -> &str {
        "Game Scene"
    }
}