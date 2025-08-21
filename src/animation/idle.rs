use ggez::{
    Context, glam,
    graphics::{Canvas, DrawParam, Image},
};

use crate::animation::AnimationTrait;

pub struct IdleAnimation {
    pub sprite_frames: Vec<Image>,
}

impl AnimationTrait for IdleAnimation {
    fn start(&mut self) {}

    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&self, canvas: &mut Canvas) {
        canvas.draw(
            &self.sprite_frames[0],
            DrawParam::default().dest(glam::vec2(0.0, 0.0)),
        );
    }

    fn is_finished(&self) -> bool {
        true
    }
}
