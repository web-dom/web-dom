#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn window_get_window(instance: i32) -> i32;
    fn window_set_window(instance: i32, value: i32);
}

pub fn get_window(instance: i32) -> i32 {
    unsafe { window_get_window(instance) }
}

pub fn set_window(instance: i32, value: i32) {
    unsafe {
        window_set_window(instance, value);
    }
}
extern "C" {
    fn window_get_self(instance: i32) -> i32;
    fn window_set_self(instance: i32, value: i32);
}

pub fn get_self(instance: i32) -> i32 {
    unsafe { window_get_self(instance) }
}

pub fn set_self(instance: i32, value: i32) {
    unsafe {
        window_set_self(instance, value);
    }
}
extern "C" {
    fn window_get_document(instance: i32) -> i32;
    fn window_set_document(instance: i32, value: i32);
}

pub fn get_document(instance: i32) -> i32 {
    unsafe { window_get_document(instance) }
}

pub fn set_document(instance: i32, value: i32) {
    unsafe {
        window_set_document(instance, value);
    }
}
extern "C" {
    fn window_get_name(instance: i32) -> CString;
    fn window_set_name(instance: i32, value: CString);
}

pub fn get_name(instance: i32) -> String {
    unsafe { cstr_to_string(window_get_name(instance)) }
}

pub fn set_name(instance: i32, value: &str) {
    unsafe {
        window_set_name(instance, cstr(value));
    }
}
extern "C" {
    fn window_get_location(instance: i32) -> i32;
    fn window_set_location(instance: i32, value: i32);
}

pub fn get_location(instance: i32) -> i32 {
    unsafe { window_get_location(instance) }
}

pub fn set_location(instance: i32, value: i32) {
    unsafe {
        window_set_location(instance, value);
    }
}
extern "C" {
    fn window_get_history(instance: i32) -> i32;
    fn window_set_history(instance: i32, value: i32);
}

pub fn get_history(instance: i32) -> i32 {
    unsafe { window_get_history(instance) }
}

pub fn set_history(instance: i32, value: i32) {
    unsafe {
        window_set_history(instance, value);
    }
}
extern "C" {
    fn window_get_custom_elements(instance: i32) -> i32;
    fn window_set_custom_elements(instance: i32, value: i32);
}

pub fn get_custom_elements(instance: i32) -> i32 {
    unsafe { window_get_custom_elements(instance) }
}

pub fn set_custom_elements(instance: i32, value: i32) {
    unsafe {
        window_set_custom_elements(instance, value);
    }
}
extern "C" {
    fn window_get_locationbar(instance: i32) -> i32;
    fn window_set_locationbar(instance: i32, value: i32);
}

pub fn get_locationbar(instance: i32) -> i32 {
    unsafe { window_get_locationbar(instance) }
}

pub fn set_locationbar(instance: i32, value: i32) {
    unsafe {
        window_set_locationbar(instance, value);
    }
}
extern "C" {
    fn window_get_menubar(instance: i32) -> i32;
    fn window_set_menubar(instance: i32, value: i32);
}

pub fn get_menubar(instance: i32) -> i32 {
    unsafe { window_get_menubar(instance) }
}

pub fn set_menubar(instance: i32, value: i32) {
    unsafe {
        window_set_menubar(instance, value);
    }
}
extern "C" {
    fn window_get_personalbar(instance: i32) -> i32;
    fn window_set_personalbar(instance: i32, value: i32);
}

pub fn get_personalbar(instance: i32) -> i32 {
    unsafe { window_get_personalbar(instance) }
}

pub fn set_personalbar(instance: i32, value: i32) {
    unsafe {
        window_set_personalbar(instance, value);
    }
}
extern "C" {
    fn window_get_scrollbars(instance: i32) -> i32;
    fn window_set_scrollbars(instance: i32, value: i32);
}

pub fn get_scrollbars(instance: i32) -> i32 {
    unsafe { window_get_scrollbars(instance) }
}

pub fn set_scrollbars(instance: i32, value: i32) {
    unsafe {
        window_set_scrollbars(instance, value);
    }
}
extern "C" {
    fn window_get_statusbar(instance: i32) -> i32;
    fn window_set_statusbar(instance: i32, value: i32);
}

pub fn get_statusbar(instance: i32) -> i32 {
    unsafe { window_get_statusbar(instance) }
}

