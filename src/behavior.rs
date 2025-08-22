use std::time::Instant;

use ggez::winit;
use log::debug;
use rand::distr::{weighted::WeightedIndex, Distribution};

/// Types of Behavior
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Behavior {
    Idle,
    WalkLeft,
    WalkRight,
    Fall,
    Jump,
}

/// BehaviorManager manages companion's behavior by providing functions that return Behavior
/// depending on various factors
pub struct BehaviorManager {
    current: Option<Behavior>,
    previous: Option<Behavior>,
    last_change: Instant,
    window: *const winit::window::Window
}

/// Simplified type for `TRANSITIONS`
type TransitionType = &'static [(Option<Behavior>, &'static [(Behavior, f32)])];

/// Transition chances between Behavior types
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
    /// Creates a new BehaviorManager. `current` and `previous` are set to None beacause at the
    /// time when BehaviorManager is constructed (`new()` in CompanionApp, basically the start of
    /// the program) there's nothing the companion is doing.
    ///
    /// # Arguments
    /// * 'window' - Raw const pointer to winit::winit::Window
    ///
    /// # Returns
    /// Created BehaviorManager
    pub fn new(window: *const winit::window::Window) -> Self {
        Self {
            current: None,
            previous: None,
            last_change: Instant::now(),
            window
        }
    }

    /// Picks a Behavior for the companion using WeightedIndex and other algorithms
    ///
    /// # Returns
    /// Some(Behavior) if current self.current is None (first call) or it's been 10 second since
    /// the last update
    /// None otherwise
    pub fn update(&mut self) -> Option<Behavior> {
        if self.current.is_none() || self.last_change.elapsed().as_secs_f32() > 10.0 {
            self.previous = self.current;
            self.current = Some(self.pick_behavior_random());
            self.last_change = Instant::now();

            debug!("ima behave >:3 {:?} also have this window size so i could test my pOiNtErS {:?}", self.current, unsafe { (*self.window).outer_size() });

            return self.current;
        }

        None
    }

    /// Picks a random Behavior using WeightedIndex
    ///
    /// # Returns
    /// Selected Behavior
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
