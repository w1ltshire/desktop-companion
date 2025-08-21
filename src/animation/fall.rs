use std::time::Instant;

use ggez::Context;

use crate::animation::AnimationTrait;

pub struct FallAnimation {
    pub start_pos: (i32, i32),
    pub end_pos: (i32, i32),
    pub duration: f32,
    pub start_time: Option<Instant>,
    pub finished: bool,
}

impl FallAnimation {
    pub fn new(start: (i32, i32), end: (i32, i32), duration: f32) -> Self {
        Self {
            start_pos: start,
            end_pos: end,
            duration,
            start_time: None,
            finished: false,
        }
    }

    fn move_window(&self, ctx: &mut Context, pos: (f32, f32)) {
        let window = ctx.gfx.window();
        window.set_outer_position(ggez::winit::dpi::LogicalPosition::new(pos.0, pos.1));
    }
}

impl AnimationTrait for FallAnimation {
    fn start(&mut self, _ctx: &mut Context) {
        // mark animation start
        self.start_time = Some(Instant::now());
        self.finished = false;
    }

    fn update(&mut self, ctx: &mut Context) {
        if self.finished || self.start_time.is_none() {
            return;
        }

        let elapsed = self.start_time.unwrap().elapsed().as_secs_f32();
        let t = (elapsed / self.duration).min(1.0);

        // interpolate between start and end
        let x = (1.0 - t) * self.start_pos.0 as f32 + t * self.end_pos.0 as f32;
        let y = (1.0 - t) * self.start_pos.1 as f32 + t * self.end_pos.1 as f32;

        self.move_window(ctx, (x, y));

        if t >= 1.0 {
            self.finished = true;
        }
    }
}