pub fn set_statusbar(instance: i32, value: i32) {
    unsafe {
        window_set_statusbar(instance, value);
    }
}
extern "C" {
    fn window_get_toolbar(instance: i32) -> i32;
    fn window_set_toolbar(instance: i32, value: i32);
}

pub fn get_toolbar(instance: i32) -> i32 {
    unsafe { window_get_toolbar(instance) }
}

pub fn set_toolbar(instance: i32, value: i32) {
    unsafe {
        window_set_toolbar(instance, value);
    }
}
extern "C" {
    fn window_get_status(instance: i32) -> CString;
    fn window_set_status(instance: i32, value: CString);
}

pub fn get_status(instance: i32) -> String {
    unsafe { cstr_to_string(window_get_status(instance)) }
}

pub fn set_status(instance: i32, value: &str) {
    unsafe {
        window_set_status(instance, cstr(value));
    }
}
extern "C" {
    fn window_close(instance: i32);
}

pub fn close(instance: i32) {
    unsafe { window_close(instance) }
}
extern "C" {
    fn window_get_closed(instance: i32) -> i32;
    fn window_set_closed(instance: i32, value: i32);
}

pub fn get_closed(instance: i32) -> i32 {
    unsafe { window_get_closed(instance) }
}

pub fn set_closed(instance: i32, value: i32) {
    unsafe {
        window_set_closed(instance, value);
    }
}
extern "C" {
    fn window_stop(instance: i32);
}

pub fn stop(instance: i32) {
    unsafe { window_stop(instance) }
}
extern "C" {
    fn window_focus(instance: i32);
}

pub fn focus(instance: i32) {
    unsafe { window_focus(instance) }
}
extern "C" {
    fn window_blur(instance: i32);
}

pub fn blur(instance: i32) {
    unsafe { window_blur(instance) }
}
extern "C" {
    fn window_get_event(instance: i32) -> i32;
    fn window_set_event(instance: i32, value: i32);
}

pub fn get_event(instance: i32) -> i32 {
    unsafe { window_get_event(instance) }
}

pub fn set_event(instance: i32, value: i32) {
    unsafe {
        window_set_event(instance, value);
    }
}
extern "C" {
    fn window_get_frames(instance: i32) -> i32;
    fn window_set_frames(instance: i32, value: i32);
}

pub fn get_frames(instance: i32) -> i32 {
    unsafe { window_get_frames(instance) }
}

pub fn set_frames(instance: i32, value: i32) {
    unsafe {
        window_set_frames(instance, value);
    }
}
extern "C" {
    fn window_get_length(instance: i32) -> i32;
    fn window_set_length(instance: i32, value: i32);
}

pub fn get_length(instance: i32) -> i32 {
    unsafe { window_get_length(instance) }
}

pub fn set_length(instance: i32, value: i32) {
    unsafe {
        window_set_length(instance, value);
    }
}
extern "C" {
    fn window_get_top(instance: i32) -> i32;
    fn window_set_top(instance: i32, value: i32);
}

pub fn get_top(instance: i32) -> i32 {
    unsafe { window_get_top(instance) }
}

pub fn set_top(instance: i32, value: i32) {
    unsafe {
        window_set_top(instance, value);
    }
}
extern "C" {
    fn window_get_opener(instance: i32) -> i32;
    fn window_set_opener(instance: i32, value: i32);
}

pub fn get_opener(instance: i32) -> i32 {
    unsafe { window_get_opener(instance) }
}

pub fn set_opener(instance: i32, value: i32) {
    unsafe {
        window_set_opener(instance, value);
    }
}
extern "C" {
    fn window_get_parent(instance: i32) -> i32;
    fn window_set_parent(instance: i32, value: i32);
}

pub fn get_parent(instance: i32) -> i32 {
    unsafe { window_get_parent(instance) }
}

pub fn set_parent(instance: i32, value: i32) {
    unsafe {
        window_set_parent(instance, value);
    }
}
extern "C" {
    fn window_get_frame_element(instance: i32) -> i32;
    fn window_set_frame_element(instance: i32, value: i32);
}

pub fn get_frame_element(instance: i32) -> i32 {
    unsafe { window_get_frame_element(instance) }
}

pub fn set_frame_element(instance: i32, value: i32) {
    unsafe {
        window_set_frame_element(instance, value);
    }
}
extern "C" {
    fn window_open(instance: i32, url: CString, target: CString, features: CString) -> i32;
}

