#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use alloc::string::String;
extern "C" {
    fn canvasgradient_add_color_stop(
        instance: DOMReference,
        add_color_stop: f32,
        add_color_stop: CString,
    );
}

pub fn add_color_stop(instance: DOMReference, offset: f32, color: &str) {
    unsafe { canvasgradient_add_color_stop(instance, offset, to_cstring(color)) }
}
