#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn path2d_add_path(instance: DOMReference, add_path: i32, add_path: i32);
}

pub fn add_path(instance: DOMReference, path: i32, transformation: i32) {
    unsafe { path2d_add_path(instance, path, transformation) }
}
