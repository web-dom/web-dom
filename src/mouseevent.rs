#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn mouseevent_get_screen_x(instance: i32) -> i32;
    fn mouseevent_set_screen_x(instance: i32, value: i32);
}

pub fn get_screen_x(instance: i32) -> i32 {
    unsafe { mouseevent_get_screen_x(instance) }
}

pub fn set_screen_x(instance: i32, value: i32) {
    unsafe {
        mouseevent_set_screen_x(instance, value);
    }
}
extern "C" {
    fn mouseevent_get_screen_y(instance: i32) -> i32;
    fn mouseevent_set_screen_y(instance: i32, value: i32);
}

pub fn get_screen_y(instance: i32) -> i32 {
    unsafe { mouseevent_get_screen_y(instance) }
}

pub fn set_screen_y(instance: i32, value: i32) {
    unsafe {
        mouseevent_set_screen_y(instance, value);
    }
}
extern "C" {
    fn mouseevent_get_client_x(instance: i32) -> i32;
    fn mouseevent_set_client_x(instance: i32, value: i32);
}

pub fn get_client_x(instance: i32) -> i32 {
    unsafe { mouseevent_get_client_x(instance) }
}

pub fn set_client_x(instance: i32, value: i32) {
    unsafe {
        mouseevent_set_client_x(instance, value);
    }
}
extern "C" {
    fn mouseevent_get_client_y(instance: i32) -> i32;
    fn mouseevent_set_client_y(instance: i32, value: i32);
}

pub fn get_client_y(instance: i32) -> i32 {
    unsafe { mouseevent_get_client_y(instance) }
}

pub fn set_client_y(instance: i32, value: i32) {
    unsafe {
        mouseevent_set_client_y(instance, value);
    }
}
extern "C" {
    fn mouseevent_get_x(instance: i32) -> i32;
    fn mouseevent_set_x(instance: i32, value: i32);
}

pub fn get_x(instance: i32) -> i32 {
    unsafe { mouseevent_get_x(instance) }
}

pub fn set_x(instance: i32, value: i32) {
    unsafe {
        mouseevent_set_x(instance, value);
    }
}
extern "C" {
    fn mouseevent_get_y(instance: i32) -> i32;
    fn mouseevent_set_y(instance: i32, value: i32);
}

pub fn get_y(instance: i32) -> i32 {
    unsafe { mouseevent_get_y(instance) }
}

pub fn set_y(instance: i32, value: i32) {
    unsafe {
        mouseevent_set_y(instance, value);
    }
}
extern "C" {
    fn mouseevent_get_offset_x(instance: i32) -> i32;
    fn mouseevent_set_offset_x(instance: i32, value: i32);
}

pub fn get_offset_x(instance: i32) -> i32 {
    unsafe { mouseevent_get_offset_x(instance) }
}

pub fn set_offset_x(instance: i32, value: i32) {
    unsafe {
        mouseevent_set_offset_x(instance, value);
    }
}
extern "C" {
    fn mouseevent_get_offset_y(instance: i32) -> i32;
    fn mouseevent_set_offset_y(instance: i32, value: i32);
}

pub fn get_offset_y(instance: i32) -> i32 {
    unsafe { mouseevent_get_offset_y(instance) }
}

pub fn set_offset_y(instance: i32, value: i32) {
    unsafe {
        mouseevent_set_offset_y(instance, value);
    }
}
extern "C" {
    fn mouseevent_get_ctrl_key(instance: i32) -> i32;
    fn mouseevent_set_ctrl_key(instance: i32, value: i32);
}

pub fn get_ctrl_key(instance: i32) -> i32 {
    unsafe { mouseevent_get_ctrl_key(instance) }
}

pub fn set_ctrl_key(instance: i32, value: i32) {
    unsafe {
        mouseevent_set_ctrl_key(instance, value);
    }
}
extern "C" {
    fn mouseevent_get_shift_key(instance: i32) -> i32;
    fn mouseevent_set_shift_key(instance: i32, value: i32);
}

pub fn get_shift_key(instance: i32) -> i32 {
    unsafe { mouseevent_get_shift_key(instance) }
}

pub fn set_shift_key(instance: i32, value: i32) {
    unsafe {
        mouseevent_set_shift_key(instance, value);
    }
}
extern "C" {
    fn mouseevent_get_alt_key(instance: i32) -> i32;
    fn mouseevent_set_alt_key(instance: i32, value: i32);
}

pub fn get_alt_key(instance: i32) -> i32 {
    unsafe { mouseevent_get_alt_key(instance) }
}

pub fn set_alt_key(instance: i32, value: i32) {
    unsafe {
        mouseevent_set_alt_key(instance, value);
    }
}
extern "C" {
    fn mouseevent_get_meta_key(instance: i32) -> i32;
    fn mouseevent_set_meta_key(instance: i32, value: i32);
}

