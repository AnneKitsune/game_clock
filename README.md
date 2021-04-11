<a href="https://crates.io/crates/game_clock">
    <img src="https://img.shields.io/crates/v/game_clock.svg" alt="Game Clock" />
</a>

# Game Clock
Support an Open Source Developer! :hearts:  
[![Become a patron](https://c5.patreon.com/external/logo/become_a_patron_button.png)](https://www.patreon.com/jojolepro)

Read the [documentation](https://docs.rs/game_clock).

# Features

* Adds a simple clock for use in games and game engines.

# Usage
Add the following to you Cargo.toml file:

```
game_clock = "1.0.0"
```

Use the clock like so:
```rust
use game_clock::Time;
use std::time::Duration;
fn main() {
    let mut time = Time::default();
    time.set_fixed_time(Duration::from_secs_f64(1.0 / 20.0));

    let step = 1.0 / 60.0;
    for _ in 0..60 {
        time.advance_frame(Duration::from_secs_f64(step));
        { } // ...Run game logic, rendering, etc...
        while time.step_fixed_update() { // runs 20 times in a frame.
            { } // Run fixed frame logic (ie. physics)
        }
    }
}
```
### Maintainer Information

* Maintainer: Jojolepro
* Contact: jojolepro [at] jojolepro [dot] com
* Website: [jojolepro.com](https://jojolepro.com)
* Patreon: [patreon](https://patreon.com/jojolepro)

