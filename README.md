# std-embedded-time

Provides an
[`embedded-time::Clock`](https://docs.rs/embedded-time/0.12.0/embedded_time/clock/trait.Clock.html)
using [`std::time`] so that `embedded-time` can eaisly be used in on-host testing.

### Usage

It's extremely straight-forward to start using a clock:

```rust
use std_embedded_time::StandardClock;
use embedded_time::Clock;

fn main() {
    let clock = StandardClock::default();

    let now = clock.try_now().unwrap();
    println!("Current time: {:?}", now);
}
```
