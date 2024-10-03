//! chrono: parsing test.
//!
//! Parsing a standard string like "2024-11-03T00:15:00-0400" fails
//! in the RFC 3339 parser in `chrono`. It only seems to accept strings
//! with a `Z` suffix, not a numeric offset suffix.
//!
//! Maybe I'm missing some other method that should be used to parse
//! strings, because the `serde` deserialization does accept strings
//! with a numeric offset suffix.
//!
//! To parse standard time formats, I had to use the external crate iso8601-timestamp,
//! and use its `iso8601_timestamp::Timestamp` type in my structs. These obviously
//! must be converted to/from the chrono types when manipulating them.

use chrono::prelude::*;

fn main() {
    let inputs = [
        "2024-11-03T00:15:00.123456789Z",
        "2024-11-03T00:15:00Z",
        "2024-11-03 00:15:00Z",
        "2024-11-03T00:15:00+00",
        "2024-11-03T00:15:00-0400",
        "2024-11-03T00:15:00-07",
        "2024-11-03T00:15:00.123456789+0000",
        "2024-11-03T00:15:00",
        "2024-11-03 00:15:00",
        "2024-11-03T00:15:00[UTC]",
        "2024-11-03T00:15:00[America/New_York]",
        "2024-11-03T00:15:00-0400[America/New_York]",
        "2024-11-03T02:15:00-0500[America/New_York]",
        "2024-11-03T00:15:00-0500[America/New_York]",
    ];

    for &s in inputs.iter() {
        let odt_result = DateTime::parse_from_rfc3339(s)
            .map(|t| t.to_string())
            .unwrap_or("-".to_owned());

        let ndt_result = s
            .parse::<chrono::NaiveDateTime>()
            .map(|t| t.to_string())
            .unwrap_or("-".to_owned());

        println!("{s:<45} {odt_result:<45} {ndt_result:<45}");
    }
}
