use std::ffi::CStr;
use std::os::raw::c_void;

pub const NULL: i32 = 0;
pub type DOMReference = i32;
pub type CString = i32;
pub type Element = i32;
pub type EventListener = i32;
pub type Event = i32;

use std::mem::size_of;

pub trait IntoBytes {
    fn into_bytes(self) -> Vec<u8>;
}

impl<T> IntoBytes for Vec<T> {
    fn into_bytes(self) -> Vec<u8> {
        let length = size_of::<T>() * self.len();
        unsafe {
            let slice = self.into_boxed_slice();
            Vec::<u8>::from_raw_parts(Box::into_raw(slice) as _, length, length)
        }
    }
}

pub fn to_cstring(s: &str) -> CString {
    std::ffi::CString::new(s).unwrap().into_raw() as i32
}

pub fn to_string(c: CString) -> String {
    let s: &CStr = unsafe { CStr::from_ptr(c as *const i8) };
    s.to_str().unwrap().to_owned()
}

extern "C" {
    fn global_convert_ref_to_string(instance: i32) -> CString;
    fn global_convert_ref_to_bool(instance: i32) -> i32;
    fn global_create_f32array(start: i32, len: i32) -> DOMReference;
    fn global_create_uint8array(start: i32, len: i32) -> DOMReference;
    fn global_is_null() -> i32;
    fn global_debugger();
    fn global_get_window() -> Element;
    fn global_release(handle: i32);
    fn global_create_event_listener() -> EventListener;
    fn global_get_property(element: Element, property_name: CString) -> i32;
}

pub fn create_f32array(bytes: &[u8]) -> DOMReference {
    unsafe { global_create_f32array(bytes.as_ptr() as _, bytes.len() as _) }
}

pub fn create_uint8array(bytes: &[u8]) -> DOMReference {
    unsafe { global_create_uint8array(bytes.as_ptr() as _, bytes.len() as _) }
}

pub fn ref_to_string(instance: DOMReference) -> String {
    unsafe { to_string(global_convert_ref_to_string(instance)) }
}

pub fn ref_to_bool(instance: DOMReference) -> bool {
    unsafe { 0 != global_convert_ref_to_bool(instance) }
}

pub fn is_null() -> bool {
    unsafe { 0 != global_is_null() }
}

pub fn debugger() {
    unsafe {
        global_debugger();
    }
}

pub fn window() -> Element {
    unsafe { global_get_window() }
}

pub fn document() -> Element {
    crate::window::get_document(window())
}

pub fn release(handle: i32) {
    unsafe { global_release(handle) }
}

pub fn create_event_listener() -> EventListener {
    unsafe { global_create_event_listener() }
}

pub fn get_property(element: Element, property_name: &str) -> i32 {
    unsafe { global_get_property(element, to_cstring(property_name)) }
}

#[no_mangle]
pub fn malloc(size: i32) -> *mut c_void {
    let mut buf = Vec::with_capacity(size as usize);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    return ptr as *mut c_void;
}
