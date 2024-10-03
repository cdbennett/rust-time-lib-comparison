//! Get the beginning of tomorrow.
//! To make it interesting, do this around a DST change.

use chrono::{prelude::*, Days};

fn main() {
    let now = Utc::now();
    let today = now.date_naive();
    let tomorrow = today.checked_add_days(Days::new(1)).unwrap();
    let start_of_tomorrow = tomorrow.and_time(NaiveTime::default());
    let start_of_tomorrow = Utc.from_utc_datetime(&start_of_tomorrow);
    println!("The start of tomorrow in UTC is {start_of_tomorrow}");
}
