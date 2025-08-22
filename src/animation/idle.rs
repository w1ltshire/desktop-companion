use ggez::{
    Context, glam,
    graphics::{Canvas, DrawParam, Image},
};

use crate::animation::AnimationTrait;

/// Animation that represents an idle state for a character.
///
/// Currently, this animation simply displays the first sprite frame and
/// does not advance or animate over time.
///
// TODO: Implement cycling over idle sprites
pub struct IdleAnimation {
    /// All frames available for this animation.
    ///
    /// For `IdleAnimation`, only the first frame is used.
    pub sprite_frames: Vec<Image>,
}

impl AnimationTrait for IdleAnimation {
    /// Start the animation.
    ///
    /// For `IdleAnimation`, this does nothing because it has no active timeline.
    fn start(&mut self) {}

    /// Update the animation state.
    ///
    /// For `IdleAnimation`, this is a no-op since the idle animation does not change over time.
    fn update(&mut self, _ctx: &mut Context) {}

    /// Draw the animation to the provided canvas.
    ///
    /// Always draws the first frame at `(0, 0)` coordinates of the window.
    ///
    /// # Arguments
    /// * `canvas` - The canvas to draw the frame onto.
    fn draw(&self, canvas: &mut Canvas) {
        canvas.draw(
            &self.sprite_frames[0],
            DrawParam::default().dest(glam::vec2(0.0, 0.0)),
        );
    }

    /// Check whether the animation has finished.
    ///
    /// For `IdleAnimation`, this always returns `true` because there is no ongoing animation.
    fn is_finished(&self) -> bool {
        true
    }
}

