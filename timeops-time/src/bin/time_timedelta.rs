//! Measure the time duration between two points on the timeline.
//! The idea is to measure it in pure linear time, i.e. in seconds or comparable simple unit.

use std::time::Instant;
use time::{ext::InstantExt, OffsetDateTime};

fn main() {
    run_using_calendar();
    run_using_instant();
}

fn run_using_calendar() {
    // Using OffsetDateTime (calendar type)

    let t1 = OffsetDateTime::now_utc();
    std::thread::sleep(std::time::Duration::from_micros(123456));
    let t2 = OffsetDateTime::now_utc();

    let d: ::time::Duration = t2 - t1;

    let float_seconds = d.as_seconds_f64();
    let ns = d.whole_nanoseconds();

    println!("Duration: {ns} ns = {float_seconds} s");
}

fn run_using_instant() {
    // Using Instant (linear time type)
    // This is really just using the core::time::Instant type built-in to Rust, the time::Instant type is deprecated.

    let t1 = Instant::now();
    std::thread::sleep(std::time::Duration::from_micros(123456));
    let t2 = Instant::now();

    let d: ::time::Duration = t2.signed_duration_since(t1);

    let float_seconds = d.as_seconds_f64();
    let ns = d.whole_nanoseconds();

    println!("Duration: {ns} ns = {float_seconds} s");
}
