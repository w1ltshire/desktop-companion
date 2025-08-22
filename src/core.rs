use std::{collections::HashMap, env::current_dir, fs, time::Instant};

use ggez::{
    Context, GameError, GameResult,
    event::{EventHandler, MouseButton},
    graphics::{self, Color, Image},
    winit::dpi::{LogicalPosition, PhysicalSize},
};

use log::debug;
use rand::Rng;

use crate::{
    animation::{
        AnimationTrait, CompanionAnimations,
        idle::IdleAnimation,
        movement::{Direction, MoveAnimation},
    },
    behavior::{Behavior, BehaviorManager},
    companion::{Companion, CompanionConfig},
};

pub struct CompanionApp {
    pub companion_data: Companion,
    pub animations: CompanionAnimations,
    pub behavior: BehaviorManager,
    pub monitor_size: PhysicalSize<u32>,
    frames: HashMap<String, Vec<Image>>,
    initialized: bool,
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
        let monitor_size = ctx
            .gfx
            .window()
            .current_monitor()
            .expect("Failed to get current monitor")
            .size();

        for (behavior, frames) in &companion_config.animations {
            let images: Vec<Image> = frames
                .iter()
                .map(|f| {
                    let path = current_dir()
                        .unwrap()
                        .join("config")
                        .join(&companion_data.name)
                        .join(&f.path);

                    debug!("{:?}", path);
                    read_image(ctx, path.to_str().unwrap()).unwrap()
                })
                .collect();
            frames_map.insert(behavior.clone(), images);
        }

        let mut animations = CompanionAnimations::new();
        animations.push(
            Box::new(IdleAnimation {
                sprite_frames: frames_map["idle"].clone(),
            }),
            "idle".into(),
        );

        CompanionApp {
            companion_data,
            animations,
            behavior: BehaviorManager::new(),
            monitor_size,
            frames: frames_map,
            initialized: false,
        }
    }

    fn initialize(&mut self, ctx: &mut Context) -> GameResult {
        let window = ctx.gfx.window();
        if let Some(true) = window.is_visible() {
            self.move_window(ctx, (self.monitor_size.width as i32 / 2, 0));

            let fall_animation = MoveAnimation {
                start_pos: (self.monitor_size.width as f32 / 2.0, -50.0),
                end: (
                    self.monitor_size.width as f32 / 2.0,
                    self.monitor_size.height as f32 - self.companion_data.height,
                ),
                duration: 0.6,
                start_time: Instant::now(),
                finished: false,
                current_pos: (0.0, 0.0),
                sprite_frames: vec![self.frames["idle"][0].clone()],
                direction: Direction::Vertical,
            };

            self.start_animation(fall_animation, "fall", ctx);
            self.initialized = true;
        }
        Ok(())
    }

    fn start_behavior(&mut self, ctx: &mut Context, behavior: Behavior) -> GameResult {
        let cur_pos = ctx
            .gfx
            .window_position()
            .expect("Failed to get window position");

        let mut rng = rand::rng();

        match behavior {
            Behavior::Idle => {
                self.animations.push(
                    Box::new(IdleAnimation {
                        sprite_frames: self.frames["idle"].clone(),
                    }),
                    "idle".into(),
                );
            }
            Behavior::WalkLeft | Behavior::WalkRight => {
                let cur_x = cur_pos.x as f32;

                let max_step = 200.0;

                let target_x = match behavior {
                    Behavior::WalkLeft => {
                        (cur_x - rng.random_range(50.0..max_step)) // step left
                            .max(0.0)
                    }
                    Behavior::WalkRight => {
                        (cur_x + rng.random_range(50.0..max_step))
                            .min(self.monitor_size.width as f32 - self.companion_data.width) // step
                                                                                             // riiiiight
                    }
                    _ => cur_x,
                };
                let duration = if self.companion_data.walkspeed > 0.0 {
                    ((target_x - cur_x).abs() / 50.0).max(0.1) // here we use .abs() to get modulus
                                                               // of distance because it can be
                                                               // negative if we're walking left 
                } else {
                    0.5
                };

                debug!(
                    "max_step {max_step} target_x {target_x} cur_x {cur_x} step {} duration {}",
                    cur_x - target_x,
                    duration
                );

                let walk_animation = MoveAnimation {
                    start_pos: (cur_pos.x as f32, cur_pos.y as f32),
                    end: (target_x, cur_pos.y as f32),
                    duration,
                    start_time: Instant::now(),
                    finished: false,
                    current_pos: (cur_pos.x as f32, cur_pos.y as f32),
                    sprite_frames: self.frames["walk"].clone(),
                    direction: if behavior == Behavior::WalkLeft {
                        Direction::Left
                    } else {
                        Direction::Right
                    },
                };

                self.start_animation(walk_animation, "walk", ctx);
            }
            Behavior::Fall | Behavior::Jump => {
                // TODO: implement
            }
        }

        Ok(())
    }

    fn move_window(&mut self, ctx: &mut Context, pos: (i32, i32)) {
        let window = ctx.gfx.window();
        window.set_outer_position(LogicalPosition::new(pos.0, pos.1));
    }

    fn start_animation(&mut self, animation: MoveAnimation, name: &str, ctx: &mut Context) {
        let mut anim = Box::new(animation);
        anim.start();
        self.animations.push(anim, name.to_string());
        self.animations.start(name, ctx);
    }
}

impl EventHandler for CompanionApp {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if !self.initialized {
            self.initialize(ctx)?;
        } else if self.animations.active.is_none() {
            if let Some(behavior) = self.behavior.update() {
                self.start_behavior(ctx, behavior)?;
            }
        }

        self.animations.update(ctx);
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) -> Result<(), GameError> {
        debug!("mouse down {:#?}", self.animations.active);
        let cur_pos = ctx
            .gfx
            .window_position()
            .expect("Failed to get window position");
        let walk_animation = MoveAnimation {
            start_pos: (cur_pos.x as f32, cur_pos.y as f32),
            end: (100.0, cur_pos.y as f32),
            duration: 4.0,
            start_time: Instant::now(),
            finished: false,
            current_pos: cur_pos.into(),
            sprite_frames: self.frames["walk"].clone(),
            direction: Direction::Left,
        };
        self.start_animation(walk_animation, "walk", ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::new(0.0, 0.0, 0.0, 0.0));
        self.animations.draw(ctx, &mut canvas);
        canvas.finish(ctx)
    }
}
