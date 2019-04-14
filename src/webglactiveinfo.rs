#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use alloc::string::String;
extern "C" {
    fn webglactiveinfo_get_size(instance: DOMReference) -> f32;
    fn webglactiveinfo_set_size(instance: DOMReference, value: f32);
}

pub fn get_size(instance: DOMReference) -> f32 {
    unsafe { webglactiveinfo_get_size(instance) }
}

pub fn set_size(instance: DOMReference, value: f32) {
    unsafe {
        webglactiveinfo_set_size(instance, value);
    }
}
extern "C" {
    fn webglactiveinfo_get_type(instance: DOMReference) -> f32;
    fn webglactiveinfo_set_type(instance: DOMReference, value: f32);
}

pub fn get_type(instance: DOMReference) -> f32 {
    unsafe { webglactiveinfo_get_type(instance) }
}

pub fn set_type(instance: DOMReference, value: f32) {
    unsafe {
        webglactiveinfo_set_type(instance, value);
    }
}
extern "C" {
    fn webglactiveinfo_get_name(instance: DOMReference) -> CString;
    fn webglactiveinfo_set_name(instance: DOMReference, value: CString);
}

pub fn get_name(instance: DOMReference) -> String {
    unsafe { to_string(webglactiveinfo_get_name(instance)) }
}

pub fn set_name(instance: DOMReference, value: &str) {
    unsafe {
        webglactiveinfo_set_name(instance, to_cstring(value));
    }
}
