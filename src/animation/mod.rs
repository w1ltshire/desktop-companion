use ggez::{Context, graphics::Canvas};
use std::collections::hash_map::HashMap;

pub mod idle;
pub mod movement;

/// Manages multiple companion animations and tracks the currently active one.
///
/// Provides functions to start, update, and draw animations by name. If no animation
/// is active, it falls back to the `"idle"` animation if present.
pub struct CompanionAnimations {
    /// Map of animation names to their boxed implementations of [`AnimationTrait`].
    pub animations: HashMap<String, Box<dyn AnimationTrait>>,

    /// Name of the currently active animation, if any.
    pub active: Option<String>,
}

/// Trait that all companion animations must implement.
///
/// Defines a minimal interface for starting, updating, drawing, and querying
/// whether an animation has finished.
pub trait AnimationTrait {
    /// Called to start the animation from its beginning.
    fn start(&mut self);

    /// Updates the animation state. Typically called once per frame.
    ///
    /// # Arguments
    /// * `ctx` - ggez context required for certain animation operations.
    fn update(&mut self, ctx: &mut Context);

    /// Draws the animation to the provided canvas.
    ///
    /// # Arguments
    /// * `canvas` - The canvas to render the animation onto.
    fn draw(&self, canvas: &mut Canvas);

    /// Returns `true` if the animation has finished and should no longer be updated.
    fn is_finished(&self) -> bool;
}

impl CompanionAnimations {
    /// Creates a new empty `CompanionAnimations` manager with no active animation.
    pub fn new() -> Self {
        CompanionAnimations {
            animations: HashMap::new(),
            active: None,
        }
    }

    /// Adds a new animation to the manager under the given name.
    ///
    /// # Arguments
    /// * `animation` - Boxed animation implementing [`AnimationTrait`].
    /// * `name` - Unique name to reference the animation.
    pub fn push(&mut self, animation: Box<dyn AnimationTrait>, name: String) {
        self.animations.insert(name, animation);
    }

    /// Starts an animation by name and sets it as active.
    ///
    /// # Arguments
    /// * `name` - Name of the animation to start.
    /// * `_ctx` - ggez context (currently unused in this method).
    pub fn start(&mut self, name: &str, _ctx: &mut Context) {
        if let Some(anim) = self.animations.get_mut(name) {
            anim.start();
            self.active = Some(name.to_string());
        }
    }

    /// Updates the currently active animation.
    ///
    /// If the active animation has finished, clears the `active` field.
    ///
    /// # Arguments
    /// * `ctx` - ggez context for updating animations.
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

    /// Draws the active animation to the canvas.
    ///
    /// If no animation is active, draws the `"idle"` animation if it exists.
    ///
    /// # Arguments
    /// * `_ctx` - ggez context (currently unused in this method).
    /// * `canvas` - Canvas to draw the animation onto.
    pub fn draw(&self, _ctx: &mut Context, canvas: &mut Canvas) {
        if let Some(active_name) = &self.active {
            if let Some(anim) = self.animations.get(active_name) {
                anim.draw(canvas);
                return;
            }
        }

        // Fallback to idle animation
        if let Some(idle_anim) = self.animations.get("idle") {
            idle_anim.draw(canvas);
        }
    }
}
