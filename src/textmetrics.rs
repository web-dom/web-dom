#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn textmetrics_get_width(instance: i32) -> i32;
    fn textmetrics_set_width(instance: i32, value: i32);
}

pub fn get_width(instance: i32) -> i32 {
    unsafe { textmetrics_get_width(instance) }
}

pub fn set_width(instance: i32, value: i32) {
    unsafe {
        textmetrics_set_width(instance, value);
    }
}