pub fn open(instance: i32, url: &str, target: &str, features: &str) -> i32 {
    unsafe { window_open(instance, cstr(url), cstr(target), cstr(features)) }
}
extern "C" {
    fn window_get_navigator(instance: i32) -> i32;
    fn window_set_navigator(instance: i32, value: i32);
}

pub fn get_navigator(instance: i32) -> i32 {
    unsafe { window_get_navigator(instance) }
}

pub fn set_navigator(instance: i32, value: i32) {
    unsafe {
        window_set_navigator(instance, value);
    }
}
extern "C" {
    fn window_get_external(instance: i32) -> i32;
    fn window_set_external(instance: i32, value: i32);
}

pub fn get_external(instance: i32) -> i32 {
    unsafe { window_get_external(instance) }
}

pub fn set_external(instance: i32, value: i32) {
    unsafe {
        window_set_external(instance, value);
    }
}
extern "C" {
    fn window_get_application_cache(instance: i32) -> i32;
    fn window_set_application_cache(instance: i32, value: i32);
}

pub fn get_application_cache(instance: i32) -> i32 {
    unsafe { window_get_application_cache(instance) }
}

pub fn set_application_cache(instance: i32, value: i32) {
    unsafe {
        window_set_application_cache(instance, value);
    }
}
extern "C" {
    fn window_alert(instance: i32, message: CString);
}

pub fn alert(instance: i32, message: &str) {
    unsafe { window_alert(instance, cstr(message)) }
}
extern "C" {
    fn window_confirm(instance: i32, message: CString) -> i32;
}

pub fn confirm(instance: i32, message: &str) -> i32 {
    unsafe { window_confirm(instance, cstr(message)) }
}
extern "C" {
    fn window_prompt(instance: i32, message: CString, default_message: CString) -> CString;
}

pub fn prompt(instance: i32, message: &str, default_message: &str) -> String {
    unsafe {
        cstr_to_string(window_prompt(
            instance,
            cstr(message),
            cstr(default_message),
        ))
    }
}
extern "C" {
    fn window_print(instance: i32);
}

pub fn print(instance: i32) {
    unsafe { window_print(instance) }
}
extern "C" {
    fn window_post_message(instance: i32, message: i32, target_origin: CString, transfer: i32);
}

pub fn post_message(instance: i32, message: i32, target_origin: &str, transfer: i32) {
    unsafe { window_post_message(instance, message, cstr(target_origin), transfer) }
}
extern "C" {
    fn window_get_onappinstalled(instance: i32) -> i32;
    fn window_set_onappinstalled(instance: i32, value: i32);
}

pub fn get_onappinstalled(instance: i32) -> i32 {
    unsafe { window_get_onappinstalled(instance) }
}

pub fn set_onappinstalled(instance: i32, value: i32) {
    unsafe {
        window_set_onappinstalled(instance, value);
    }
}
extern "C" {
    fn window_capture_events(instance: i32);
}

pub fn capture_events(instance: i32) {
    unsafe { window_capture_events(instance) }
}
extern "C" {
    fn window_release_events(instance: i32);
}

pub fn release_events(instance: i32) {
    unsafe { window_release_events(instance) }
}
extern "C" {
    fn window_get_selection(instance: i32) -> i32;
}

pub fn get_selection(instance: i32) -> i32 {
    unsafe { window_get_selection(instance) }
}
extern "C" {
    fn window_get_computed_style(instance: i32, elt: i32, pseudo_elt: CString) -> i32;
}

pub fn get_computed_style(instance: i32, elt: i32, pseudo_elt: &str) -> i32 {
    unsafe { window_get_computed_style(instance, elt, cstr(pseudo_elt)) }
}
extern "C" {
    fn window_match_media(instance: i32, query: CString) -> i32;
}

pub fn match_media(instance: i32, query: &str) -> i32 {
    unsafe { window_match_media(instance, cstr(query)) }
}
extern "C" {
    fn window_get_screen(instance: i32) -> i32;
    fn window_set_screen(instance: i32, value: i32);
}

pub fn get_screen(instance: i32) -> i32 {
    unsafe { window_get_screen(instance) }
}

pub fn set_screen(instance: i32, value: i32) {
    unsafe {
        window_set_screen(instance, value);
    }
}
extern "C" {
    fn window_move_to(instance: i32, x: i32, y: i32);
}

