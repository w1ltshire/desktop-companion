use std::fs;

use ggez::{
    event::{EventHandler, MouseButton}, glam, graphics::{self, Color, DrawParam, Image}, Context, GameError, GameResult
};
use log::debug;

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
        debug!("mouse down");
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::new(0.0, 0.0, 0.0, 0.0));
        let sprite_bytes = fs::read(
            format!("{}/config/{}/idle.png", std::env::current_dir().unwrap().to_str().unwrap(), self.companion.path)
        ).unwrap();
        let image = Image::from_bytes(ctx, &sprite_bytes);

        canvas.draw(
            &image.unwrap(),
            DrawParam::default().dest(glam::vec2(0.0, 0.0)),
        );
        canvas.finish(ctx)
    }
}
