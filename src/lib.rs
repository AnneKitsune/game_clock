//! Utilities for working with time in games.
//!
//! Original from the Amethyst Engine.
//! Under the dual license Apache/MIT.

use std::time::Duration;

/// Frame timing values.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Time {
    /// Time elapsed since the last frame.
    delta_time: Duration,
    /// Time elapsed since the last frame ignoring the time speed multiplier.
    delta_real_time: Duration,
    /// Rate at which `State::fixed_update` is called.
    fixed_time: Duration,
    /// The total number of frames that have been played in this session.
    frame_number: u64,
    ///Time elapsed since game start, ignoring the speed multipler.
    absolute_real_time: Duration,
    ///Time elapsed since game start, taking the speed multiplier into account.
    absolute_time: Duration,
    ///Time multiplier. Affects returned delta_time and absolute_time.
    time_scale: f32,
    /// Fixed timestep accumulator.
    fixed_time_accumulator: Duration,
}

impl Time {
    /// Gets the time difference between frames.
    pub fn delta_time(&self) -> Duration {
        self.delta_time
    }

    /// Gets the time difference between frames ignoring the time speed multiplier.
    pub fn delta_real_time(&self) -> Duration {
        self.delta_real_time
    }

    /// Gets the fixed time step.
    /// Must be used instead of delta_time during fixed updates.
    pub fn fixed_time(&self) -> Duration {
        self.fixed_time
    }

    /// Gets the current frame number.  This increments by 1 every frame.  There is no frame 0.
    pub fn frame_number(&self) -> u64 {
        self.frame_number
    }

    /// Gets the time since the start of the game, taking into account the speed multiplier.
    pub fn absolute_time(&self) -> Duration {
        self.absolute_time
    }

    /// Gets the time since the start of the game, ignoring the speed multiplier.
    pub fn absolute_real_time(&self) -> Duration {
        self.absolute_real_time
    }

    /// Gets the current time speed multiplier.
    pub fn time_scale(&self) -> f32 {
        self.time_scale
    }

    /// Sets delta_time to the given `Duration`.
    /// Updates the struct to reflect the changes of this frame.
    /// This should be called before using step_fixed_update.
    pub fn advance_frame(&mut self, time_diff: Duration) {
        let secs = duration_to_secs(time_diff);
        self.delta_time = secs_to_duration(secs * self.time_scale as f64);
        self.delta_real_time = time_diff;
        self.frame_number += 1;

        self.absolute_time += self.delta_time;
        self.absolute_real_time += self.delta_real_time;
        self.fixed_time_accumulator += self.delta_real_time;
    }

    /// Sets both `fixed_time` and `fixed_seconds` based on the duration given.
    pub fn set_fixed_time(&mut self, time: Duration) {
        self.fixed_time = time;
    }

    /// Sets the time multiplier that affects how time values are computed,
    /// effectively slowing or speeding up your game.
    ///
    /// ## Panics
    /// This will panic if multiplier is NaN, Infinity, or less than 0.
    pub fn set_time_scale(&mut self, multiplier: f32) {
        assert!(multiplier >= 0.0);
        assert!(multiplier != std::f32::INFINITY);
        self.time_scale = multiplier;
    }

    /// Checks to see if we should perform another fixed update iteration, and if so, returns true
    /// and reduces the accumulator.
    pub fn step_fixed_update(&mut self) -> bool {
        if self.fixed_time_accumulator >= self.fixed_time {
            self.fixed_time_accumulator -= self.fixed_time;
            true
        } else {
            false
        }
    }
}

impl Default for Time {
    fn default() -> Time {
        Time {
            delta_time: Duration::from_secs(0),
            delta_real_time: Duration::from_secs(0),
            fixed_time: Duration::new(0, 16_666_666),
            fixed_time_accumulator: Duration::new(0, 0),
            frame_number: 0,
            absolute_real_time: Duration::default(),
            absolute_time: Duration::default(),
            time_scale: 1.0,
        }
    }
}

/// Converts a Duration to the time in seconds.
pub fn duration_to_secs(duration: Duration) -> f64 {
    duration.as_secs() as f64 + (f64::from(duration.subsec_nanos()) / 1.0e9)
}

/// Converts a time in seconds to a duration
pub fn secs_to_duration(secs: f64) -> Duration {
    Duration::new(secs as u64, ((secs % 1.0) * 1.0e9) as u32)
}

#[cfg(test)]
mod tests {
    use crate::*;
    // Test that fixed_update methods accumulate and return correctly
    // Test confirms that with a fixed update of 120fps, we run fixed update twice with the timer
    // Runs at 10 times game speed, which shouldn't affect fixed updates
    #[test]
    fn fixed_update_120fps() {
        let mut time = Time::default();
        time.set_fixed_time(secs_to_duration(1.0 / 120.0));
        time.set_time_scale(10.0);

        let step = 1.0 / 60.0;
        let mut fixed_count = 0;
        for _ in 0..60 {
            time.advance_frame(secs_to_duration(step));
            while time.step_fixed_update() {
                fixed_count += 1;
            }
        }

        assert_eq!(fixed_count, 120);
    }

    // Test that fixed_update methods accumulate and return correctly
    // Test confirms that with a fixed update every 1 second, it runs every 1 second only
    #[test]
    fn fixed_update_1sec() {
        let mut time = Time::default();
        time.set_fixed_time(secs_to_duration(1.0));

        let step = 1.0 / 60.0;
        let mut fixed_count = 0;
        for _ in 0..130 {
            // Run two seconds
            time.advance_frame(secs_to_duration(step));
            while time.step_fixed_update() {
                fixed_count += 1;
            }
        }
        assert_eq!(fixed_count, 2);
    }
}

