use std::collections::hash_map::HashMap;

use ggez::{Context, graphics::Canvas};

pub mod idle;
pub mod movement;

pub struct CompanionAnimations {
    pub animations: HashMap<String, Box<dyn AnimationTrait>>,
    pub active: Option<String>,
}

pub trait AnimationTrait {
    fn start(&mut self);
    fn update(&mut self, ctx: &mut Context);
    fn draw(&self, canvas: &mut Canvas);
    fn is_finished(&self) -> bool;
}

impl CompanionAnimations {
    pub fn new() -> Self {
        CompanionAnimations {
            animations: HashMap::new(),
            active: None,
        }
    }

    pub fn push(&mut self, animation: Box<dyn AnimationTrait>, name: String) {
        self.animations.insert(name, animation);
    }

    pub fn start(&mut self, name: &str, _ctx: &mut Context) {
        if let Some(anim) = self.animations.get_mut(name) {
            anim.start();
            self.active = Some(name.to_string());
        }
    }

    pub fn update(&mut self, ctx: &mut Context) {
        if let Some(active_name) = &self.active {
            let finished = if let Some(anim) = self.animations.get_mut(active_name) {
                anim.update(ctx);
                anim.is_finished()
            } else {
                false
            };

            if finished {
                self.active = None;
            }
        }
    }

    pub fn draw(&self, _ctx: &mut Context, canvas: &mut Canvas) {
        if let Some(active_name) = &self.active {
            if let Some(anim) = self.animations.get(active_name) {
                anim.draw(canvas);
                return;
            }
        }

        // fallback to idle
        if let Some(idle_anim) = self.animations.get("idle") {
            idle_anim.draw(canvas);
        }
    }
}
