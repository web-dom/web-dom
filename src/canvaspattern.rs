#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn canvaspattern_set_transform(instance: i32, matrix: i32);
}

pub fn set_transform(instance: i32, matrix: i32) {
    unsafe { canvaspattern_set_transform(instance, matrix) }
}
