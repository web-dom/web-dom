#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn keyboardevent_get_char_code(instance: DOMReference) -> f32;
    fn keyboardevent_set_char_code(instance: DOMReference, value: f32);
}

pub fn get_char_code(instance: DOMReference) -> f32 {
    unsafe { keyboardevent_get_char_code(instance) }
}

pub fn set_char_code(instance: DOMReference, value: f32) {
    unsafe {
        keyboardevent_set_char_code(instance, value);
    }
}
extern "C" {
    fn keyboardevent_get_key_code(instance: DOMReference) -> f32;
    fn keyboardevent_set_key_code(instance: DOMReference, value: f32);
}

pub fn get_key_code(instance: DOMReference) -> f32 {
    unsafe { keyboardevent_get_key_code(instance) }
}

pub fn set_key_code(instance: DOMReference, value: f32) {
    unsafe {
        keyboardevent_set_key_code(instance, value);
    }
}
extern "C" {
    fn keyboardevent_get_alt_key(instance: DOMReference) -> i32;
    fn keyboardevent_set_alt_key(instance: DOMReference, value: i32);
}

pub fn get_alt_key(instance: DOMReference) -> bool {
    unsafe { 0 != keyboardevent_get_alt_key(instance) }
}

pub fn set_alt_key(instance: DOMReference, value: bool) {
    unsafe {
        keyboardevent_set_alt_key(instance, if value == true { 1 } else { 0 });
    }
}
extern "C" {
    fn keyboardevent_get_ctrl_key(instance: DOMReference) -> i32;
    fn keyboardevent_set_ctrl_key(instance: DOMReference, value: i32);
}

pub fn get_ctrl_key(instance: DOMReference) -> bool {
    unsafe { 0 != keyboardevent_get_ctrl_key(instance) }
}

pub fn set_ctrl_key(instance: DOMReference, value: bool) {
    unsafe {
        keyboardevent_set_ctrl_key(instance, if value == true { 1 } else { 0 });
    }
}
extern "C" {
    fn keyboardevent_get_shift_key(instance: DOMReference) -> i32;
    fn keyboardevent_set_shift_key(instance: DOMReference, value: i32);
}

pub fn get_shift_key(instance: DOMReference) -> bool {
    unsafe { 0 != keyboardevent_get_shift_key(instance) }
}

pub fn set_shift_key(instance: DOMReference, value: bool) {
    unsafe {
        keyboardevent_set_shift_key(instance, if value == true { 1 } else { 0 });
    }
}
extern "C" {
    fn keyboardevent_get_meta_key(instance: DOMReference) -> i32;
    fn keyboardevent_set_meta_key(instance: DOMReference, value: i32);
}

pub fn get_meta_key(instance: DOMReference) -> bool {
    unsafe { 0 != keyboardevent_get_meta_key(instance) }
}

pub fn set_meta_key(instance: DOMReference, value: bool) {
    unsafe {
        keyboardevent_set_meta_key(instance, if value == true { 1 } else { 0 });
    }
}
extern "C" {
    fn keyboardevent_get_modifier_state(instance: DOMReference, get_modifier_state: CString)
        -> i32;
}

pub fn get_modifier_state(instance: DOMReference, key: &str) -> bool {
    unsafe { 0 != keyboardevent_get_modifier_state(instance, to_cstring(key)) }
}
extern "C" {
    fn keyboardevent_get_location(instance: DOMReference) -> f32;
    fn keyboardevent_set_location(instance: DOMReference, value: f32);
}

pub fn get_location(instance: DOMReference) -> f32 {
    unsafe { keyboardevent_get_location(instance) }
}

pub fn set_location(instance: DOMReference, value: f32) {
    unsafe {
        keyboardevent_set_location(instance, value);
    }
}
extern "C" {
    fn keyboardevent_get_repeat(instance: DOMReference) -> i32;
    fn keyboardevent_set_repeat(instance: DOMReference, value: i32);
}

pub fn get_repeat(instance: DOMReference) -> bool {
    unsafe { 0 != keyboardevent_get_repeat(instance) }
}

pub fn set_repeat(instance: DOMReference, value: bool) {
    unsafe {
        keyboardevent_set_repeat(instance, if value == true { 1 } else { 0 });
    }
}
extern "C" {
    fn keyboardevent_get_is_composing(instance: DOMReference) -> i32;
    fn keyboardevent_set_is_composing(instance: DOMReference, value: i32);
}

pub fn get_is_composing(instance: DOMReference) -> bool {
    unsafe { 0 != keyboardevent_get_is_composing(instance) }
}

pub fn set_is_composing(instance: DOMReference, value: bool) {
    unsafe {
        keyboardevent_set_is_composing(instance, if value == true { 1 } else { 0 });
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
        instance: DOMReference,
        init_keyboard_event: CString,
        init_keyboard_event: i32,
        init_keyboard_event: i32,
        init_keyboard_event: i32,
        init_keyboard_event: CString,
        init_keyboard_event: f32,
        init_keyboard_event: i32,
        init_keyboard_event: i32,
        init_keyboard_event: i32,
        init_keyboard_event: i32,
    );
}

pub fn init_keyboard_event(
    instance: DOMReference,
    type_arg: &str,
    bubbles_arg: bool,
    cancelable_arg: bool,
    view_arg: i32,
    key_arg: &str,
    location_arg: f32,
    ctrl_key: bool,
    alt_key: bool,
    shift_key: bool,
    meta_key: bool,
) {
    unsafe {
        keyboardevent_init_keyboard_event(
            instance,
            to_cstring(type_arg),
            if bubbles_arg { 1 } else { 0 },
            if cancelable_arg { 1 } else { 0 },
            view_arg,
            to_cstring(key_arg),
            location_arg,
            if ctrl_key { 1 } else { 0 },
            if alt_key { 1 } else { 0 },
            if shift_key { 1 } else { 0 },
            if meta_key { 1 } else { 0 },
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
