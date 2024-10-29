//! Measure the time duration between two points on the timeline.
//! The idea is to measure it in pure linear time, i.e. in seconds or comparable simple unit.

use jiff::*;

fn main() -> Result<(), jiff::Error> {
    run_using_calendar();
    run_using_instant();
    Ok(())
}

fn run_using_calendar() {
    // Using Zoned (calendar type)

    let t1 = Zoned::now();
    std::thread::sleep(std::time::Duration::from_micros(123456));
    let t2 = Zoned::now();

    let d = t2.duration_since(&t1);

    let float_seconds = d.as_secs_f64();
    let ns = d.as_nanos();

    println!("Duration: {ns} ns = {float_seconds} s");
}

fn run_using_instant() {
    // Using Timestamp (linear time type)

    let t1 = Timestamp::now();
    std::thread::sleep(std::time::Duration::from_micros(123456));
    let t2 = Timestamp::now();

    let d = t2.duration_since(t1);

    let float_seconds = d.as_secs_f64();
    let ns = d.as_nanos();

    println!("Duration: {ns} ns = {float_seconds} s");
}
