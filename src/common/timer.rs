//! Tickable timer.
//!
//! Allows timers that depend exclusively on the in-game tick loop, makikng the
//! game more deterministic and squashing pause-screen exploits.

use super::simul::Tickable;

pub struct Timer {
    elapsed: f64,
    action: fn() -> (),
    threshold: f64,
    repeating: bool,
    done: bool,
    paused: bool,
}

impl Timer {
    pub fn new_timeout(after: f64, action: fn() -> ()) -> Self {
        Timer {
            elapsed: 0.0,
            threshold: after,
            repeating: false,
            action,
            done: false,
            paused: false,
        }
    }

    pub fn new_interval(after: f64, action: fn() -> ()) -> Self {
        Timer {
            elapsed: 0.0,
            threshold: after,
            repeating: true,
            action,
            done: false,
            paused: false,
        }
    }

    pub fn stop(&mut self) {
        self.done = true
    }

    pub fn pause(&mut self) {
        self.paused = true
    }

    pub fn unpause(&mut self) {
        self.paused = false
    }
}

impl Tickable for Timer {
    fn tick(&mut self, delta_time: f64) {
        if self.done || self.paused {
            return;
        }

        self.elapsed += delta_time;

        while self.elapsed >= self.threshold {
            (self.action)();
            self.elapsed -= self.threshold;
            if !self.repeating {
                self.done = true;
                return;
            }
        }
    }

    fn is_destroyed(&self) -> bool {
        self.done
    }
}
