use chrono::{DateTime, Utc, Duration};
use std::ops::Add;

const GIGASECOND:i64 = 1_000_000_000;

pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    start.add(Duration::seconds(GIGASECOND))
}
