use chrono::{DateTime, Utc};

const GIGASECONDS: i64 = 1_000_000_000;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    start + chrono::Duration::seconds(GIGASECONDS)
}
