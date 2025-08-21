use ggez::{
    Context, GameError, GameResult,
    event::{EventHandler, MouseButton},
    glam,
    graphics::{self, Color, DrawParam},
};

use crate::companion::Companion;

pub struct CompanionApp {
    pub companion: Companion
}

impl CompanionApp {
    pub fn new(_ctx: &mut Context, companion: Companion) -> CompanionApp {
        CompanionApp {
            companion
        }
    }
}

impl EventHandler for CompanionApp {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) -> Result<(), GameError> {
        println!("mouse down");
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::new(0.0, 0.0, 0.0, 0.0));
        let test = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect {
                x: 0.0,
                y: 0.0,
                w: 10.0,
                h: 10.0,
            },
            graphics::Color::WHITE,
        );
        canvas.draw(
            &test.unwrap(),
            DrawParam::default().dest(glam::vec2(0.0, 0.0)),
        );
        canvas.finish(ctx)
    }
}