pub fn move_to(instance: i32, x: i32, y: i32) {
    unsafe { window_move_to(instance, x, y) }
}
extern "C" {
    fn window_move_by(instance: i32, x: i32, y: i32);
}

pub fn move_by(instance: i32, x: i32, y: i32) {
    unsafe { window_move_by(instance, x, y) }
}
extern "C" {
    fn window_resize_to(instance: i32, x: i32, y: i32);
}

pub fn resize_to(instance: i32, x: i32, y: i32) {
    unsafe { window_resize_to(instance, x, y) }
}
extern "C" {
    fn window_resize_by(instance: i32, x: i32, y: i32);
}

pub fn resize_by(instance: i32, x: i32, y: i32) {
    unsafe { window_resize_by(instance, x, y) }
}
extern "C" {
    fn window_get_inner_width(instance: i32) -> i32;
    fn window_set_inner_width(instance: i32, value: i32);
}

pub fn get_inner_width(instance: i32) -> i32 {
    unsafe { window_get_inner_width(instance) }
}

pub fn set_inner_width(instance: i32, value: i32) {
    unsafe {
        window_set_inner_width(instance, value);
    }
}
extern "C" {
    fn window_get_inner_height(instance: i32) -> i32;
    fn window_set_inner_height(instance: i32, value: i32);
}

pub fn get_inner_height(instance: i32) -> i32 {
    unsafe { window_get_inner_height(instance) }
}

pub fn set_inner_height(instance: i32, value: i32) {
    unsafe {
        window_set_inner_height(instance, value);
    }
}
extern "C" {
    fn window_scroll(instance: i32, x: i32, y: i32);
}

pub fn scroll(instance: i32, x: i32, y: i32) {
    unsafe { window_scroll(instance, x, y) }
}
extern "C" {
    fn window_scroll_to(instance: i32, x: i32, y: i32);
}

pub fn scroll_to(instance: i32, x: i32, y: i32) {
    unsafe { window_scroll_to(instance, x, y) }
}
extern "C" {
    fn window_scroll_by(instance: i32, x: i32, y: i32);
}

pub fn scroll_by(instance: i32, x: i32, y: i32) {
    unsafe { window_scroll_by(instance, x, y) }
}
extern "C" {
    fn window_get_scroll_x(instance: i32) -> i32;
    fn window_set_scroll_x(instance: i32, value: i32);
}

pub fn get_scroll_x(instance: i32) -> i32 {
    unsafe { window_get_scroll_x(instance) }
}

pub fn set_scroll_x(instance: i32, value: i32) {
    unsafe {
        window_set_scroll_x(instance, value);
    }
}
extern "C" {
    fn window_get_page_x_offset(instance: i32) -> i32;
    fn window_set_page_x_offset(instance: i32, value: i32);
}

pub fn get_page_x_offset(instance: i32) -> i32 {
    unsafe { window_get_page_x_offset(instance) }
}

pub fn set_page_x_offset(instance: i32, value: i32) {
    unsafe {
        window_set_page_x_offset(instance, value);
    }
}
extern "C" {
    fn window_get_scroll_y(instance: i32) -> i32;
    fn window_set_scroll_y(instance: i32, value: i32);
}

pub fn get_scroll_y(instance: i32) -> i32 {
    unsafe { window_get_scroll_y(instance) }
}

pub fn set_scroll_y(instance: i32, value: i32) {
    unsafe {
        window_set_scroll_y(instance, value);
    }
}
extern "C" {
    fn window_get_page_y_offset(instance: i32) -> i32;
    fn window_set_page_y_offset(instance: i32, value: i32);
}

pub fn get_page_y_offset(instance: i32) -> i32 {
    unsafe { window_get_page_y_offset(instance) }
}

pub fn set_page_y_offset(instance: i32, value: i32) {
    unsafe {
        window_set_page_y_offset(instance, value);
    }
}
extern "C" {
    fn window_get_screen_x(instance: i32) -> i32;
    fn window_set_screen_x(instance: i32, value: i32);
}

pub fn get_screen_x(instance: i32) -> i32 {
    unsafe { window_get_screen_x(instance) }
}

pub fn set_screen_x(instance: i32, value: i32) {
    unsafe {
        window_set_screen_x(instance, value);
    }
}
extern "C" {
    fn window_get_screen_y(instance: i32) -> i32;
    fn window_set_screen_y(instance: i32, value: i32);
}

pub fn get_screen_y(instance: i32) -> i32 {
    unsafe { window_get_screen_y(instance) }
}

