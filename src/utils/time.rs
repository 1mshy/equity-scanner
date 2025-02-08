use std::time::{SystemTime, UNIX_EPOCH};

/// Gets current unix time in seconds
pub fn current_unix_time() -> u128 {
    let start = SystemTime::now();
    start.duration_since(UNIX_EPOCH).expect("msg").as_millis()
}
