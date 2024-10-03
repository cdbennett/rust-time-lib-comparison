//! Serialization test.
//! The jiff crate automatically defaults to serializing to ISO 8601 / RFC 3339
//! format (with `Zoned` using TC39 extended data, suffixing it with the time zone
//! name in brackets). You only need to enable the `serde` feature of `jiff` to
//! make serialization work.

use jiff::{civil::DateTime, Timestamp, Zoned};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct MyData {
    my_instant: Timestamp,
    my_zoned: Zoned,
    my_civil: DateTime,
}

fn main() {
    let t: Timestamp = "2024-10-03T16:44:57.548805092Z".parse().unwrap();
    let zdt = "2024-10-03T09:44:57.548842272-07:00[America/Los_Angeles]"
        .parse()
        .unwrap();
    let cdt = DateTime::from(&zdt);
    let data = MyData {
        my_instant: t,
        my_zoned: zdt,
        my_civil: cdt,
    };

    let json = serde_json::to_string_pretty(&data).unwrap();

    assert_eq!(
        json,
        r#"{
  "my_instant": "2024-10-03T16:44:57.548805092Z",
  "my_zoned": "2024-10-03T09:44:57.548842272-07:00[America/Los_Angeles]",
  "my_civil": "2024-10-03T09:44:57.548842272"
}"#
    );
}
