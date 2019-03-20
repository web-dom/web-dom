#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn path2d_add_path(instance: i32, path: i32, transformation: i32);
}

pub fn add_path(instance: i32, path: i32, transformation: i32) {
    unsafe { path2d_add_path(instance, path, transformation) }
}
