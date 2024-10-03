//! Get the beginning of tomorrow.
//! This tests doing some manipulations with time/date values, and
//! also tests around a DST change, which `time` does not support well.

use time::{Date, OffsetDateTime, Time, UtcOffset};

fn main() {
    // let now = OffsetDateTime::now_utc();
    // Given America/New_York time zone, which is UTC-4 during the summer, and changes to UTC-5 on 2024-11-03.
    // Given the time is 00:15 on 3 November, just hours before the DST change.
    // Note that the start of the local day is 04:00 UTC. But the start of tomorrow is 05:00 UTC (the 3rd is a 25-hour day).
    let now = OffsetDateTime::new_in_offset(
        Date::from_calendar_date(2024, time::Month::November, 3).unwrap(),
        Time::from_hms(0, 15, 0).unwrap(),
        UtcOffset::from_hms(-4, 0, 0).unwrap(),
    );
    // Extract the civil (time zone-naive) date.
    let today = now.date();
    let tomorrow = today.next_day().unwrap();
    let start_of_tomorrow = tomorrow.with_hms(0, 0, 0).unwrap();
    // It's not too obvious, but `assume_offset()` actually changes the time value, it doesn't just combine it with an offset.
    // In this case we have "00:00:00".assume_offset(UTC-4) == "04:00:00".
    let start_of_tomorrow = start_of_tomorrow.assume_offset(now.offset());
    // In actuality it should be 2024-11-04T05:00:00Z, but we get 04:00 instead
    // since DST change is not handled.
    assert_eq!(
        start_of_tomorrow.to_string(),
        "2024-11-04 0:00:00.0 -04:00:00"
    );
    // The default format is kind of odd:
    // - hours are not padded to 2 digits
    // - sub-seconds value is included even if empty but has 1..N digits
    // - time zone offset includes seconds
}
