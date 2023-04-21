use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_today_weekday() -> u8 {
    let now = SystemTime::now();
    let days_since_epoch = now.duration_since(UNIX_EPOCH).unwrap().as_secs() / 86400;
    ((days_since_epoch + 4) % 7) as u8
}
