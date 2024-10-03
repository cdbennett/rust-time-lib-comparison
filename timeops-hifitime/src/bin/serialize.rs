//! Serialization test.
//! The hifitime crate does not serialize in ISO 8601 format to JSON.
//! Example: `{"start_time":{"duration_since_j1900_tai":{"centuries":1,"nanoseconds":776212273920270592},"time_scale":"UTC"}}`
//!
//! Issues:
//!
//! - no 'T' separator which time lib requires to parse
//! - too much precision, who wants ns precision
//! - offset not in standard format, and who needs seconds there
//!
//! You can use
//! iso8601-timestamp = { version = "0.2", features = ["std"] }
//! crate and then in your code, use its Timestamp type in your struct:
//! iso8601_timestamp::Timestamp. It uses canonical ISO 8601 format.
//! The output is truncated to milliseconds.

use hifitime::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct MyData {
    start_time: Epoch,
}

fn main() {
    let t = Epoch::now().unwrap();
    let data = MyData { start_time: t };
    let json = serde_json::to_string(&data).unwrap();
    println!("{json}");
}
