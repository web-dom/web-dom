#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn mouseevent_get_screen_x(instance: DOMReference) -> f32;
    fn mouseevent_set_screen_x(instance: DOMReference, value: f32);
}

pub fn get_screen_x(instance: DOMReference) -> f32 {
    unsafe { mouseevent_get_screen_x(instance) }
}

pub fn set_screen_x(instance: DOMReference, value: f32) {
    unsafe {
        mouseevent_set_screen_x(instance, value);
    }
}
extern "C" {
    fn mouseevent_get_screen_y(instance: DOMReference) -> f32;
    fn mouseevent_set_screen_y(instance: DOMReference, value: f32);
}

pub fn get_screen_y(instance: DOMReference) -> f32 {
    unsafe { mouseevent_get_screen_y(instance) }
}

pub fn set_screen_y(instance: DOMReference, value: f32) {
    unsafe {
        mouseevent_set_screen_y(instance, value);
    }
}
extern "C" {
    fn mouseevent_get_client_x(instance: DOMReference) -> f32;
    fn mouseevent_set_client_x(instance: DOMReference, value: f32);
}

pub fn get_client_x(instance: DOMReference) -> f32 {
    unsafe { mouseevent_get_client_x(instance) }
}

pub fn set_client_x(instance: DOMReference, value: f32) {
    unsafe {
        mouseevent_set_client_x(instance, value);
    }
}
extern "C" {
    fn mouseevent_get_client_y(instance: DOMReference) -> f32;
    fn mouseevent_set_client_y(instance: DOMReference, value: f32);
}

pub fn get_client_y(instance: DOMReference) -> f32 {
    unsafe { mouseevent_get_client_y(instance) }
}

pub fn set_client_y(instance: DOMReference, value: f32) {
    unsafe {
        mouseevent_set_client_y(instance, value);
    }
}
extern "C" {
    fn mouseevent_get_x(instance: DOMReference) -> f32;
    fn mouseevent_set_x(instance: DOMReference, value: f32);
}

pub fn get_x(instance: DOMReference) -> f32 {
    unsafe { mouseevent_get_x(instance) }
}

pub fn set_x(instance: DOMReference, value: f32) {
    unsafe {
        mouseevent_set_x(instance, value);
    }
}
extern "C" {
    fn mouseevent_get_y(instance: DOMReference) -> f32;
    fn mouseevent_set_y(instance: DOMReference, value: f32);
}

pub fn get_y(instance: DOMReference) -> f32 {
    unsafe { mouseevent_get_y(instance) }
}

pub fn set_y(instance: DOMReference, value: f32) {
    unsafe {
        mouseevent_set_y(instance, value);
    }
}
extern "C" {
    fn mouseevent_get_offset_x(instance: DOMReference) -> f32;
    fn mouseevent_set_offset_x(instance: DOMReference, value: f32);
}

pub fn get_offset_x(instance: DOMReference) -> f32 {
    unsafe { mouseevent_get_offset_x(instance) }
}

pub fn set_offset_x(instance: DOMReference, value: f32) {
    unsafe {
        mouseevent_set_offset_x(instance, value);
    }
}
extern "C" {
    fn mouseevent_get_offset_y(instance: DOMReference) -> f32;
    fn mouseevent_set_offset_y(instance: DOMReference, value: f32);
}

pub fn get_offset_y(instance: DOMReference) -> f32 {
    unsafe { mouseevent_get_offset_y(instance) }
}

pub fn set_offset_y(instance: DOMReference, value: f32) {
    unsafe {
        mouseevent_set_offset_y(instance, value);
    }
}
extern "C" {
    fn mouseevent_get_ctrl_key(instance: DOMReference) -> i32;
    fn mouseevent_set_ctrl_key(instance: DOMReference, value: i32);
}

pub fn get_ctrl_key(instance: DOMReference) -> bool {
    unsafe { 0 != mouseevent_get_ctrl_key(instance) }
}

pub fn set_ctrl_key(instance: DOMReference, value: bool) {
    unsafe {
        mouseevent_set_ctrl_key(instance, if value == true { 1 } else { 0 });
    }
}
extern "C" {
    fn mouseevent_get_shift_key(instance: DOMReference) -> i32;
    fn mouseevent_set_shift_key(instance: DOMReference, value: i32);
}

pub fn get_shift_key(instance: DOMReference) -> bool {
    unsafe { 0 != mouseevent_get_shift_key(instance) }
}

pub fn set_shift_key(instance: DOMReference, value: bool) {
    unsafe {
        mouseevent_set_shift_key(instance, if value == true { 1 } else { 0 });
    }
}
extern "C" {
    fn mouseevent_get_alt_key(instance: DOMReference) -> i32;
    fn mouseevent_set_alt_key(instance: DOMReference, value: i32);
}

pub fn get_alt_key(instance: DOMReference) -> bool {
    unsafe { 0 != mouseevent_get_alt_key(instance) }
}

pub fn set_alt_key(instance: DOMReference, value: bool) {
    unsafe {
        mouseevent_set_alt_key(instance, if value == true { 1 } else { 0 });
    }
}
extern "C" {
    fn mouseevent_get_meta_key(instance: DOMReference) -> i32;
    fn mouseevent_set_meta_key(instance: DOMReference, value: i32);
}

