#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use alloc::string::String;
extern "C" {
    fn canvaspattern_set_transform(instance: DOMReference, set_transform: DOMReference);
}

pub fn set_transform(instance: DOMReference, matrix: DOMReference) {
    unsafe { canvaspattern_set_transform(instance, matrix) }
}
