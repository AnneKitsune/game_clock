use game_time::Time;
use std::time::Duration;
fn main() {
    let mut time = Time::default();
    time.set_fixed_time(Duration::from_secs_f64(1.0 / 20.0));

    let step = 1.0 / 60.0;
    for _ in 0..60 {
        time.advance_frame(Duration::from_secs_f64(step));
        { } // Run dynamic frame (ie. game logic, rendering)
        while time.step_fixed_update() { // runs 20 times in a frame.
            { }// Run fixed frame (ie. physics)
        }
    }
}
