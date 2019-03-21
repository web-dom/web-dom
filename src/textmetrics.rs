#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn textmetrics_get_width(instance: DOMReference) -> i32;
    fn textmetrics_set_width(instance: DOMReference, value: i32);
}

pub fn get_width(instance: DOMReference) -> i32 {
    unsafe { textmetrics_get_width(instance) }
}

pub fn set_width(instance: DOMReference, value: i32) {
    unsafe {
        textmetrics_set_width(instance, value);
    }
}
