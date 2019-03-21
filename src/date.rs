extern "C" {
    fn date_now() -> f64;
    fn date_get_timezone_offset() -> i32;
}

pub fn now() -> f64 {
    unsafe { date_now() }
}

pub fn get_timezone_offset() -> i32 {
    unsafe { date_get_timezone_offset() }
}
