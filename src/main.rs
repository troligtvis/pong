use std::{env, path};

use ggez;
use ggez::event;
use ggez::graphics;
use ggez::graphics::{DrawParam, Text};
use ggez::input::keyboard::{self, KeyCode, KeyMods};
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

mod ball;
mod collidable;
mod constants;
mod player;
mod scenes;
mod util;
mod world;

use ball::Ball;
use player::{Controls, Paddle, Player};
use scenes::*;
use world::World;

struct MainState {
    world: World,
    current_scene: Box<dyn Scene<World>>,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> Self {
        let initial_scene = Box::new(scenes::menu::MenuScene::new(ctx));

        let max_score = 5;
        let world = World::new(ctx, max_score);

        MainState {
            world,
            current_scene: initial_scene,
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ggez::timer::delta(ctx).as_secs_f32();
        self.world.update_delta_time(dt);

        if let Some(next_scene) = self.current_scene.update(ctx, &mut self.world) {
            self.current_scene = next_scene
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        self.current_scene.draw(ctx, &mut self.world);

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        repeat: bool,
    ) {
        if keycode == KeyCode::Escape {
            event::quit(_ctx)
        }

        self.current_scene
            .input(&mut self.world, keycode, true, repeat)
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        self.current_scene
            .input(&mut self.world, keycode, false, false);
    }
}

fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let c = ggez::conf::Conf::new();
    let (ref mut ctx, ref mut event_loop) = ggez::ContextBuilder::new("Pong", "Troligtvis")
        .add_resource_path(resource_dir)
        .conf(c)
        .build()
        .unwrap();

    graphics::set_window_title(ctx, "PONG");

    let mut state = MainState::new(ctx);
    event::run(ctx, event_loop, &mut state)
}
