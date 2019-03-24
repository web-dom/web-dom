#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn eventtarget_add_event_listener(
        instance: DOMReference,
        add_event_listener: CString,
        add_event_listener: i32,
    );
}

pub fn add_event_listener(instance: DOMReference, event_type: &str, listener: i32) {
    unsafe { eventtarget_add_event_listener(instance, to_cstring(event_type), listener) }
}
extern "C" {
    fn eventtarget_remove_event_listener(
        instance: DOMReference,
        remove_event_listener: CString,
        remove_event_listener: i32,
    );
}

pub fn remove_event_listener(instance: DOMReference, event_type: &str, listener: i32) {
    unsafe { eventtarget_remove_event_listener(instance, to_cstring(event_type), listener) }
}
extern "C" {
    fn eventtarget_dispatch_event(instance: DOMReference, dispatch_event: i32) -> i32;
}

pub fn dispatch_event(instance: DOMReference, event: i32) -> bool {
    unsafe { 0 != eventtarget_dispatch_event(instance, event) }
}
