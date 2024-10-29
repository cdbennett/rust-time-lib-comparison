//! Measure the time duration between two points on the timeline.
//! The idea is to measure it in pure linear time, i.e. in seconds or comparable simple unit.

use hifitime::prelude::*;

fn main() {
    let t1 = Epoch::now().unwrap();
    std::thread::sleep(std::time::Duration::from_micros(123456));
    let t2 = Epoch::now().unwrap();

    let d = t2 - t1;

    let float_seconds = d.to_seconds();
    let ns = d.total_nanoseconds();

    println!("Duration: {ns} ns = {float_seconds} s");
}
