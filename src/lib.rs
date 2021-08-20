//! # Standard `embedded-time`
//!
//! This library provides an [embedded_time::Clock] that can be used for host-side testing.
//!
//! The provided [embedded_time::Clock] implementation is based on [std::time].
//!
//! # Usage
//!
//! ```rust
//! use std_embedded_time::StandardClock;
//! use embedded_time::Clock;
//!
//! fn main() {
//!     let clock = StandardClock::default();
//!
//!     let now = clock.try_now().unwrap();
//!     println!("Current time: {:?}", now);
//! }
//! ```

pub use embedded_time;

use embedded_time::{fraction::Fraction, Instant};

/// A clock with nanosecond precision.
///
/// To construct a clock, use [StandardClock::default()].
///
/// The clock is "started" when it is constructed.
///
/// # Limitations
/// The clock represents up to ~584 years worth of time, after which it will roll over.
#[derive(Copy, Clone, Debug)]
pub struct StandardClock {
    start: std::time::Instant,
}

impl Default for StandardClock {
    fn default() -> Self {
        Self {
            start: std::time::Instant::now(),
        }
    }
}

impl embedded_time::Clock for StandardClock {
    /// With a 64-bit tick register, the clock can represent times up to approximately 594 years in
    /// duration, after which the clock will roll over.
    type T = u64;

    /// Each tick of the clock is equivalent to 1 nanosecond.
    const SCALING_FACTOR: Fraction = Fraction::new(1, 1_000_000_000);

    /// Get the current time from the clock.
    fn try_now(&self) -> Result<Instant<Self>, embedded_time::clock::Error> {
        let now = std::time::Instant::now();

        let elapsed = now.duration_since(self.start);

        // Note: We are discarding the upper 64 bits of nanoseconds. However, even while doing so,
        // we can represent ~594 years of time, so this should not be relevant.
        Ok(Instant::new(elapsed.as_nanos() as u64))
    }
}

#[cfg(test)]
mod test {
    use super::StandardClock;
    use core::convert::TryFrom;
    use embedded_time::{duration::*, Clock};
    use std::time::Duration;

    #[test]
    fn test_measurement() {
        let clock = StandardClock::default();

        let start = clock.try_now().unwrap();
        std::thread::sleep(Duration::from_secs(1));
        let end = clock.try_now().unwrap();

        let elapsed = end - start;

        let lower_bound = Milliseconds::<u64>::try_from(999u32.milliseconds()).unwrap();
        assert!(elapsed > lower_bound.into());

        let upper_bound = Milliseconds::<u64>::try_from(2000u32.milliseconds()).unwrap();
        assert!(elapsed < upper_bound.into());
    }
}
