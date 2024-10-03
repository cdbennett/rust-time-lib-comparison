//! chrono: serialization test
//!
//! The chrono crate does serialize to ISO 8601 by default.
//!
//! However, some things are clunky due to the use of traits for time zones, so
//! you get local time with `Local::now()`, but then you can't store this in a
//! variable of type `DateTime<FixedOffset>` which is the usual date/time type.

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct MyData {
    offset_dt: DateTime<FixedOffset>,
    //local_dt: DateTime<FixedOffset>,
    civil_dt: NaiveDateTime,
}

fn main() {
    let offset_dt = DateTime::parse_from_rfc3339("2024-10-03T17:36:05.910625056Z").unwrap();
    //let local_now: DateTime<Local> = Local::now();
    let civil_dt = NaiveDateTime::new(
        NaiveDate::from_ymd_opt(2024, 11, 3).unwrap(),
        NaiveTime::from_hms_opt(1, 30, 0).unwrap(),
    );
    let data = MyData {
        offset_dt,
        //local_dt: local_now, // can't do this directly. How to store current local time in a general-purpose DateTime field?
        civil_dt,
    };
    let json = serde_json::to_string_pretty(&data).unwrap();
    // We would hope to get a fixed-offset time like this, but the traits make this difficult:
    //"local_dt": "2024-10-03T10:36:05.910631358-07:00",
    assert_eq!(
        json,
        r#"{
  "offset_dt": "2024-10-03T17:36:05.910625056Z",
  "civil_dt": "2024-11-03T01:30:00"
}"#
    );

    // Deserialize

    let given_json = r#"{
  "offset_dt": "2024-10-03T17:36:05.910-0700",
  "civil_dt": "2024-11-03T01:30:00"
}"#;
    let deserialized: MyData = serde_json::from_str(given_json).unwrap();
    assert_eq!(
        deserialized.offset_dt.to_string(),
        "2024-10-03 17:36:05.910 -07:00"
    );
    assert_eq!(deserialized.civil_dt.to_string(), "2024-11-03T01:30:00");
}