pub fn set_screen_y(instance: i32, value: i32) {
    unsafe {
        window_set_screen_y(instance, value);
    }
}
extern "C" {
    fn window_get_outer_width(instance: i32) -> i32;
    fn window_set_outer_width(instance: i32, value: i32);
}

pub fn get_outer_width(instance: i32) -> i32 {
    unsafe { window_get_outer_width(instance) }
}

pub fn set_outer_width(instance: i32, value: i32) {
    unsafe {
        window_set_outer_width(instance, value);
    }
}
extern "C" {
    fn window_get_outer_height(instance: i32) -> i32;
    fn window_set_outer_height(instance: i32, value: i32);
}

pub fn get_outer_height(instance: i32) -> i32 {
    unsafe { window_get_outer_height(instance) }
}

pub fn set_outer_height(instance: i32, value: i32) {
    unsafe {
        window_set_outer_height(instance, value);
    }
}
extern "C" {
    fn window_get_device_pixel_ratio(instance: i32) -> i32;
    fn window_set_device_pixel_ratio(instance: i32, value: i32);
}

pub fn get_device_pixel_ratio(instance: i32) -> i32 {
    unsafe { window_get_device_pixel_ratio(instance) }
}

pub fn set_device_pixel_ratio(instance: i32, value: i32) {
    unsafe {
        window_set_device_pixel_ratio(instance, value);
    }
}
extern "C" {
    fn window_request_animation_frame(instance: i32, callback: i32) -> i32;
}

pub fn request_animation_frame(instance: i32, callback: i32) -> i32 {
    unsafe { window_request_animation_frame(instance, callback) }
}
extern "C" {
    fn window_cancel_animation_frame(instance: i32, handle: i32);
}

pub fn cancel_animation_frame(instance: i32, handle: i32) {
    unsafe { window_cancel_animation_frame(instance, handle) }
}
extern "C" {
    fn window_get_performance(instance: i32) -> i32;
    fn window_set_performance(instance: i32, value: i32);
}

pub fn get_performance(instance: i32) -> i32 {
    unsafe { window_get_performance(instance) }
}

pub fn set_performance(instance: i32, value: i32) {
    unsafe {
        window_set_performance(instance, value);
    }
}
extern "C" {
    fn window_get_orientation(instance: i32) -> i32;
    fn window_set_orientation(instance: i32, value: i32);
}

pub fn get_orientation(instance: i32) -> i32 {
    unsafe { window_get_orientation(instance) }
}

pub fn set_orientation(instance: i32, value: i32) {
    unsafe {
        window_set_orientation(instance, value);
    }
}
extern "C" {
    fn window_get_onorientationchange(instance: i32) -> i32;
    fn window_set_onorientationchange(instance: i32, value: i32);
}

pub fn get_onorientationchange(instance: i32) -> i32 {
    unsafe { window_get_onorientationchange(instance) }
}

pub fn set_onorientationchange(instance: i32, value: i32) {
    unsafe {
        window_set_onorientationchange(instance, value);
    }
}
extern "C" {
    fn window_get_onvrdisplayconnect(instance: i32) -> i32;
    fn window_set_onvrdisplayconnect(instance: i32, value: i32);
}

pub fn get_onvrdisplayconnect(instance: i32) -> i32 {
    unsafe { window_get_onvrdisplayconnect(instance) }
}

pub fn set_onvrdisplayconnect(instance: i32, value: i32) {
    unsafe {
        window_set_onvrdisplayconnect(instance, value);
    }
}
extern "C" {
    fn window_get_onvrdisplaydisconnect(instance: i32) -> i32;
    fn window_set_onvrdisplaydisconnect(instance: i32, value: i32);
}

pub fn get_onvrdisplaydisconnect(instance: i32) -> i32 {
    unsafe { window_get_onvrdisplaydisconnect(instance) }
}

pub fn set_onvrdisplaydisconnect(instance: i32, value: i32) {
    unsafe {
        window_set_onvrdisplaydisconnect(instance, value);
    }
}
extern "C" {
    fn window_get_onvrdisplayactivate(instance: i32) -> i32;
    fn window_set_onvrdisplayactivate(instance: i32, value: i32);
}

pub fn get_onvrdisplayactivate(instance: i32) -> i32 {
    unsafe { window_get_onvrdisplayactivate(instance) }
}

