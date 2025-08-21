use std::{collections::HashMap, env::current_dir, fs, path::PathBuf, thread::current};

use ggez::{
    Context, GameError, GameResult,
    event::{EventHandler, MouseButton},
    glam,
    graphics::{self, Canvas, Color, DrawParam, Image},
    winit::dpi::LogicalPosition,
};

use log::{debug, error, trace};

use std::time::Instant;

use crate::companion::{Companion, CompanionConfig};

pub struct CompanionApp {
    pub companion_data: Companion,
    initialized: bool,
    animating: bool,
    start_time: Option<Instant>,
    target_x: i32,
    start_y: i32,
    frames: HashMap<String, Vec<Image>>,
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
            initialized: false,
            animating: false,
            start_time: None,
            target_x: 0,
            start_y: 0,
            frames: frames_map,
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

    fn move_window(&mut self, ctx: &mut Context, pos: (i32, i32)) {
        let window = ctx.gfx.window();
        trace!("moving");
        window.set_outer_position(LogicalPosition::new(pos.0, pos.1));
    }

    fn start_window_animation(&mut self, ctx: &mut ggez::Context) {
        if !self.initialized {
            if let Some(true) = ctx.gfx.window().is_visible() {
                let window = ctx.gfx.window();
                if let Some(monitor) = window.current_monitor() {
                    let dimensions = monitor.size();
                    let mut rng = rand::rng();
                    let max_width = dimensions
                        .width
                        .saturating_sub(self.companion_data.width as u32);
                    let width = rand::Rng::random_range(&mut rng, 0..max_width) as i32;

                    self.target_x = width;
                    self.start_y = dimensions.height as i32;
                    self.start_time = Some(Instant::now());
                    self.animating = true;
                }
            }
            self.initialized = true;
        }
    }

    fn update_window_animation(&mut self, ctx: &mut ggez::Context) {
        if !self.animating {
            return;
        }
        if let Some(monitor) = ctx.gfx.window().current_monitor() {
            let dimensions = monitor.size();
            let elapsed = self.start_time.unwrap().elapsed().as_secs_f32();
            let t = (elapsed / 0.3).min(1.0);
            let target_y = dimensions.height - self.companion_data.height as u32;
            let start_y = -self.companion_data.height as i32;
            let height = ((1.0 - t) * start_y as f32 + t * target_y as f32) as i32;

            self.move_window(ctx, (self.target_x, height));

            if t >= 1.0 {
                self.animating = false;
            }
        }
    }
}

impl EventHandler for CompanionApp {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.start_window_animation(ctx);
        self.update_window_animation(ctx);
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
