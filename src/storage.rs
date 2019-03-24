#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn storage_get_length(instance: DOMReference) -> f32;
    fn storage_set_length(instance: DOMReference, value: f32);
}

pub fn get_length(instance: DOMReference) -> f32 {
    unsafe { storage_get_length(instance) }
}

pub fn set_length(instance: DOMReference, value: f32) {
    unsafe {
        storage_set_length(instance, value);
    }
}
extern "C" {
    fn storage_key(instance: DOMReference, key: f32) -> CString;
}

pub fn key(instance: DOMReference, index: f32) -> String {
    unsafe { to_string(storage_key(instance, index)) }
}
extern "C" {
    fn storage_get_item(instance: DOMReference, get_item: CString) -> CString;
}

pub fn get_item(instance: DOMReference, key: &str) -> String {
    unsafe { to_string(storage_get_item(instance, to_cstring(key))) }
}
extern "C" {
    fn storage_set_item(instance: DOMReference, set_item: CString, set_item: CString);
}

pub fn set_item(instance: DOMReference, key: &str, value: &str) {
    unsafe { storage_set_item(instance, to_cstring(key), to_cstring(value)) }
}
extern "C" {
    fn storage_remove_item(instance: DOMReference, remove_item: CString);
}

pub fn remove_item(instance: DOMReference, key: &str) {
    unsafe { storage_remove_item(instance, to_cstring(key)) }
}
extern "C" {
    fn storage_clear(instance: DOMReference);
}

pub fn clear(instance: DOMReference) {
    unsafe { storage_clear(instance) }
}
extern "C" {
    fn storage_get_is_session_only(instance: DOMReference) -> i32;
    fn storage_set_is_session_only(instance: DOMReference, value: i32);
}

pub fn get_is_session_only(instance: DOMReference) -> bool {
    unsafe { 0 != storage_get_is_session_only(instance) }
}

pub fn set_is_session_only(instance: DOMReference, value: bool) {
    unsafe {
        storage_set_is_session_only(instance, if value == true { 1 } else { 0 });
    }
}
