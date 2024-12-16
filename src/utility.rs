use std::time::{SystemTime, UNIX_EPOCH};

pub fn now_in_timescale(timescale: i64) -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("A ripple in the space-time continuum was detected")
        .as_millis() as i64
        * timescale
        / 1000
}
