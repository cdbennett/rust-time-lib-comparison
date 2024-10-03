//! Parsing test.
//! The `time` create makes it verbose to parse standard time strings, meaning it
//! doesn't implement `FromStr` but instead requires you to pass also a specification
//! of the format of your time string.

use time::{
    format_description::well_known::{Iso8601, Rfc3339},
    OffsetDateTime, PrimitiveDateTime,
};

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
        let odt_result = OffsetDateTime::parse(s, &Iso8601::PARSING)
            .map(|t| t.to_string())
            .unwrap_or("-".to_owned());

        // I'm confused about this one, parsing as RFC 3339 fails for most strings.
        // Shouldn't it parse "2024-11-03T00:15:00-0400" for example?
        let odt_rfc_result = OffsetDateTime::parse(s, &Rfc3339)
            .map(|t| t.to_string())
            .unwrap_or("-".to_owned());

        let ndt_result = PrimitiveDateTime::parse(s, &Iso8601::PARSING)
            .map(|t| t.to_string())
            .unwrap_or("-".to_owned());

        println!("{s:<45} {odt_result:<45} {odt_rfc_result:<45} {ndt_result:<45}");
    }
}
