# Rust Time Libraries

Compare performing common time and date operations using the various Rust time crates.

## Test Suite

Some common tasks we would like to compare:

- Parsing: Read lines from stdin consisting of an ISO timestamp and parse them
- Formatting: Write the current time in UTC time zone, ISO 8601 format
- Serializing: serialize to JSON
- Timezones: Write the current time in local time zone
- Arithmetic: Calculate the difference between two times
- Manipulation: Find the beginning of the next day

## Crates

- Time: not generic WRT timezone. But cumbersome to parse. Serializes to non-standard format.

- Chrono: generic WRT timezone, which is clunky. Parsing seems overly strict,
  not accepting timezone offsets for RFC 3339 parsing.

- Hifitime: scientific time measurement, serialized to non-standard format

- Jiff: seems to resolve all the issues with the other time crates. Serializes to standard
  formats automatically. Handles manipulation of date/time values in a time zone properly
  even when around a DST transition. Building on TC39/Temporal.js means it has a solid
  foundation and knowledge can transfer to/from the JS ecosystem.
