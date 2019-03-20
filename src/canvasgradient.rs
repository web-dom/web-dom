#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn canvasgradient_add_color_stop(instance: i32, offset: i32, color: CString);
}

pub fn add_color_stop(instance: i32, offset: i32, color: &str) {
    unsafe { canvasgradient_add_color_stop(instance, offset, to_cstring(color)) }
}
