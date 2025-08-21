use std::time::Instant;

use rand::seq::IndexedMutRandom;

#[derive(Debug, Clone, Copy)]
pub enum Behavior {
    Idle,
    WalkLeft,
    WalkRight,
    Fall,
    Jump,
}

pub struct BehaviorManager {
    current: Option<Behavior>,
    last_change: Instant,
}

impl BehaviorManager {
    pub fn new() -> Self {
        Self {
            current: None,
            last_change: Instant::now(),
        }
    }

    pub fn update(&mut self) -> Option<Behavior> {
        if self.current.is_none() || self.last_change.elapsed().as_secs_f32() > 3.0 {
            let mut choices = [
                Behavior::Idle,
                Behavior::WalkLeft,
                Behavior::WalkRight,
                Behavior::Fall,
            ];

            let mut rng = rand::rng();
            self.current = Some(*choices.choose_mut(&mut rng).unwrap());
            self.last_change = Instant::now();

            return self.current;
        }

        None
    }
}