pub fn get_meta_key(instance: i32) -> i32 {
    unsafe { mouseevent_get_meta_key(instance) }
}

pub fn set_meta_key(instance: i32, value: i32) {
    unsafe {
        mouseevent_set_meta_key(instance, value);
    }
}
extern "C" {
    fn mouseevent_get_button(instance: i32) -> i32;
    fn mouseevent_set_button(instance: i32, value: i32);
}

pub fn get_button(instance: i32) -> i32 {
    unsafe { mouseevent_get_button(instance) }
}

pub fn set_button(instance: i32, value: i32) {
    unsafe {
        mouseevent_set_button(instance, value);
    }
}
extern "C" {
    fn mouseevent_get_buttons(instance: i32) -> i32;
    fn mouseevent_set_buttons(instance: i32, value: i32);
}

pub fn get_buttons(instance: i32) -> i32 {
    unsafe { mouseevent_get_buttons(instance) }
}

pub fn set_buttons(instance: i32, value: i32) {
    unsafe {
        mouseevent_set_buttons(instance, value);
    }
}
extern "C" {
    fn mouseevent_get_related_target(instance: i32) -> i32;
    fn mouseevent_set_related_target(instance: i32, value: i32);
}

pub fn get_related_target(instance: i32) -> i32 {
    unsafe { mouseevent_get_related_target(instance) }
}

pub fn set_related_target(instance: i32, value: i32) {
    unsafe {
        mouseevent_set_related_target(instance, value);
    }
}
extern "C" {
    fn mouseevent_get_region(instance: i32) -> CString;
    fn mouseevent_set_region(instance: i32, value: CString);
}

pub fn get_region(instance: i32) -> String {
    unsafe { cstr_to_string(mouseevent_get_region(instance)) }
}

pub fn set_region(instance: i32, value: &str) {
    unsafe {
        mouseevent_set_region(instance, cstr(value));
    }
}
extern "C" {
    fn mouseevent_get_movement_x(instance: i32) -> i32;
    fn mouseevent_set_movement_x(instance: i32, value: i32);
}

pub fn get_movement_x(instance: i32) -> i32 {
    unsafe { mouseevent_get_movement_x(instance) }
}

pub fn set_movement_x(instance: i32, value: i32) {
    unsafe {
        mouseevent_set_movement_x(instance, value);
    }
}
extern "C" {
    fn mouseevent_get_movement_y(instance: i32) -> i32;
    fn mouseevent_set_movement_y(instance: i32, value: i32);
}

pub fn get_movement_y(instance: i32) -> i32 {
    unsafe { mouseevent_get_movement_y(instance) }
}

pub fn set_movement_y(instance: i32, value: i32) {
    unsafe {
        mouseevent_set_movement_y(instance, value);
    }
}
extern "C" {
    fn mouseevent_init_mouse_event(
        instance: i32,
        type_arg: CString,
        can_bubble_arg: i32,
        cancelable_arg: i32,
        view_arg: i32,
        detail_arg: i32,
        screen_x_arg: i32,
        screen_y_arg: i32,
        client_x_arg: i32,
        client_y_arg: i32,
        ctrl_key_arg: i32,
        alt_key_arg: i32,
        shift_key_arg: i32,
        meta_key_arg: i32,
        button_arg: i32,
        related_target_arg: i32,
    );
}

pub fn init_mouse_event(
    instance: i32,
    type_arg: &str,
    can_bubble_arg: i32,
    cancelable_arg: i32,
    view_arg: i32,
    detail_arg: i32,
    screen_x_arg: i32,
    screen_y_arg: i32,
    client_x_arg: i32,
    client_y_arg: i32,
    ctrl_key_arg: i32,
    alt_key_arg: i32,
    shift_key_arg: i32,
    meta_key_arg: i32,
    button_arg: i32,
    related_target_arg: i32,
) {
    unsafe {
        mouseevent_init_mouse_event(
            instance,
            cstr(type_arg),
            can_bubble_arg,
            cancelable_arg,
            view_arg,
            detail_arg,
            screen_x_arg,
            screen_y_arg,
            client_x_arg,
            client_y_arg,
            ctrl_key_arg,
            alt_key_arg,
            shift_key_arg,
            meta_key_arg,
            button_arg,
            related_target_arg,
        )
    }
}
extern "C" {
    fn mouseevent_get_modifier_state(instance: i32, key_arg: CString) -> i32;
}

pub fn get_modifier_state(instance: i32, key_arg: &str) -> i32 {
    unsafe { mouseevent_get_modifier_state(instance, cstr(key_arg)) }
}
