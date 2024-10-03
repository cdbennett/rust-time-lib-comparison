//! Parsing test.
//! The `time` create makes it verbose to parse standard time strings, meaning it
//! doesn't implement `FromStr` but instead requires you to pass also a specification
//! of the format of your time string.

use std::io::stdin;

use time::{format_description::well_known::Iso8601, OffsetDateTime};

fn main() {
    for line in stdin().lines() {
        let line = line.expect("read input line");
        let line = line.trim();
        let t: OffsetDateTime =
            OffsetDateTime::parse(&line, &Iso8601::PARSING).expect(&format!("parse time '{line}'"));
        println!("Parsed time: {t:?}");
    }
}
