use std::fs;

use ggez::{
    event::{EventHandler, MouseButton}, glam, graphics::{self, Canvas, Color, DrawParam, Image}, Context, GameError, GameResult
};
use log::{debug, error};

use crate::companion::{Companion, CompanionConfig, Sprite};

pub struct CompanionApp {
    pub companion_data: Companion,
    pub companion_config: CompanionConfig,
}

impl CompanionApp {
    pub fn new(
        _ctx: &mut Context,
        companion_data: Companion,
        companion_config: CompanionConfig,
    ) -> CompanionApp {
        CompanionApp {
            companion_data,
            companion_config,
        }
    }

    fn read_image(&mut self, ctx: &mut Context, path: String) -> Result<Image, GameError> {
        let sprite_bytes = fs::read(&path).unwrap_or_else(|_| panic!("Failed to read file {}", &path));
        Image::from_bytes(ctx, &sprite_bytes)
    }

    fn draw_sprite(&mut self, sprite: &str, ctx: &mut Context, canvas: &mut Canvas) -> Result<(), GameError> {
        let path = format!(
            "{}/config/{}/{}",
            std::env::current_dir().unwrap().to_str().unwrap(),
            self.companion_data.path,
            sprite
        );
        let image = self.read_image(ctx, path);
        canvas.draw(
            &image.unwrap(),
            DrawParam::default().dest(glam::vec2(0.0, 0.0)),
        );
        Ok(())
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
        match self.draw_sprite("idle1.png", ctx, &mut canvas) {
            Ok(_) => {},
            Err(e) => error!("Failed to draw_sprite: {e}"),
        }
        canvas.finish(ctx)
    }
}
