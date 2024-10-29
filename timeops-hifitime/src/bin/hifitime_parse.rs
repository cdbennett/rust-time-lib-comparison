//! Parsing test.
//! Example
//! echo "2024-08-06T22:00:15.870" | cargo run --bin parse
//! For some reason hifitime does not like the "Z" timezone offset suffix;
//! it does not return an error, but it ignores the subseconds value if you add the "Z" unless you put a space first.

use std::io::stdin;

use hifitime::{efmt::consts::ISO8601, prelude::*};

fn main() {
    for line in stdin().lines() {
        let line = line.expect("read input line");
        let line = line.trim();
        let t: Epoch = ISO8601
            .parse(line)
            .unwrap_or_else(|_| panic!("parse time '{line}'"));
        println!("Parsed time: {t:?}");
    }
}
