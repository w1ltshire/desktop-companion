use std::time::Instant;
use ggez::{
    Context, glam,
    graphics::{Canvas, DrawParam, Image},
    winit::dpi::LogicalPosition,
};
use crate::animation::AnimationTrait;

/// Speed multiplier for walking animation frames.
const WALKSPEED: f32 = 5.0;

/// Direction of movement for the animation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Direction {
    /// Moving vertically (up/down)
    Vertical,
    /// Moving left
    Left,
    /// Moving right
    Right,
}

/// Animation that moves a character from a start to an end position over time.
///
/// Handles updating the character's position, determining completion,
/// and drawing the appropriate walking sprte
#[derive(Debug)]
pub struct MoveAnimation {
    /// Starting position `(x, y)` in logical coordinates.
    pub start_pos: (f32, f32),

    /// Target position `(x, y)` to move toward.
    pub end: (f32, f32),

    /// Duration of the movement in seconds.
    pub duration: f32,

    /// Time when the animation started.
    pub start_time: Instant,

    /// Whether the animation has finished.
    pub finished: bool,

    /// Sprite frames for walking animation.
    pub sprite_frames: Vec<Image>,

    /// Current position `(x, y)` updated over time.
    pub current_pos: (f32, f32),

    /// Direction of movement.
    pub direction: Direction,
}

impl AnimationTrait for MoveAnimation {
    /// Initializes the animation, recording the start time and resetting the finished flag.
    fn start(&mut self) {
        self.start_time = Instant::now();
        self.finished = false;
        self.current_pos = self.start_pos;
    }

    /// Updates the animation state.
    ///
    /// - Interpolates the position based on elapsed time and duration.
    /// - Updates the window's logical position to match the current position.
    /// - Marks the animation as finished if the end position is reached.
    ///
    /// # Arguments
    /// * `ctx` - ggez context required to move the window.
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

    /// Draws the current frame of the animation to the canvas.
    ///
    /// - Chooses a sprite frame based on elapsed time and `WALKSPEED`.
    /// - Mirrors the sprite horizontally if moving to the right.
    ///
    /// # Arguments
    /// * `canvas` - The canvas to draw the sprite onto.
    fn draw(&self, canvas: &mut Canvas) {
        if !self.is_finished() {
            let frame_index = ((self.start_time.elapsed().as_secs_f32() * WALKSPEED) as usize)
                % self.sprite_frames.len();
            let sprite = &self.sprite_frames[frame_index];

            let mut param = DrawParam::default().dest(glam::vec2(0.0, 0.0));
            if self.direction == Direction::Right {
                param = param
                    .scale(glam::vec2(-1.0, 1.0)) // mirror horizontally
                    .offset(glam::vec2(1.0, 0.0)); // pivot around center
            }

            canvas.draw(sprite, param);
        }
    }

    /// Returns true if the movement animation has finished.
    fn is_finished(&self) -> bool {
        self.finished
    }
}
