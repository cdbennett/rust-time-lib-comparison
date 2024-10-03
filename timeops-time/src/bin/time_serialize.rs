//! Serialization test.
//! The `time` crate does not serialize in ISO 8601 format to JSON.
//! Example: `{"start_time":"2024-08-06 21:30:00.335356438 +00:00:00"}`
//!
//! Issues:
//!
//! - non-standard serialization format (not ISO 8601 or RFC 3339)
//!   - no 'T' separator which time lib requires to parse
//!   - time zone offset in non-standard format (why seconds?!)
//!
//! To make it work correctly to serialize to a standard-compatible format,
//! you can use the `iso8601-timestamp` crate's `Timestamp` type in your
//! structs you will be serializing.
//!
//! Add `iso8601-timestamp = { version = "0.2", features = ["std"] }`
//! to Cargo.toml then store values as iso8601_timestamp::Timestamp.
//! It uses canonical ISO 8601 format.
//! (Note that this truncates the output to millisecond precision.)

use serde::{Deserialize, Serialize};
use time::{format_description::well_known::Rfc3339, OffsetDateTime, PrimitiveDateTime};

#[derive(Serialize, Deserialize, Debug)]
struct MyData {
    my_offset_date_time: OffsetDateTime,
    my_civil_date_time: PrimitiveDateTime,
}

fn main() {
    // Because `time` crate requires us to tell it what the time string format is in order to parse it,
    // it doesn't implement the simple `FromStr` trait, so we can't do `let t: OffsetDateTime = s.parse()?`
    // but have to do `let t = OffsetDateTime::parse(s, fmt)?` which is much less helpful in the common case
    // where we are using the standard time format.
    let odt = OffsetDateTime::parse("2024-10-03T10:59:06.749952421-07:00", &Rfc3339).unwrap();
    // The Iso8601<EncodedConfig> type seems flexible but very cumbersome. I gave up on it.
    // The Rfc3339 format doesn't require more work. But it's rigid.

    // Is there a simpler way to get a PrimitiveDateTime for an OffsetDateTime?
    let cdt = PrimitiveDateTime::new(odt.date(), odt.time());
    let data = MyData {
        my_offset_date_time: odt,
        my_civil_date_time: cdt,
    };
    let json = serde_json::to_string_pretty(&data).unwrap();
    assert_eq!(
        json,
        r#"{
  "my_offset_date_time": "2024-10-03 10:59:06.749952421 -07:00:00",
  "my_civil_date_time": "2024-10-03 10:59:06.749952421"
}"#
    );

    // We get particularly unhelpful error messages from the `time` crate when parsing fails.
    let r = OffsetDateTime::parse("2024-10-03 10:08:31.906912391 -07:00:00", &Rfc3339);
    let err = r.unwrap_err();
    assert_eq!(err.to_string(), "a character literal was not valid");
    // Can you even see the difference? And this string was taken directly from the serialized output of `time`.
}
