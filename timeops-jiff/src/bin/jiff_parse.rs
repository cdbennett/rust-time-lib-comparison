//! Parsing test.
//! Jiff accepts either a "T" or space as the date-time separator.
//! Jiff requires timezone offset and/or timezone suffix
//! to parse a Zoned date/time.

use jiff::{civil::DateTime, Timestamp, Zoned};

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
    let mut errors = Vec::new();
    println!(
        "{:>3} {:<45} {:<45} {:<45} {:<45}",
        "#", "Input", "Timestamp", "Zoned", "Civil"
    );
    for (i, &input) in inputs.iter().enumerate() {
        test_parse(i, input, &mut errors);
    }
    println!("\nErrors:");
    for e in errors {
        println!("{e}");
    }
}

fn test_parse(i: usize, s: &str, errors: &mut Vec<String>) {
    let ts = match parse_timestamp(s) {
        Ok(s) => s,
        Err(e) => {
            errors.push(format!("[{i}] `{s}` can't parse as timestamp: {e}"));
            "-".to_owned()
        }
    };

    let zdt: String = match parse_zoned(s) {
        Ok(s) => s,
        Err(e) => {
            errors.push(format!("[{i}] `{s}` can't parse as zoned: {e}"));
            "-".to_owned()
        }
    };

    let cdt: String = match parse_civil(s) {
        Ok(s) => s,
        Err(e) => {
            errors.push(format!("[{i}] `{s}` can't parse as civil: {e}"));
            "-".to_owned()
        }
    };

    println!("{i:>3} {s:<45} {ts:<45} {zdt:<45} {cdt:<45}");
}

fn parse_timestamp(s: &str) -> Result<String, jiff::Error> {
    let t: Timestamp = s.parse()?;
    Ok(t.to_string())
}

fn parse_zoned(s: &str) -> Result<String, jiff::Error> {
    let t: Zoned = s.parse()?;
    Ok(t.to_string())
}

fn parse_civil(s: &str) -> Result<String, jiff::Error> {
    let t: DateTime = s.parse()?;
    Ok(t.to_string())
}
