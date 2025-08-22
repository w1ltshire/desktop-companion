//! Module `behavior_manager` handles the companion's behavior selection.
//!
//! This module is internal and manages:
//! - Tracking current and previous behaviors
//! - Timing updates to avoid rapid behavior changes
//! - Picking behaviors based on transition weights
//!
//! # Safety
//! This module uses raw pointers to `winit::Window`. They must remain valid while
//! the manager exists.

use std::time::Instant;

use ggez::winit;
use log::debug;
use rand::distr::{Distribution, weighted::WeightedIndex};

/// Possible behaviors for the companion character.
///
/// Used by [`BehaviorManager`] to determine animations and actions.
/// - `Idle`: doing nothing
/// - `WalkLeft` / `WalkRight`: moving horizontally
/// - `Fall`: fall out from top of the screen, only if companion is at desired position
/// - `Jump`: self-explanatory
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Behavior {
    Idle,
    WalkLeft,
    WalkRight,
    Fall,
    Jump,
}

/// Manages the companion's behavior state.
///
/// Tracks the current and previous [`Behavior`] and updates them based on
/// transition probabilities. Updates occur at intervals to prevent rapid changes.
///
/// # Safety
/// The `window` pointer must remain valid for the lifetime of the manager.
pub struct BehaviorManager {
    current: Option<Behavior>,
    previous: Option<Behavior>,
    last_change: Instant,
    window: *const winit::window::Window,
}

/// Simplified type for [`TRANSITIONS`]
type TransitionType = &'static [(Option<Behavior>, &'static [(Behavior, f32)])];

/// Maps a previous [`Behavior`] to weighted probabilities for the next behavior.
///
/// Each entry contains:
/// - An `Option<Behavior>` representing the previous state (or `None` for initial state)
/// - A slice of `(Behavior, weight)` tuples representing possible next behaviors and their relative probabilities
static TRANSITIONS: TransitionType = &[
    (
        Some(Behavior::Idle),
        &[
            (Behavior::Idle, 0.2),
            (Behavior::WalkLeft, 0.3),
            (Behavior::WalkRight, 0.3),
            (Behavior::Jump, 0.1),
        ],
    ),
    (
        Some(Behavior::WalkLeft),
        &[
            (Behavior::Idle, 0.4),
            (Behavior::WalkLeft, 0.1),
            (Behavior::WalkRight, 0.3),
            (Behavior::Jump, 0.1),
        ],
    ),
    (
        None,
        &[
            (Behavior::Idle, 0.25),
            (Behavior::WalkLeft, 0.25),
            (Behavior::WalkRight, 0.25),
            (Behavior::Jump, 0.15),
        ],
    ),
];

impl BehaviorManager {
    /// Creates a new [`BehaviorManager'] associated with the companions [`winit::winit::Window`].
    ///
    /// Initially, both `current` and `previous` are None.
    ///
    /// # Safety
    /// The caller must ensure that the provided [`winit::winit::Window`] pointer is non-null and
    /// remains valid for the lifetime of the manager
    pub fn new(window: *const winit::window::Window) -> Self {
        Self {
            current: None,
            previous: None,
            last_change: Instant::now(),
            window,
        }
    }

    /// Updates the companion’s behavior if enough time has passed.
    ///
    /// Returns `Some(Behavior)` if:
    /// - this is the first call, or
    /// - more than 10 seconds have elapsed since the last change.
    ///
    /// Otherwise, returns `None`.
    pub fn update(&mut self) -> Option<Behavior> {
        if self.current.is_none() || self.last_change.elapsed().as_secs_f32() > 10.0 {
            self.previous = self.current;
            self.current = Some(self.pick_behavior_random());
            self.last_change = Instant::now();

            debug!(
                "ima behave >:3 {:?} also have this window size so i could test my pOiNtErS {:?}",
                self.current,
                unsafe { (*self.window).outer_size() }
            );

            return self.current;
        }

        None
    }

    /// Selects a random [`Behavior`] according to the transition weights.
    ///
    /// Called internally by [`update`].
    fn pick_behavior_random(&mut self) -> Behavior {
        let weights = TRANSITIONS
            .iter()
            .find(|(state, _)| *state == self.previous)
            .map(|(_, probs)| *probs)
            .unwrap_or_else(|| TRANSITIONS.last().unwrap().1);

        let mut rng = rand::rng();
        let dist = WeightedIndex::new(weights.iter().map(|(_, w)| *w)).unwrap();
        weights[dist.sample(&mut rng)].0
    }
}
