#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn canvaspattern_set_transform(instance: DOMReference, set_transform: i32);
}

pub fn set_transform(instance: DOMReference, matrix: i32) {
    unsafe { canvaspattern_set_transform(instance, matrix) }
}
