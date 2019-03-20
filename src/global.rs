use std::ffi::CStr;
use std::os::raw::c_void;

pub type CString = i32;
pub type Element = i32;
pub type EventListener = i32;
pub type Event = i32;

pub fn cstr(s: &str) -> CString {
    std::ffi::CString::new(s).unwrap().into_raw() as i32
}

pub fn cstr_to_string(c: CString) -> String {
    let s: &CStr = unsafe { CStr::from_ptr(c as *const i8) };
    s.to_str().unwrap().to_owned()
}

extern "C" {
    fn global_debugger();
    fn global_get_window() -> Element;
    fn global_release(handle: i32);
    fn global_create_event_listener() -> EventListener;
    fn global_get_property(element: Element, property_name: CString) -> CString;
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

pub fn get_property(element: Element, property_name: &str) -> String {
    unsafe { cstr_to_string(global_get_property(element, cstr(property_name))) }
}

#[no_mangle]
pub fn malloc(size: i32) -> *mut c_void {
    let mut buf = Vec::with_capacity(size as usize);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    return ptr as *mut c_void;
}
