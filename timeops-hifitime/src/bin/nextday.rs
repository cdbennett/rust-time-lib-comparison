//! Get the beginning of tomorrow.

use hifitime::prelude::*;

fn main() {
    let now = Epoch::now().unwrap();
    let (year, month, day, _, _, _, _) = now.to_gregorian_utc();
    let _today = Epoch::from_gregorian_utc_at_midnight(year, month, day);
    println!(
        "does not seem like hifitime has the calendar features needed to add a true calendar day"
    );
    // ... does not seem like hifitime has the calendar features needed to add a true calendar day ...
    // let tomorrow = today.next_day().unwrap();
    // let start_of_tomorrow = tomorrow.with_hms(0,0,0).unwrap();
    // let start_of_tomorrow = start_of_tomorrow.assume_utc();
    // println!("The start of tomorrow in UTC is {start_of_tomorrow}");
}
