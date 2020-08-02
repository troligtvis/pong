use crate::{
    game::GameScene as gs, graphics, menu::MenuScene as ms, na, Context, KeyCode, Scene, World,
};

pub struct EndScene {
    title_text: graphics::Text,

    is_done: bool,
    is_retry: bool,
}

impl EndScene {
    pub fn new(ctx: &mut Context, winner: String) -> Self {
        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf").unwrap();
        let winner_text: String = format!("{} is the winner!", winner);
        let title_text_fragment = graphics::TextFragment::new(winner_text)
            .color(graphics::Color::from((192, 128, 64, 255)))
            .font(font)
            .scale(graphics::Scale::uniform(56.0));

        Self {
            title_text: graphics::Text::new(title_text_fragment),
            is_done: false,
            is_retry: false,
        }
    }
}

impl Scene<World> for EndScene {
    fn update(&mut self, ctx: &mut Context, _world: &mut World) -> Option<Box<dyn Scene<World>>> {
        if self.is_done {
            Some(Box::new(ms::new(ctx)))
        } else if self.is_retry {
            Some(Box::new(gs::new(ctx)))
        } else {
            None
        }
    }

    fn draw(&mut self, ctx: &mut Context, _world: &mut World) {
        let (scr_width, scr_height) = graphics::drawable_size(ctx);

        let title_text_width = self.title_text.width(ctx) as f32;
        let title_text_height = self.title_text.height(ctx) as f32;

        let title_text_dest = na::Point2::new(
            (scr_width as f32 / 2.0) - (title_text_width / 2.0),
            (scr_height as f32 / 2.0) - (title_text_height + 20.0),
        );

        graphics::draw(
            ctx,
            &self.title_text,
            graphics::DrawParam::new().dest(title_text_dest),
        )
        .unwrap();
    }

        if keycode == KeyCode::Space {
            self.is_done = true;
        }
    fn input(&mut self, _ctx: &mut Context, _world: &mut World, keycode: KeyCode, _pressed: bool, _repeat: bool) {
    }

    fn name(&self) -> &str {
        "End Scene"
    }
}
