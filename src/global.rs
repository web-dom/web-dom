use std::ffi::CStr;

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
pub fn malloc(_len: i32) -> i32 {
    return 0;
}
