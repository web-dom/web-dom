#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use alloc::string::String;
extern "C" {
    fn eventtarget_add_event_listener(
        instance: DOMReference,
        add_event_listener: CString,
        add_event_listener: DOMReference,
    );
}

pub fn add_event_listener(instance: DOMReference, event_type: &str, listener: DOMReference) {
    unsafe { eventtarget_add_event_listener(instance, to_cstring(event_type), listener) }
}
extern "C" {
    fn eventtarget_remove_event_listener(
        instance: DOMReference,
        remove_event_listener: CString,
        remove_event_listener: DOMReference,
    );
}

pub fn remove_event_listener(instance: DOMReference, event_type: &str, listener: DOMReference) {
    unsafe { eventtarget_remove_event_listener(instance, to_cstring(event_type), listener) }
}
extern "C" {
    fn eventtarget_dispatch_event(instance: DOMReference, dispatch_event: DOMReference) -> i32;
}

pub fn dispatch_event(instance: DOMReference, event: DOMReference) -> bool {
    unsafe { 0 != eventtarget_dispatch_event(instance, event) }
}
