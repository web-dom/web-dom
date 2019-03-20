extern "C" {
    fn date_now_seconds() -> i32;
    fn date_get_timezone_offset() -> i32;
}

pub fn now_seconds() -> i32 {
    unsafe { date_now_seconds() }
}

pub fn get_timezone_offset() -> i32 {
    unsafe { date_get_timezone_offset() }
}
