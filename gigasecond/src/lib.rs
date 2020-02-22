use chrono::{DateTime, Utc};

const GIGASECONDS: i64 = 1000000000;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let ts = start.naive_utc().timestamp() + GIGASECONDS;
    let nt = chrono::NaiveDateTime::from_timestamp(ts, 0);
    let res = chrono::DateTime::from_utc(nt, *start.offset());
    res
}
