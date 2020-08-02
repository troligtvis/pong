use crate::{
    game::GameScene as gs, graphics, menu::MenuScene as ms, na, Context, KeyCode, Scene, World,
};

pub struct EndScene {
    title_text: graphics::Text,

    is_done: bool,
    is_retry: bool,

    selected_item_index: i32,
    menu_items: Vec<graphics::Text>,

    silver_color: graphics::Color,
}

impl EndScene {
    pub fn new(ctx: &mut Context, winner: String) -> Self {
        let silver = graphics::Color::from_rgba(192, 192, 192, 255);

        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf").unwrap();
        let winner_text: String = format!("{} is the winner!", winner);
        let title_text_fragment = graphics::TextFragment::new(winner_text)
            .color(graphics::Color::from((192, 128, 64, 255)))
            .font(font)
            .scale(graphics::Scale::uniform(56.0));

        let items: Vec<graphics::Text> = ["Retry", "Main Menu"]
            .iter()
            .map(|title| {
                graphics::Text::new(
                    graphics::TextFragment::new(*title)
                        .color(silver)
                        .font(font)
                        .scale(graphics::Scale::uniform(36.0)),
                )
            })
            .collect();

        Self {
            title_text: graphics::Text::new(title_text_fragment),
            is_done: false,
            is_retry: false,
            selected_item_index: 0,
            menu_items: items,
            silver_color: silver,
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

        for (index, item) in self.menu_items.iter_mut().enumerate() {
            let height = item.height(ctx) as f32;

            let dest = na::Point2::new(
                (scr_width as f32 / 2.0) - (title_text_width as f32 / 2.0),
                (scr_height as f32 / 2.0)
                    + ((title_text_height + 20.0) - (height + 20.0))
                    + (index as f32 * 50.),
            );

            if index as i32 == self.selected_item_index {
                item.fragments_mut()[0].color = Some(graphics::WHITE);
            } else {
                item.fragments_mut()[0].color = Some(self.silver_color);
            }

            graphics::draw(ctx, item, graphics::DrawParam::new().dest(dest)).unwrap();
        }

        graphics::draw(
            ctx,
            &self.title_text,
            graphics::DrawParam::new().dest(title_text_dest),
        )
        .unwrap();
    }

    fn input(
        &mut self,
        _ctx: &mut Context,
        _world: &mut World,
        keycode: KeyCode,
        _pressed: bool,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Space | KeyCode::Return | KeyCode::D | KeyCode::Right => {
                if self.selected_item_index == 0 {
                    self.is_retry = true;
                } else {
                    self.is_done = true;
                }
            }
            KeyCode::Up | KeyCode::W => {
                let item_count = self.menu_items.len() as i32;
                self.selected_item_index = if self.selected_item_index == 0 {
                    item_count - 1
                } else {
                    (self.selected_item_index - 1) % item_count
                };
            }
            KeyCode::Down | KeyCode::S => {
                let item_count = self.menu_items.len() as i32;
                self.selected_item_index = (self.selected_item_index + 1) % item_count;
            }
            _ => {}
        };
    }

    fn name(&self) -> &str {
        "End Scene"
    }
}
