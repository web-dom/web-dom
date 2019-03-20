#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn eventtarget_add_event_listener(instance: i32, event_type: CString, listener: i32);
}

pub fn add_event_listener(instance: i32, event_type: &str, listener: i32) {
    unsafe { eventtarget_add_event_listener(instance, to_cstring(event_type), listener) }
}
extern "C" {
    fn eventtarget_remove_event_listener(instance: i32, event_type: CString, listener: i32);
}

pub fn remove_event_listener(instance: i32, event_type: &str, listener: i32) {
    unsafe { eventtarget_remove_event_listener(instance, to_cstring(event_type), listener) }
}
extern "C" {
    fn eventtarget_dispatch_event(instance: i32, event: i32) -> i32;
}

pub fn dispatch_event(instance: i32, event: i32) -> i32 {
    unsafe { eventtarget_dispatch_event(instance, event) }
}
