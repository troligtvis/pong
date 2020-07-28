use crate::{game::GameScene as gs, graphics, na, Context, KeyCode, Paddle, Scene, World};

pub struct MenuScene {
    title_text: graphics::Text,

    is_done: bool,

    selected_item_index: i32,
    menu_items: Vec<graphics::Text>,

    silver_color: graphics::Color,
}

impl MenuScene {
    pub fn new(ctx: &mut Context) -> Self {
        let silver = graphics::Color::from_rgba(192, 192, 192, 255);

        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf").unwrap();
        let title_text_fragment = graphics::TextFragment::new("Pong-A-Long")
            .color(graphics::Color::from((192, 128, 64, 255)))
            .font(font)
            .scale(graphics::Scale::uniform(56.0));

        let items: Vec<graphics::Text> = ["Play", "Options", "About", "Exit"]
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
            selected_item_index: 0,
            menu_items: items,
            silver_color: silver,
        }
    }
}

impl Scene<World> for MenuScene {
    fn update(&mut self, ctx: &mut Context, _world: &mut World) -> Option<Box<dyn Scene<World>>> {
        if self.is_done {
            let next_scene = gs::new(ctx);
            Some(Box::new(next_scene))
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

        for (index, item) in self.menu_items.iter_mut().enumerate() {
            let width = item.width(ctx) as f32;
            let height = item.height(ctx) as f32;

            let dest = na::Point2::new(
                (scr_width as f32 / 2.0) - (title_text_width as f32 / 2.0),
                (scr_height as f32 / 2.0)
                    + ((title_text_height + 20.0) - (height + 20.0))
                    + (index as f32 * 50.),
            );

            if index as i32 == self.selected_item_index {
                item.fragments_mut()[0].color = Some(graphics::WHITE);

                let item_height_half = height * 0.5;
                let paddle_size = (6., 20.);

                let left_indicator_paddle = Paddle::new(
                    ctx,
                    dest.x - item_height_half,
                    dest.y + item_height_half - (paddle_size.1 * 0.5),
                    graphics::Rect::new(
                        dest.x - item_height_half,
                        dest.y + item_height_half - (paddle_size.1 * 0.5),
                        paddle_size.0,
                        paddle_size.1,
                    ),
                );
                let right_indicator_paddle = Paddle::new(
                    ctx,
                    dest.x + width + item_height_half - paddle_size.0,
                    dest.y + item_height_half - (paddle_size.1 * 0.5),
                    graphics::Rect::new(
                        dest.x + width + item_height_half - paddle_size.0,
                        dest.y + item_height_half - (paddle_size.1 * 0.5),
                        paddle_size.0,
                        paddle_size.1,
                    ),
                );

                graphics::draw(
                    ctx,
                    left_indicator_paddle.get_mesh(),
                    graphics::DrawParam::default(),
                )
                .unwrap();
                graphics::draw(
                    ctx,
                    right_indicator_paddle.get_mesh(),
                    graphics::DrawParam::default(),
                )
                .unwrap();
            } else {
                item.fragments_mut()[0].color = Some(self.silver_color);
            }

            graphics::draw(ctx, item, graphics::DrawParam::new().dest(dest)).unwrap();
        }
    }

    fn input(&mut self, _world: &mut World, keycode: KeyCode, _pressed: bool, _repeat: bool) {
        match keycode {
            KeyCode::Space | KeyCode::Return => {
                if self.selected_item_index == 0 {
                    self.is_done = true;
                }
            }
            KeyCode::Up => {
                let item_count = self.menu_items.len() as i32;
                self.selected_item_index = if self.selected_item_index == 0 {
                    item_count - 1
                } else {
                    (self.selected_item_index - 1) % item_count
                };
            }
            KeyCode::Down => {
                let item_count = self.menu_items.len() as i32;
                self.selected_item_index = (self.selected_item_index + 1) % item_count;
            }
            _ => {}
        };
    }

    fn name(&self) -> &str {
        "Menu Scene"
    }
}
