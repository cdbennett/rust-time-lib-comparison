//! Get the beginning of tomorrow.
//! This is implemented in `jiff` in a way that works correctly even
//! when it involves a DST offset change.

use jiff::civil::date;
use jiff::*;

fn main() -> Result<(), jiff::Error> {
    // Given we are just a few hours before the DST change from UTC-5 to UTC-4
    let now = date(2024, 11, 03)
        .at(0, 15, 0, 0)
        .intz("America/New_York")
        .unwrap();

    // Get the start of this local day. This will be 04:00 UTC.
    let today = now.round(ZonedRound::new().smallest(Unit::Day).mode(RoundMode::Floor))?;

    // And add a day to get to tomorrow.
    let tomorrow = today.checked_add(1.day()).unwrap();
    let start_of_tomorrow = tomorrow.intz("UTC")?;

    // Tomorrow should start at 05:00 in UTC.
    assert_eq!(
        start_of_tomorrow.to_string(),
        "2024-11-04T05:00:00+00:00[UTC]"
    );

    Ok(())
}
