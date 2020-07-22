pub mod game;
pub mod menu;

use crate::{Context, KeyCode};

pub trait Scene<W> {
    fn update(&mut self, ctx: &mut Context, world: &mut W) -> Option<Box<dyn Scene<W>>>;
    fn draw(&mut self, ctx: &mut Context, world: &mut W);
    fn input(&mut self, world: &mut W, keycode: KeyCode, pressed: bool, repeat: bool);
    fn name(&self) -> &str;
}
