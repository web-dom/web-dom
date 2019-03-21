#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn keyboardevent_get_char_code(instance: DOMReference) -> i32;
    fn keyboardevent_set_char_code(instance: DOMReference, value: i32);
}

pub fn get_char_code(instance: DOMReference) -> i32 {
    unsafe { keyboardevent_get_char_code(instance) }
}

pub fn set_char_code(instance: DOMReference, value: i32) {
    unsafe {
        keyboardevent_set_char_code(instance, value);
    }
}
extern "C" {
    fn keyboardevent_get_key_code(instance: DOMReference) -> i32;
    fn keyboardevent_set_key_code(instance: DOMReference, value: i32);
}

pub fn get_key_code(instance: DOMReference) -> i32 {
    unsafe { keyboardevent_get_key_code(instance) }
}

pub fn set_key_code(instance: DOMReference, value: i32) {
    unsafe {
        keyboardevent_set_key_code(instance, value);
    }
}
extern "C" {
    fn keyboardevent_get_alt_key(instance: DOMReference) -> i32;
    fn keyboardevent_set_alt_key(instance: DOMReference, value: i32);
}

pub fn get_alt_key(instance: DOMReference) -> i32 {
    unsafe { keyboardevent_get_alt_key(instance) }
}

pub fn set_alt_key(instance: DOMReference, value: i32) {
    unsafe {
        keyboardevent_set_alt_key(instance, value);
    }
}
extern "C" {
    fn keyboardevent_get_ctrl_key(instance: DOMReference) -> i32;
    fn keyboardevent_set_ctrl_key(instance: DOMReference, value: i32);
}

pub fn get_ctrl_key(instance: DOMReference) -> i32 {
    unsafe { keyboardevent_get_ctrl_key(instance) }
}

pub fn set_ctrl_key(instance: DOMReference, value: i32) {
    unsafe {
        keyboardevent_set_ctrl_key(instance, value);
    }
}
extern "C" {
    fn keyboardevent_get_shift_key(instance: DOMReference) -> i32;
    fn keyboardevent_set_shift_key(instance: DOMReference, value: i32);
}

pub fn get_shift_key(instance: DOMReference) -> i32 {
    unsafe { keyboardevent_get_shift_key(instance) }
}

pub fn set_shift_key(instance: DOMReference, value: i32) {
    unsafe {
        keyboardevent_set_shift_key(instance, value);
    }
}
extern "C" {
    fn keyboardevent_get_meta_key(instance: DOMReference) -> i32;
    fn keyboardevent_set_meta_key(instance: DOMReference, value: i32);
}

pub fn get_meta_key(instance: DOMReference) -> i32 {
    unsafe { keyboardevent_get_meta_key(instance) }
}

pub fn set_meta_key(instance: DOMReference, value: i32) {
    unsafe {
        keyboardevent_set_meta_key(instance, value);
    }
}
extern "C" {
    fn keyboardevent_get_modifier_state(instance: i32, key: CString) -> i32;
}

pub fn get_modifier_state(instance: i32, key: &str) -> i32 {
    unsafe { keyboardevent_get_modifier_state(instance, to_cstring(key)) }
}
extern "C" {
    fn keyboardevent_get_location(instance: DOMReference) -> i32;
    fn keyboardevent_set_location(instance: DOMReference, value: i32);
}

pub fn get_location(instance: DOMReference) -> i32 {
    unsafe { keyboardevent_get_location(instance) }
}

pub fn set_location(instance: DOMReference, value: i32) {
    unsafe {
        keyboardevent_set_location(instance, value);
    }
}
extern "C" {
    fn keyboardevent_get_repeat(instance: DOMReference) -> i32;
    fn keyboardevent_set_repeat(instance: DOMReference, value: i32);
}

pub fn get_repeat(instance: DOMReference) -> i32 {
    unsafe { keyboardevent_get_repeat(instance) }
}

pub fn set_repeat(instance: DOMReference, value: i32) {
    unsafe {
        keyboardevent_set_repeat(instance, value);
    }
}
extern "C" {
    fn keyboardevent_get_is_composing(instance: DOMReference) -> i32;
    fn keyboardevent_set_is_composing(instance: DOMReference, value: i32);
}

pub fn get_is_composing(instance: DOMReference) -> i32 {
    unsafe { keyboardevent_get_is_composing(instance) }
}

pub fn set_is_composing(instance: DOMReference, value: i32) {
    unsafe {
        keyboardevent_set_is_composing(instance, value);
    }
}
extern "C" {
    fn keyboardevent_get_key(instance: DOMReference) -> CString;
    fn keyboardevent_set_key(instance: DOMReference, value: CString);
}

pub fn get_key(instance: DOMReference) -> String {
    unsafe { to_string(keyboardevent_get_key(instance)) }
}

pub fn set_key(instance: DOMReference, value: &str) {
    unsafe {
        keyboardevent_set_key(instance, to_cstring(value));
    }
}
extern "C" {
    fn keyboardevent_get_code(instance: DOMReference) -> CString;
    fn keyboardevent_set_code(instance: DOMReference, value: CString);
}

pub fn get_code(instance: DOMReference) -> String {
    unsafe { to_string(keyboardevent_get_code(instance)) }
}

pub fn set_code(instance: DOMReference, value: &str) {
    unsafe {
        keyboardevent_set_code(instance, to_cstring(value));
    }
}
extern "C" {
    fn keyboardevent_init_keyboard_event(
        instance: i32,
        type_arg: CString,
        bubbles_arg: i32,
        cancelable_arg: i32,
        view_arg: i32,
        key_arg: CString,
        location_arg: i32,
        ctrl_key: i32,
        alt_key: i32,
        shift_key: i32,
        meta_key: i32,
    );
}

pub fn init_keyboard_event(
    instance: i32,
    type_arg: &str,
    bubbles_arg: i32,
    cancelable_arg: i32,
    view_arg: i32,
    key_arg: &str,
    location_arg: i32,
    ctrl_key: i32,
    alt_key: i32,
    shift_key: i32,
    meta_key: i32,
) {
    unsafe {
        keyboardevent_init_keyboard_event(
            instance,
            to_cstring(type_arg),
            bubbles_arg,
            cancelable_arg,
            view_arg,
            to_cstring(key_arg),
            location_arg,
            ctrl_key,
            alt_key,
            shift_key,
            meta_key,
        )
    }
}
extern "C" {
    fn keyboardevent_get_init_dict(instance: DOMReference) -> i32;
    fn keyboardevent_set_init_dict(instance: DOMReference, value: i32);
}

pub fn get_init_dict(instance: DOMReference) -> i32 {
    unsafe { keyboardevent_get_init_dict(instance) }
}

pub fn set_init_dict(instance: DOMReference, value: i32) {
    unsafe {
        keyboardevent_set_init_dict(instance, value);
    }
}
