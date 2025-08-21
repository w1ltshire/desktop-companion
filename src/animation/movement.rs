use std::time::Instant;

use ggez::{
    Context, glam,
    graphics::{Canvas, DrawParam, Image},
    winit::dpi::LogicalPosition,
};

use crate::animation::AnimationTrait;

pub struct MoveAnimation {
    pub start_pos: (f32, f32),
    pub end: (f32, f32),
    pub duration: f32,
    pub start_time: Instant,
    pub finished: bool,
    pub sprite_frames: Vec<Image>, // multiple sprites for walking
    pub current_pos: (f32, f32),
}

impl AnimationTrait for MoveAnimation {
    fn start(&mut self) {
        self.start_time = Instant::now();
        self.finished = false;
        self.current_pos = self.start_pos;
    }

    fn update(&mut self, ctx: &mut Context) {
        let elapsed = self.start_time.elapsed().as_secs_f32();
        let t = (elapsed / self.duration).min(1.0);

        self.current_pos = (
            self.start_pos.0 + (self.end.0 - self.start_pos.0) * t,
            self.start_pos.1 + (self.end.1 - self.start_pos.1) * t,
        );

        ctx.gfx
            .window()
            .set_outer_position(LogicalPosition::new(self.current_pos.0, self.current_pos.1));

        if t >= 1.0 {
            self.finished = true;
        }
    }

    fn draw(&self, canvas: &mut Canvas) {
        let frame_index =
            ((self.start_time.elapsed().as_secs_f32() * 10.0) as usize) % self.sprite_frames.len();
        let sprite = &self.sprite_frames[frame_index];

        canvas.draw(
            sprite,
            DrawParam::default().dest(glam::vec2(self.current_pos.0, self.current_pos.1)),
        );
    }

    fn is_finished(&self) -> bool {
        self.finished
    }
}

impl MoveAnimation {}
