#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn keyboardevent_get_charCode(instance: i32) -> i32;
    fn keyboardevent_set_charCode(instance: i32, value: i32);
}

pub fn get_char_code(instance: i32) -> i32 {
    unsafe { keyboardevent_get_charCode(instance) }
}

pub fn set_char_code(instance: i32, value: i32) {
    unsafe {
        keyboardevent_set_charCode(instance, value);
    }
}
extern "C" {
    fn keyboardevent_get_keyCode(instance: i32) -> i32;
    fn keyboardevent_set_keyCode(instance: i32, value: i32);
}

pub fn get_key_code(instance: i32) -> i32 {
    unsafe { keyboardevent_get_keyCode(instance) }
}

pub fn set_key_code(instance: i32, value: i32) {
    unsafe {
        keyboardevent_set_keyCode(instance, value);
    }
}
extern "C" {
    fn keyboardevent_get_altKey(instance: i32) -> i32;
    fn keyboardevent_set_altKey(instance: i32, value: i32);
}

pub fn get_alt_key(instance: i32) -> i32 {
    unsafe { keyboardevent_get_altKey(instance) }
}

pub fn set_alt_key(instance: i32, value: i32) {
    unsafe {
        keyboardevent_set_altKey(instance, value);
    }
}
extern "C" {
    fn keyboardevent_get_ctrlKey(instance: i32) -> i32;
    fn keyboardevent_set_ctrlKey(instance: i32, value: i32);
}

pub fn get_ctrl_key(instance: i32) -> i32 {
    unsafe { keyboardevent_get_ctrlKey(instance) }
}

pub fn set_ctrl_key(instance: i32, value: i32) {
    unsafe {
        keyboardevent_set_ctrlKey(instance, value);
    }
}
extern "C" {
    fn keyboardevent_get_shiftKey(instance: i32) -> i32;
    fn keyboardevent_set_shiftKey(instance: i32, value: i32);
}

pub fn get_shift_key(instance: i32) -> i32 {
    unsafe { keyboardevent_get_shiftKey(instance) }
}

pub fn set_shift_key(instance: i32, value: i32) {
    unsafe {
        keyboardevent_set_shiftKey(instance, value);
    }
}
extern "C" {
    fn keyboardevent_get_metaKey(instance: i32) -> i32;
    fn keyboardevent_set_metaKey(instance: i32, value: i32);
}

pub fn get_meta_key(instance: i32) -> i32 {
    unsafe { keyboardevent_get_metaKey(instance) }
}

pub fn set_meta_key(instance: i32, value: i32) {
    unsafe {
        keyboardevent_set_metaKey(instance, value);
    }
}
extern "C" {
    fn keyboardevent_get_modifier_state(instance: i32, key: CString) -> i32;
}

pub fn get_modifier_state(instance: i32, key: &str) -> i32 {
    unsafe { keyboardevent_get_modifier_state(instance, cstr(key)) }
}
extern "C" {
    fn keyboardevent_get_location(instance: i32) -> i32;
    fn keyboardevent_set_location(instance: i32, value: i32);
}

pub fn get_location(instance: i32) -> i32 {
    unsafe { keyboardevent_get_location(instance) }
}

pub fn set_location(instance: i32, value: i32) {
    unsafe {
        keyboardevent_set_location(instance, value);
    }
}
extern "C" {
    fn keyboardevent_get_repeat(instance: i32) -> i32;
    fn keyboardevent_set_repeat(instance: i32, value: i32);
}

pub fn get_repeat(instance: i32) -> i32 {
    unsafe { keyboardevent_get_repeat(instance) }
}

pub fn set_repeat(instance: i32, value: i32) {
    unsafe {
        keyboardevent_set_repeat(instance, value);
    }
}
extern "C" {
    fn keyboardevent_get_isComposing(instance: i32) -> i32;
    fn keyboardevent_set_isComposing(instance: i32, value: i32);
}

pub fn get_is_composing(instance: i32) -> i32 {
    unsafe { keyboardevent_get_isComposing(instance) }
}

pub fn set_is_composing(instance: i32, value: i32) {
    unsafe {
        keyboardevent_set_isComposing(instance, value);
    }
}
extern "C" {
    fn keyboardevent_get_key(instance: i32) -> CString;
    fn keyboardevent_set_key(instance: i32, value: CString);
}

pub fn get_key(instance: i32) -> String {
    unsafe { cstr_to_string(keyboardevent_get_key(instance)) }
}

pub fn set_key(instance: i32, value: &str) {
    unsafe {
        keyboardevent_set_key(instance, cstr(value));
    }
}
extern "C" {
    fn keyboardevent_get_code(instance: i32) -> CString;
    fn keyboardevent_set_code(instance: i32, value: CString);
}

pub fn get_code(instance: i32) -> String {
    unsafe { cstr_to_string(keyboardevent_get_code(instance)) }
}

pub fn set_code(instance: i32, value: &str) {
    unsafe {
        keyboardevent_set_code(instance, cstr(value));
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
            cstr(type_arg),
            bubbles_arg,
            cancelable_arg,
            view_arg,
            cstr(key_arg),
            location_arg,
            ctrl_key,
            alt_key,
            shift_key,
            meta_key,
        )
    }
}
extern "C" {
    fn keyboardevent_get_initDict(instance: i32) -> i32;
    fn keyboardevent_set_initDict(instance: i32, value: i32);
}

pub fn get_init_dict(instance: i32) -> i32 {
    unsafe { keyboardevent_get_initDict(instance) }
}

pub fn set_init_dict(instance: i32, value: i32) {
    unsafe {
        keyboardevent_set_initDict(instance, value);
    }
}
