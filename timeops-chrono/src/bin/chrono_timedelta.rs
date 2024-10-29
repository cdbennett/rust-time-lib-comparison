//! Measure the time duration between two points on the timeline.
//! The idea is to measure it in pure linear time, i.e. in seconds or comparable simple unit.

use chrono::prelude::*;

fn main() {
    let t1 = Utc::now();
    std::thread::sleep(std::time::Duration::from_micros(123456));
    let t2 = Utc::now();

    let d = t2 - t1;

    // The chrono TimeDelta type does not appear to have a way to directly get
    // floating point seconds or total nanoseconds. So we go through the std Duration type.
    let float_seconds = d.to_std().unwrap().as_secs_f64();
    let ns = d.to_std().unwrap().as_nanos();

    println!("Duration: {ns} ns = {float_seconds} s");
}
