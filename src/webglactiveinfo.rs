#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn webglactiveinfo_get_size(instance: DOMReference) -> DOMReference;
    fn webglactiveinfo_set_size(instance: DOMReference, value: DOMReference);
}

pub fn get_size(instance: DOMReference) -> DOMReference {
    unsafe { webglactiveinfo_get_size(instance) }
}

pub fn set_size(instance: DOMReference, value: DOMReference) {
    unsafe {
        webglactiveinfo_set_size(instance, value);
    }
}
extern "C" {
    fn webglactiveinfo_get_type(instance: DOMReference) -> DOMReference;
    fn webglactiveinfo_set_type(instance: DOMReference, value: DOMReference);
}

pub fn get_type(instance: DOMReference) -> DOMReference {
    unsafe { webglactiveinfo_get_type(instance) }
}

pub fn set_type(instance: DOMReference, value: DOMReference) {
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
