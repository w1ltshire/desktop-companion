use std::{collections::HashMap, env::current_dir, fs};

use ggez::{
    Context, GameError, GameResult,
    event::{EventHandler, MouseButton},
    glam,
    graphics::{self, Canvas, Color, DrawParam, Image}
};

use log::{debug, error, trace};
use rand::{rng, Rng};

use crate::{animation::{fall::FallAnimation, CompanionAnimations}, companion::{Companion, CompanionConfig}};

pub struct CompanionApp {
    pub companion_data: Companion,
    animations: CompanionAnimations,
    frames: HashMap<String, Vec<Image>>,
    initialized: bool
}

fn read_image(ctx: &mut Context, path: &str) -> Result<Image, GameError> {
    let sprite_bytes = fs::read(path).unwrap_or_else(|_| panic!("Failed to read file {}", &path));
    Image::from_bytes(ctx, &sprite_bytes)
}

impl CompanionApp {
    pub fn new(
        ctx: &mut Context,
        companion_data: Companion,
        companion_config: CompanionConfig,
    ) -> CompanionApp {
        let mut frames_map = HashMap::new();
        let animations = CompanionAnimations::new();

        for (behavior, frames) in &companion_config.animations {
            let images: Vec<Image> = frames
                .iter()
                .map(|f| {
                    let path = current_dir().unwrap()
                        .join("config")
                        .join(&companion_data.name)
                        .join(&f.path);

                    debug!("{:?}", path);
                    read_image(ctx, path.to_str().unwrap()).unwrap()
                })
                .collect();
            frames_map.insert(behavior.clone(), images);
        }

        CompanionApp {
            companion_data,
            animations,
            frames: frames_map,
            initialized: false
        }
    }

    fn draw_sprite(
        &mut self,
        sprite: &str,
        canvas: &mut Canvas,
    ) -> Result<(), GameError> {
        trace!("looking for sprite {sprite} in frames");

        let image = &self.frames[sprite][0];
        canvas.draw(
            image,
            DrawParam::default().dest(glam::vec2(0.0, 0.0)),
        );
        Ok(())
    }

    fn start_fall(&mut self, ctx: &mut Context) {
        if let Some(monitor) = ctx.gfx.window().current_monitor() {
            let dimensions = monitor.size();
            let rand_x = rng().random_range(self.companion_data.width..dimensions.width as f32 - self.companion_data.width) as i32;
            let start = (rand_x, -(self.companion_data.height as i32)); 
            let end = (rand_x, (dimensions.height - self.companion_data.height as u32) as i32);

            let fall = FallAnimation::new(start, end, 0.5);
            self.animations.push(Box::new(fall), "fall".into());
            self.animations.start("fall", ctx);
                            self.animations.update(ctx);

        }
    }
}

impl EventHandler for CompanionApp {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if !self.initialized {
            let window = ctx.gfx.window();
            if let Some(true) = window.is_visible() {
                self.start_fall(ctx);
                self.initialized = true;
            }
        }

        self.animations.update(ctx);
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

        match self.draw_sprite("idle", &mut canvas) {
            Ok(_) => {}
            Err(e) => error!("Failed to draw_sprite: {e}"),
        }

        canvas.finish(ctx)
    }
}