pub fn set_onvrdisplayactivate(instance: i32, value: i32) {
    unsafe {
        window_set_onvrdisplayactivate(instance, value);
    }
}
extern "C" {
    fn window_get_onvrdisplaydeactivate(instance: i32) -> i32;
    fn window_set_onvrdisplaydeactivate(instance: i32, value: i32);
}

pub fn get_onvrdisplaydeactivate(instance: i32) -> i32 {
    unsafe { window_get_onvrdisplaydeactivate(instance) }
}

pub fn set_onvrdisplaydeactivate(instance: i32, value: i32) {
    unsafe {
        window_set_onvrdisplaydeactivate(instance, value);
    }
}
extern "C" {
    fn window_get_onvrdisplaypresentchange(instance: i32) -> i32;
    fn window_set_onvrdisplaypresentchange(instance: i32, value: i32);
}

pub fn get_onvrdisplaypresentchange(instance: i32) -> i32 {
    unsafe { window_get_onvrdisplaypresentchange(instance) }
}

pub fn set_onvrdisplaypresentchange(instance: i32, value: i32) {
    unsafe {
        window_set_onvrdisplaypresentchange(instance, value);
    }
}
extern "C" {
    fn window_get_paint_worklet(instance: i32) -> i32;
    fn window_set_paint_worklet(instance: i32, value: i32);
}

pub fn get_paint_worklet(instance: i32) -> i32 {
    unsafe { window_get_paint_worklet(instance) }
}

pub fn set_paint_worklet(instance: i32, value: i32) {
    unsafe {
        window_set_paint_worklet(instance, value);
    }
}
extern "C" {
    fn window_request_idle_callback(instance: i32, callback: i32, options: i32) -> i32;
}

pub fn request_idle_callback(instance: i32, callback: i32, options: i32) -> i32 {
    unsafe { window_request_idle_callback(instance, callback, options) }
}
extern "C" {
    fn window_cancel_idle_callback(instance: i32, handle: i32);
}

pub fn cancel_idle_callback(instance: i32, handle: i32) {
    unsafe { window_cancel_idle_callback(instance, handle) }
}
extern "C" {
    fn window_get_origin(instance: i32) -> i32;
    fn window_set_origin(instance: i32, value: i32);
}

pub fn get_origin(instance: i32) -> i32 {
    unsafe { window_get_origin(instance) }
}

pub fn set_origin(instance: i32, value: i32) {
    unsafe {
        window_set_origin(instance, value);
    }
}
extern "C" {
    fn window_btoa(instance: i32, btoa: CString) -> CString;
}

pub fn btoa(instance: i32, btoa: &str) -> String {
    unsafe { cstr_to_string(window_btoa(instance, cstr(btoa))) }
}
extern "C" {
    fn window_atob(instance: i32, atob: CString) -> CString;
}

pub fn atob(instance: i32, atob: &str) -> String {
    unsafe { cstr_to_string(window_atob(instance, cstr(atob))) }
}
extern "C" {
    fn window_set_timeout(instance: i32, handler: i32, timeout: i32) -> i32;
}

pub fn set_timeout(instance: i32, handler: i32, timeout: i32) -> i32 {
    unsafe { window_set_timeout(instance, handler, timeout) }
}
extern "C" {
    fn window_clear_timeout(instance: i32, handle: i32);
}

pub fn clear_timeout(instance: i32, handle: i32) {
    unsafe { window_clear_timeout(instance, handle) }
}
extern "C" {
    fn window_set_interval(instance: i32, handler: i32, timeout: i32) -> i32;
}

pub fn set_interval(instance: i32, handler: i32, timeout: i32) -> i32 {
    unsafe { window_set_interval(instance, handler, timeout) }
}
extern "C" {
    fn window_clear_interval(instance: i32, handle: i32);
}

pub fn clear_interval(instance: i32, handle: i32) {
    unsafe { window_clear_interval(instance, handle) }
}
extern "C" {
    fn window_create_image_bitmap(
        instance: i32,
        a_image: i32,
        a_sx: i32,
        a_sy: i32,
        a_sw: i32,
        a_sh: i32,
    ) -> i32;
}

pub fn create_image_bitmap(
    instance: i32,
    a_image: i32,
    a_sx: i32,
    a_sy: i32,
    a_sw: i32,
    a_sh: i32,
) -> i32 {
    unsafe { window_create_image_bitmap(instance, a_image, a_sx, a_sy, a_sw, a_sh) }
}
