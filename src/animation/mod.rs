use std::collections::hash_map::HashMap;

use ggez::Context;

pub mod fall;

pub struct CompanionAnimations {
    animations: HashMap<String, Box<dyn AnimationTrait>>,
}

pub trait AnimationTrait {
    fn start(&mut self, ctx: &mut Context);
    fn update(&mut self, ctx: &mut Context);
}

impl CompanionAnimations {
    pub fn new() -> Self {
        CompanionAnimations {
            animations: HashMap::new() 
        }
    }

    pub fn push(&mut self, animation: Box<dyn AnimationTrait>, name: String) {
        self.animations.insert(name, animation);
    }

    pub fn start(&mut self, name: &str, ctx: &mut Context) {
        if let Some(anim) = self.animations.get_mut(name) {
            anim.start(ctx);
        }
    }

    pub fn update(&mut self, ctx: &mut Context) {
        for anim in self.animations.values_mut() {
            anim.update(ctx);
        }
    }
}