pub fn get_meta_key(instance: DOMReference) -> bool {
    unsafe { 0 != mouseevent_get_meta_key(instance) }
}

pub fn set_meta_key(instance: DOMReference, value: bool) {
    unsafe {
        mouseevent_set_meta_key(instance, if value == true { 1 } else { 0 });
    }
}
extern "C" {
    fn mouseevent_get_button(instance: DOMReference) -> f32;
    fn mouseevent_set_button(instance: DOMReference, value: f32);
}

pub fn get_button(instance: DOMReference) -> f32 {
    unsafe { mouseevent_get_button(instance) }
}

pub fn set_button(instance: DOMReference, value: f32) {
    unsafe {
        mouseevent_set_button(instance, value);
    }
}
extern "C" {
    fn mouseevent_get_buttons(instance: DOMReference) -> f32;
    fn mouseevent_set_buttons(instance: DOMReference, value: f32);
}

pub fn get_buttons(instance: DOMReference) -> f32 {
    unsafe { mouseevent_get_buttons(instance) }
}

pub fn set_buttons(instance: DOMReference, value: f32) {
    unsafe {
        mouseevent_set_buttons(instance, value);
    }
}
extern "C" {
    fn mouseevent_get_related_target(instance: DOMReference) -> DOMReference;
    fn mouseevent_set_related_target(instance: DOMReference, value: DOMReference);
}

pub fn get_related_target(instance: DOMReference) -> DOMReference {
    unsafe { mouseevent_get_related_target(instance) }
}

pub fn set_related_target(instance: DOMReference, value: DOMReference) {
    unsafe {
        mouseevent_set_related_target(instance, value);
    }
}
extern "C" {
    fn mouseevent_get_region(instance: DOMReference) -> CString;
    fn mouseevent_set_region(instance: DOMReference, value: CString);
}

pub fn get_region(instance: DOMReference) -> String {
    unsafe { to_string(mouseevent_get_region(instance)) }
}

pub fn set_region(instance: DOMReference, value: &str) {
    unsafe {
        mouseevent_set_region(instance, to_cstring(value));
    }
}
extern "C" {
    fn mouseevent_get_movement_x(instance: DOMReference) -> f32;
    fn mouseevent_set_movement_x(instance: DOMReference, value: f32);
}

pub fn get_movement_x(instance: DOMReference) -> f32 {
    unsafe { mouseevent_get_movement_x(instance) }
}

pub fn set_movement_x(instance: DOMReference, value: f32) {
    unsafe {
        mouseevent_set_movement_x(instance, value);
    }
}
extern "C" {
    fn mouseevent_get_movement_y(instance: DOMReference) -> f32;
    fn mouseevent_set_movement_y(instance: DOMReference, value: f32);
}

pub fn get_movement_y(instance: DOMReference) -> f32 {
    unsafe { mouseevent_get_movement_y(instance) }
}

pub fn set_movement_y(instance: DOMReference, value: f32) {
    unsafe {
        mouseevent_set_movement_y(instance, value);
    }
}
extern "C" {
    fn mouseevent_init_mouse_event(
        instance: DOMReference,
        init_mouse_event: CString,
        init_mouse_event: i32,
        init_mouse_event: i32,
        init_mouse_event: DOMReference,
        init_mouse_event: f32,
        init_mouse_event: f32,
        init_mouse_event: f32,
        init_mouse_event: f32,
        init_mouse_event: f32,
        init_mouse_event: i32,
        init_mouse_event: i32,
        init_mouse_event: i32,
        init_mouse_event: i32,
        init_mouse_event: f32,
        init_mouse_event: DOMReference,
    );
}

pub fn init_mouse_event(
    instance: DOMReference,
    type_arg: &str,
    can_bubble_arg: bool,
    cancelable_arg: bool,
    view_arg: DOMReference,
    detail_arg: f32,
    screen_x_arg: f32,
    screen_y_arg: f32,
    client_x_arg: f32,
    client_y_arg: f32,
    ctrl_key_arg: bool,
    alt_key_arg: bool,
    shift_key_arg: bool,
    meta_key_arg: bool,
    button_arg: f32,
    related_target_arg: DOMReference,
) {
    unsafe {
        mouseevent_init_mouse_event(
            instance,
            to_cstring(type_arg),
            if can_bubble_arg { 1 } else { 0 },
            if cancelable_arg { 1 } else { 0 },
            view_arg,
            detail_arg,
            screen_x_arg,
            screen_y_arg,
            client_x_arg,
            client_y_arg,
            if ctrl_key_arg { 1 } else { 0 },
            if alt_key_arg { 1 } else { 0 },
            if shift_key_arg { 1 } else { 0 },
            if meta_key_arg { 1 } else { 0 },
            button_arg,
            related_target_arg,
        )
    }
}
extern "C" {
    fn mouseevent_get_modifier_state(instance: DOMReference, get_modifier_state: CString) -> i32;
}

pub fn get_modifier_state(instance: DOMReference, key_arg: &str) -> bool {
    unsafe { 0 != mouseevent_get_modifier_state(instance, to_cstring(key_arg)) }
}
