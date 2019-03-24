#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn window_get_window(instance: DOMReference) -> DOMReference;
    fn window_set_window(instance: DOMReference, value: DOMReference);
}

pub fn get_window(instance: DOMReference) -> DOMReference {
    unsafe { window_get_window(instance) }
}

pub fn set_window(instance: DOMReference, value: DOMReference) {
    unsafe {
        window_set_window(instance, value);
    }
}
extern "C" {
    fn window_get_self(instance: DOMReference) -> DOMReference;
    fn window_set_self(instance: DOMReference, value: DOMReference);
}

pub fn get_self(instance: DOMReference) -> DOMReference {
    unsafe { window_get_self(instance) }
}

pub fn set_self(instance: DOMReference, value: DOMReference) {
    unsafe {
        window_set_self(instance, value);
    }
}
extern "C" {
    fn window_get_document(instance: DOMReference) -> DOMReference;
    fn window_set_document(instance: DOMReference, value: DOMReference);
}

pub fn get_document(instance: DOMReference) -> DOMReference {
    unsafe { window_get_document(instance) }
}

pub fn set_document(instance: DOMReference, value: DOMReference) {
    unsafe {
        window_set_document(instance, value);
    }
}
extern "C" {
    fn window_get_name(instance: DOMReference) -> CString;
    fn window_set_name(instance: DOMReference, value: CString);
}

pub fn get_name(instance: DOMReference) -> String {
    unsafe { to_string(window_get_name(instance)) }
}

pub fn set_name(instance: DOMReference, value: &str) {
    unsafe {
        window_set_name(instance, to_cstring(value));
    }
}
extern "C" {
    fn window_get_location(instance: DOMReference) -> DOMReference;
    fn window_set_location(instance: DOMReference, value: DOMReference);
}

pub fn get_location(instance: DOMReference) -> DOMReference {
    unsafe { window_get_location(instance) }
}

pub fn set_location(instance: DOMReference, value: DOMReference) {
    unsafe {
        window_set_location(instance, value);
    }
}
extern "C" {
    fn window_get_history(instance: DOMReference) -> DOMReference;
    fn window_set_history(instance: DOMReference, value: DOMReference);
}

pub fn get_history(instance: DOMReference) -> DOMReference {
    unsafe { window_get_history(instance) }
}

pub fn set_history(instance: DOMReference, value: DOMReference) {
    unsafe {
        window_set_history(instance, value);
    }
}
extern "C" {
    fn window_get_custom_elements(instance: DOMReference) -> DOMReference;
    fn window_set_custom_elements(instance: DOMReference, value: DOMReference);
}

pub fn get_custom_elements(instance: DOMReference) -> DOMReference {
    unsafe { window_get_custom_elements(instance) }
}

pub fn set_custom_elements(instance: DOMReference, value: DOMReference) {
    unsafe {
        window_set_custom_elements(instance, value);
    }
}
extern "C" {
    fn window_get_locationbar(instance: DOMReference) -> DOMReference;
    fn window_set_locationbar(instance: DOMReference, value: DOMReference);
}

pub fn get_locationbar(instance: DOMReference) -> DOMReference {
    unsafe { window_get_locationbar(instance) }
}

pub fn set_locationbar(instance: DOMReference, value: DOMReference) {
    unsafe {
        window_set_locationbar(instance, value);
    }
}
extern "C" {
    fn window_get_menubar(instance: DOMReference) -> DOMReference;
    fn window_set_menubar(instance: DOMReference, value: DOMReference);
}

pub fn get_menubar(instance: DOMReference) -> DOMReference {
    unsafe { window_get_menubar(instance) }
}

pub fn set_menubar(instance: DOMReference, value: DOMReference) {
    unsafe {
        window_set_menubar(instance, value);
    }
}
extern "C" {
    fn window_get_personalbar(instance: DOMReference) -> DOMReference;
    fn window_set_personalbar(instance: DOMReference, value: DOMReference);
}

pub fn get_personalbar(instance: DOMReference) -> DOMReference {
    unsafe { window_get_personalbar(instance) }
}

pub fn set_personalbar(instance: DOMReference, value: DOMReference) {
    unsafe {
        window_set_personalbar(instance, value);
    }
}
extern "C" {
    fn window_get_scrollbars(instance: DOMReference) -> DOMReference;
    fn window_set_scrollbars(instance: DOMReference, value: DOMReference);
}

pub fn get_scrollbars(instance: DOMReference) -> DOMReference {
    unsafe { window_get_scrollbars(instance) }
}

pub fn set_scrollbars(instance: DOMReference, value: DOMReference) {
    unsafe {
        window_set_scrollbars(instance, value);
    }
}
extern "C" {
    fn window_get_statusbar(instance: DOMReference) -> DOMReference;
    fn window_set_statusbar(instance: DOMReference, value: DOMReference);
}

pub fn get_statusbar(instance: DOMReference) -> DOMReference {
    unsafe { window_get_statusbar(instance) }
}

pub fn set_statusbar(instance: DOMReference, value: DOMReference) {
    unsafe {
        window_set_statusbar(instance, value);
    }
}
extern "C" {
    fn window_get_toolbar(instance: DOMReference) -> DOMReference;
    fn window_set_toolbar(instance: DOMReference, value: DOMReference);
}

pub fn get_toolbar(instance: DOMReference) -> DOMReference {
    unsafe { window_get_toolbar(instance) }
}

pub fn set_toolbar(instance: DOMReference, value: DOMReference) {
    unsafe {
        window_set_toolbar(instance, value);
    }
}
extern "C" {
    fn window_get_status(instance: DOMReference) -> CString;
    fn window_set_status(instance: DOMReference, value: CString);
}

pub fn get_status(instance: DOMReference) -> String {
    unsafe { to_string(window_get_status(instance)) }
}

pub fn set_status(instance: DOMReference, value: &str) {
    unsafe {
        window_set_status(instance, to_cstring(value));
    }
}
extern "C" {
    fn window_close(instance: DOMReference);
}

pub fn close(instance: DOMReference) {
    unsafe { window_close(instance) }
}
extern "C" {
    fn window_get_closed(instance: DOMReference) -> i32;
    fn window_set_closed(instance: DOMReference, value: i32);
}

pub fn get_closed(instance: DOMReference) -> bool {
    unsafe { 0 != window_get_closed(instance) }
}

pub fn set_closed(instance: DOMReference, value: bool) {
    unsafe {
        window_set_closed(instance, if value == true { 1 } else { 0 });
    }
}
extern "C" {
    fn window_stop(instance: DOMReference);
}

pub fn stop(instance: DOMReference) {
    unsafe { window_stop(instance) }
}
extern "C" {
    fn window_focus(instance: DOMReference);
}

pub fn focus(instance: DOMReference) {
    unsafe { window_focus(instance) }
}
extern "C" {
    fn window_blur(instance: DOMReference);
}

pub fn blur(instance: DOMReference) {
    unsafe { window_blur(instance) }
}
extern "C" {
    fn window_get_event(instance: DOMReference) -> DOMReference;
    fn window_set_event(instance: DOMReference, value: DOMReference);
}

pub fn get_event(instance: DOMReference) -> DOMReference {
    unsafe { window_get_event(instance) }
}

pub fn set_event(instance: DOMReference, value: DOMReference) {
    unsafe {
        window_set_event(instance, value);
    }
}
extern "C" {
    fn window_get_frames(instance: DOMReference) -> DOMReference;
    fn window_set_frames(instance: DOMReference, value: DOMReference);
}

pub fn get_frames(instance: DOMReference) -> DOMReference {
    unsafe { window_get_frames(instance) }
}

pub fn set_frames(instance: DOMReference, value: DOMReference) {
    unsafe {
        window_set_frames(instance, value);
    }
}
extern "C" {
    fn window_get_length(instance: DOMReference) -> f32;
    fn window_set_length(instance: DOMReference, value: f32);
}

pub fn get_length(instance: DOMReference) -> f32 {
    unsafe { window_get_length(instance) }
}

pub fn set_length(instance: DOMReference, value: f32) {
    unsafe {
        window_set_length(instance, value);
    }
}
extern "C" {
    fn window_get_top(instance: DOMReference) -> DOMReference;
    fn window_set_top(instance: DOMReference, value: DOMReference);
}

pub fn get_top(instance: DOMReference) -> DOMReference {
    unsafe { window_get_top(instance) }
}

pub fn set_top(instance: DOMReference, value: DOMReference) {
    unsafe {
        window_set_top(instance, value);
    }
}
extern "C" {
    fn window_get_opener(instance: DOMReference) -> DOMReference;
    fn window_set_opener(instance: DOMReference, value: DOMReference);
}

pub fn get_opener(instance: DOMReference) -> DOMReference {
    unsafe { window_get_opener(instance) }
}

pub fn set_opener(instance: DOMReference, value: DOMReference) {
    unsafe {
        window_set_opener(instance, value);
    }
}
extern "C" {
    fn window_get_parent(instance: DOMReference) -> DOMReference;
    fn window_set_parent(instance: DOMReference, value: DOMReference);
}

pub fn get_parent(instance: DOMReference) -> DOMReference {
    unsafe { window_get_parent(instance) }
}

pub fn set_parent(instance: DOMReference, value: DOMReference) {
    unsafe {
        window_set_parent(instance, value);
    }
}
extern "C" {
    fn window_get_frame_element(instance: DOMReference) -> DOMReference;
    fn window_set_frame_element(instance: DOMReference, value: DOMReference);
}

pub fn get_frame_element(instance: DOMReference) -> DOMReference {
    unsafe { window_get_frame_element(instance) }
}

pub fn set_frame_element(instance: DOMReference, value: DOMReference) {
    unsafe {
        window_set_frame_element(instance, value);
    }
}
extern "C" {
    fn window_open(
        instance: DOMReference,
        open: CString,
        open: CString,
        open: CString,
    ) -> DOMReference;
}

pub fn open(instance: DOMReference, url: &str, target: &str, features: &str) -> DOMReference {
    unsafe {
        window_open(
            instance,
            to_cstring(url),
            to_cstring(target),
            to_cstring(features),
        )
    }
}
extern "C" {
    fn window_get_navigator(instance: DOMReference) -> DOMReference;
    fn window_set_navigator(instance: DOMReference, value: DOMReference);
}

pub fn get_navigator(instance: DOMReference) -> DOMReference {
    unsafe { window_get_navigator(instance) }
}

pub fn set_navigator(instance: DOMReference, value: DOMReference) {
    unsafe {
        window_set_navigator(instance, value);
    }
}
extern "C" {
    fn window_get_external(instance: DOMReference) -> DOMReference;
    fn window_set_external(instance: DOMReference, value: DOMReference);
}

pub fn get_external(instance: DOMReference) -> DOMReference {
    unsafe { window_get_external(instance) }
}

pub fn set_external(instance: DOMReference, value: DOMReference) {
    unsafe {
        window_set_external(instance, value);
    }
}
extern "C" {
    fn window_get_application_cache(instance: DOMReference) -> DOMReference;
    fn window_set_application_cache(instance: DOMReference, value: DOMReference);
}

pub fn get_application_cache(instance: DOMReference) -> DOMReference {
    unsafe { window_get_application_cache(instance) }
}

pub fn set_application_cache(instance: DOMReference, value: DOMReference) {
    unsafe {
        window_set_application_cache(instance, value);
    }
}
extern "C" {
    fn window_alert(instance: DOMReference, alert: CString);
}

pub fn alert(instance: DOMReference, message: &str) {
    unsafe { window_alert(instance, to_cstring(message)) }
}
extern "C" {
    fn window_confirm(instance: DOMReference, confirm: CString) -> i32;
}

pub fn confirm(instance: DOMReference, message: &str) -> bool {
    unsafe { 0 != window_confirm(instance, to_cstring(message)) }
}
extern "C" {
    fn window_prompt(instance: DOMReference, prompt: CString, prompt: CString) -> DOMReference;
}

pub fn prompt(instance: DOMReference, message: &str, default_message: &str) -> DOMReference {
    unsafe { window_prompt(instance, to_cstring(message), to_cstring(default_message)) }
}
extern "C" {
    fn window_print(instance: DOMReference);
}

pub fn print(instance: DOMReference) {
    unsafe { window_print(instance) }
}
extern "C" {
    fn window_post_message(
        instance: DOMReference,
        post_message: DOMReference,
        post_message: CString,
        post_message: DOMReference,
    );
}

pub fn post_message(
    instance: DOMReference,
    message: DOMReference,
    target_origin: &str,
    transfer: DOMReference,
) {
    unsafe { window_post_message(instance, message, to_cstring(target_origin), transfer) }
}
extern "C" {
    fn window_get_onappinstalled(instance: DOMReference) -> DOMReference;
    fn window_set_onappinstalled(instance: DOMReference, value: DOMReference);
}

pub fn get_onappinstalled(instance: DOMReference) -> DOMReference {
    unsafe { window_get_onappinstalled(instance) }
}

pub fn set_onappinstalled(instance: DOMReference, value: DOMReference) {
    unsafe {
        window_set_onappinstalled(instance, value);
    }
}
extern "C" {
    fn window_get_session_storage(instance: DOMReference) -> DOMReference;
    fn window_set_session_storage(instance: DOMReference, value: DOMReference);
}

pub fn get_session_storage(instance: DOMReference) -> DOMReference {
    unsafe { window_get_session_storage(instance) }
}

pub fn set_session_storage(instance: DOMReference, value: DOMReference) {
    unsafe {
        window_set_session_storage(instance, value);
    }
}
extern "C" {
    fn window_get_local_storage(instance: DOMReference) -> DOMReference;
    fn window_set_local_storage(instance: DOMReference, value: DOMReference);
}

pub fn get_local_storage(instance: DOMReference) -> DOMReference {
    unsafe { window_get_local_storage(instance) }
}

pub fn set_local_storage(instance: DOMReference, value: DOMReference) {
    unsafe {
        window_set_local_storage(instance, value);
    }
}
extern "C" {
    fn window_capture_events(instance: DOMReference);
}

pub fn capture_events(instance: DOMReference) {
    unsafe { window_capture_events(instance) }
}
extern "C" {
    fn window_release_events(instance: DOMReference);
}

pub fn release_events(instance: DOMReference) {
    unsafe { window_release_events(instance) }
}
extern "C" {
    fn window_get_selection(instance: DOMReference) -> DOMReference;
}

pub fn get_selection(instance: DOMReference) -> DOMReference {
    unsafe { window_get_selection(instance) }
}
extern "C" {
    fn window_get_computed_style(
        instance: DOMReference,
        get_computed_style: DOMReference,
        get_computed_style: CString,
    ) -> DOMReference;
}

pub fn get_computed_style(
    instance: DOMReference,
    elt: DOMReference,
    pseudo_elt: &str,
) -> DOMReference {
    unsafe { window_get_computed_style(instance, elt, to_cstring(pseudo_elt)) }
}
extern "C" {
    fn window_match_media(instance: DOMReference, match_media: CString) -> DOMReference;
}

pub fn match_media(instance: DOMReference, query: &str) -> DOMReference {
    unsafe { window_match_media(instance, to_cstring(query)) }
}
extern "C" {
    fn window_get_screen(instance: DOMReference) -> DOMReference;
    fn window_set_screen(instance: DOMReference, value: DOMReference);
}

pub fn get_screen(instance: DOMReference) -> DOMReference {
    unsafe { window_get_screen(instance) }
}

pub fn set_screen(instance: DOMReference, value: DOMReference) {
    unsafe {
        window_set_screen(instance, value);
    }
}
extern "C" {
    fn window_move_to(instance: DOMReference, move_to: f32, move_to: f32);
}

pub fn move_to(instance: DOMReference, x: f32, y: f32) {
    unsafe { window_move_to(instance, x, y) }
}
extern "C" {
    fn window_move_by(instance: DOMReference, move_by: f32, move_by: f32);
}

pub fn move_by(instance: DOMReference, x: f32, y: f32) {
    unsafe { window_move_by(instance, x, y) }
}
extern "C" {
    fn window_resize_to(instance: DOMReference, resize_to: f32, resize_to: f32);
}

pub fn resize_to(instance: DOMReference, x: f32, y: f32) {
    unsafe { window_resize_to(instance, x, y) }
}
extern "C" {
    fn window_resize_by(instance: DOMReference, resize_by: f32, resize_by: f32);
}

pub fn resize_by(instance: DOMReference, x: f32, y: f32) {
    unsafe { window_resize_by(instance, x, y) }
}
extern "C" {
    fn window_get_inner_width(instance: DOMReference) -> DOMReference;
    fn window_set_inner_width(instance: DOMReference, value: DOMReference);
}

pub fn get_inner_width(instance: DOMReference) -> DOMReference {
    unsafe { window_get_inner_width(instance) }
}

pub fn set_inner_width(instance: DOMReference, value: DOMReference) {
    unsafe {
        window_set_inner_width(instance, value);
    }
}
extern "C" {
    fn window_get_inner_height(instance: DOMReference) -> DOMReference;
    fn window_set_inner_height(instance: DOMReference, value: DOMReference);
}

pub fn get_inner_height(instance: DOMReference) -> DOMReference {
    unsafe { window_get_inner_height(instance) }
}

pub fn set_inner_height(instance: DOMReference, value: DOMReference) {
    unsafe {
        window_set_inner_height(instance, value);
    }
}
extern "C" {
    fn window_scroll(instance: DOMReference, scroll: f32, scroll: f32);
}

pub fn scroll(instance: DOMReference, x: f32, y: f32) {
    unsafe { window_scroll(instance, x, y) }
}
extern "C" {
    fn window_scroll_to(instance: DOMReference, scroll_to: f32, scroll_to: f32);
}

pub fn scroll_to(instance: DOMReference, x: f32, y: f32) {
    unsafe { window_scroll_to(instance, x, y) }
}
extern "C" {
    fn window_scroll_by(instance: DOMReference, scroll_by: f32, scroll_by: f32);
}

pub fn scroll_by(instance: DOMReference, x: f32, y: f32) {
    unsafe { window_scroll_by(instance, x, y) }
}
extern "C" {
    fn window_get_scroll_x(instance: DOMReference) -> f32;
    fn window_set_scroll_x(instance: DOMReference, value: f32);
}

pub fn get_scroll_x(instance: DOMReference) -> f32 {
    unsafe { window_get_scroll_x(instance) }
}

pub fn set_scroll_x(instance: DOMReference, value: f32) {
    unsafe {
        window_set_scroll_x(instance, value);
    }
}
extern "C" {
    fn window_get_page_x_offset(instance: DOMReference) -> f32;
    fn window_set_page_x_offset(instance: DOMReference, value: f32);
}

pub fn get_page_x_offset(instance: DOMReference) -> f32 {
    unsafe { window_get_page_x_offset(instance) }
}

pub fn set_page_x_offset(instance: DOMReference, value: f32) {
    unsafe {
        window_set_page_x_offset(instance, value);
    }
}
extern "C" {
    fn window_get_scroll_y(instance: DOMReference) -> f32;
    fn window_set_scroll_y(instance: DOMReference, value: f32);
}

pub fn get_scroll_y(instance: DOMReference) -> f32 {
    unsafe { window_get_scroll_y(instance) }
}

pub fn set_scroll_y(instance: DOMReference, value: f32) {
    unsafe {
        window_set_scroll_y(instance, value);
    }
}
extern "C" {
    fn window_get_page_y_offset(instance: DOMReference) -> f32;
    fn window_set_page_y_offset(instance: DOMReference, value: f32);
}

pub fn get_page_y_offset(instance: DOMReference) -> f32 {
    unsafe { window_get_page_y_offset(instance) }
}

pub fn set_page_y_offset(instance: DOMReference, value: f32) {
    unsafe {
        window_set_page_y_offset(instance, value);
    }
}
extern "C" {
    fn window_get_screen_x(instance: DOMReference) -> DOMReference;
    fn window_set_screen_x(instance: DOMReference, value: DOMReference);
}

pub fn get_screen_x(instance: DOMReference) -> DOMReference {
    unsafe { window_get_screen_x(instance) }
}

pub fn set_screen_x(instance: DOMReference, value: DOMReference) {
    unsafe {
        window_set_screen_x(instance, value);
    }
}
extern "C" {
    fn window_get_screen_y(instance: DOMReference) -> DOMReference;
    fn window_set_screen_y(instance: DOMReference, value: DOMReference);
}

pub fn get_screen_y(instance: DOMReference) -> DOMReference {
    unsafe { window_get_screen_y(instance) }
}

pub fn set_screen_y(instance: DOMReference, value: DOMReference) {
    unsafe {
        window_set_screen_y(instance, value);
    }
}
extern "C" {
    fn window_get_outer_width(instance: DOMReference) -> DOMReference;
    fn window_set_outer_width(instance: DOMReference, value: DOMReference);
}

pub fn get_outer_width(instance: DOMReference) -> DOMReference {
    unsafe { window_get_outer_width(instance) }
}

pub fn set_outer_width(instance: DOMReference, value: DOMReference) {
    unsafe {
        window_set_outer_width(instance, value);
    }
}
extern "C" {
    fn window_get_outer_height(instance: DOMReference) -> DOMReference;
    fn window_set_outer_height(instance: DOMReference, value: DOMReference);
}

pub fn get_outer_height(instance: DOMReference) -> DOMReference {
    unsafe { window_get_outer_height(instance) }
}

pub fn set_outer_height(instance: DOMReference, value: DOMReference) {
    unsafe {
        window_set_outer_height(instance, value);
    }
}
extern "C" {
    fn window_get_device_pixel_ratio(instance: DOMReference) -> f32;
    fn window_set_device_pixel_ratio(instance: DOMReference, value: f32);
}

pub fn get_device_pixel_ratio(instance: DOMReference) -> f32 {
    unsafe { window_get_device_pixel_ratio(instance) }
}

pub fn set_device_pixel_ratio(instance: DOMReference, value: f32) {
    unsafe {
        window_set_device_pixel_ratio(instance, value);
    }
}
extern "C" {
    fn window_request_animation_frame(
        instance: DOMReference,
        request_animation_frame: DOMReference,
    ) -> f32;
}

pub fn request_animation_frame(instance: DOMReference, callback: DOMReference) -> f32 {
    unsafe { window_request_animation_frame(instance, callback) }
}
extern "C" {
    fn window_cancel_animation_frame(instance: DOMReference, cancel_animation_frame: f32);
}

pub fn cancel_animation_frame(instance: DOMReference, handle: f32) {
    unsafe { window_cancel_animation_frame(instance, handle) }
}
extern "C" {
    fn window_get_performance(instance: DOMReference) -> DOMReference;
    fn window_set_performance(instance: DOMReference, value: DOMReference);
}

pub fn get_performance(instance: DOMReference) -> DOMReference {
    unsafe { window_get_performance(instance) }
}

pub fn set_performance(instance: DOMReference, value: DOMReference) {
    unsafe {
        window_set_performance(instance, value);
    }
}
extern "C" {
    fn window_get_orientation(instance: DOMReference) -> f32;
    fn window_set_orientation(instance: DOMReference, value: f32);
}

pub fn get_orientation(instance: DOMReference) -> f32 {
    unsafe { window_get_orientation(instance) }
}

pub fn set_orientation(instance: DOMReference, value: f32) {
    unsafe {
        window_set_orientation(instance, value);
    }
}
extern "C" {
    fn window_get_onorientationchange(instance: DOMReference) -> DOMReference;
    fn window_set_onorientationchange(instance: DOMReference, value: DOMReference);
}

pub fn get_onorientationchange(instance: DOMReference) -> DOMReference {
    unsafe { window_get_onorientationchange(instance) }
}

pub fn set_onorientationchange(instance: DOMReference, value: DOMReference) {
    unsafe {
        window_set_onorientationchange(instance, value);
    }
}
extern "C" {
    fn window_get_onvrdisplayconnect(instance: DOMReference) -> DOMReference;
    fn window_set_onvrdisplayconnect(instance: DOMReference, value: DOMReference);
}

pub fn get_onvrdisplayconnect(instance: DOMReference) -> DOMReference {
    unsafe { window_get_onvrdisplayconnect(instance) }
}

pub fn set_onvrdisplayconnect(instance: DOMReference, value: DOMReference) {
    unsafe {
        window_set_onvrdisplayconnect(instance, value);
    }
}
extern "C" {
    fn window_get_onvrdisplaydisconnect(instance: DOMReference) -> DOMReference;
    fn window_set_onvrdisplaydisconnect(instance: DOMReference, value: DOMReference);
}

pub fn get_onvrdisplaydisconnect(instance: DOMReference) -> DOMReference {
    unsafe { window_get_onvrdisplaydisconnect(instance) }
}

pub fn set_onvrdisplaydisconnect(instance: DOMReference, value: DOMReference) {
    unsafe {
        window_set_onvrdisplaydisconnect(instance, value);
    }
}
extern "C" {
    fn window_get_onvrdisplayactivate(instance: DOMReference) -> DOMReference;
    fn window_set_onvrdisplayactivate(instance: DOMReference, value: DOMReference);
}

pub fn get_onvrdisplayactivate(instance: DOMReference) -> DOMReference {
    unsafe { window_get_onvrdisplayactivate(instance) }
}

pub fn set_onvrdisplayactivate(instance: DOMReference, value: DOMReference) {
    unsafe {
        window_set_onvrdisplayactivate(instance, value);
    }
}
extern "C" {
    fn window_get_onvrdisplaydeactivate(instance: DOMReference) -> DOMReference;
    fn window_set_onvrdisplaydeactivate(instance: DOMReference, value: DOMReference);
}

pub fn get_onvrdisplaydeactivate(instance: DOMReference) -> DOMReference {
    unsafe { window_get_onvrdisplaydeactivate(instance) }
}

pub fn set_onvrdisplaydeactivate(instance: DOMReference, value: DOMReference) {
    unsafe {
        window_set_onvrdisplaydeactivate(instance, value);
    }
}
extern "C" {
    fn window_get_onvrdisplaypresentchange(instance: DOMReference) -> DOMReference;
    fn window_set_onvrdisplaypresentchange(instance: DOMReference, value: DOMReference);
}

pub fn get_onvrdisplaypresentchange(instance: DOMReference) -> DOMReference {
    unsafe { window_get_onvrdisplaypresentchange(instance) }
}

pub fn set_onvrdisplaypresentchange(instance: DOMReference, value: DOMReference) {
    unsafe {
        window_set_onvrdisplaypresentchange(instance, value);
    }
}
extern "C" {
    fn window_get_paint_worklet(instance: DOMReference) -> DOMReference;
    fn window_set_paint_worklet(instance: DOMReference, value: DOMReference);
}

pub fn get_paint_worklet(instance: DOMReference) -> DOMReference {
    unsafe { window_get_paint_worklet(instance) }
}

pub fn set_paint_worklet(instance: DOMReference, value: DOMReference) {
    unsafe {
        window_set_paint_worklet(instance, value);
    }
}
extern "C" {
    fn window_request_idle_callback(
        instance: DOMReference,
        request_idle_callback: DOMReference,
        request_idle_callback: DOMReference,
    ) -> f32;
}

pub fn request_idle_callback(
    instance: DOMReference,
    callback: DOMReference,
    options: DOMReference,
) -> f32 {
    unsafe { window_request_idle_callback(instance, callback, options) }
}
extern "C" {
    fn window_cancel_idle_callback(instance: DOMReference, cancel_idle_callback: f32);
}

pub fn cancel_idle_callback(instance: DOMReference, handle: f32) {
    unsafe { window_cancel_idle_callback(instance, handle) }
}
extern "C" {
    fn window_get_origin(instance: DOMReference) -> DOMReference;
    fn window_set_origin(instance: DOMReference, value: DOMReference);
}

pub fn get_origin(instance: DOMReference) -> DOMReference {
    unsafe { window_get_origin(instance) }
}

pub fn set_origin(instance: DOMReference, value: DOMReference) {
    unsafe {
        window_set_origin(instance, value);
    }
}
extern "C" {
    fn window_btoa(instance: DOMReference, btoa: CString) -> CString;
}

pub fn btoa(instance: DOMReference, btoa: &str) -> String {
    unsafe { to_string(window_btoa(instance, to_cstring(btoa))) }
}
extern "C" {
    fn window_atob(instance: DOMReference, atob: CString) -> CString;
}

pub fn atob(instance: DOMReference, atob: &str) -> String {
    unsafe { to_string(window_atob(instance, to_cstring(atob))) }
}
extern "C" {
    fn window_set_timeout(
        instance: DOMReference,
        set_timeout: DOMReference,
        set_timeout: f32,
    ) -> f32;
}

pub fn set_timeout(instance: DOMReference, handler: DOMReference, timeout: f32) -> f32 {
    unsafe { window_set_timeout(instance, handler, timeout) }
}
extern "C" {
    fn window_clear_timeout(instance: DOMReference, clear_timeout: f32);
}

pub fn clear_timeout(instance: DOMReference, handle: f32) {
    unsafe { window_clear_timeout(instance, handle) }
}
extern "C" {
    fn window_set_interval(
        instance: DOMReference,
        set_interval: DOMReference,
        set_interval: f32,
    ) -> f32;
}

pub fn set_interval(instance: DOMReference, handler: DOMReference, timeout: f32) -> f32 {
    unsafe { window_set_interval(instance, handler, timeout) }
}
extern "C" {
    fn window_clear_interval(instance: DOMReference, clear_interval: f32);
}

pub fn clear_interval(instance: DOMReference, handle: f32) {
    unsafe { window_clear_interval(instance, handle) }
}
extern "C" {
    fn window_create_image_bitmap(
        instance: DOMReference,
        create_image_bitmap: DOMReference,
        create_image_bitmap: f32,
        create_image_bitmap: f32,
        create_image_bitmap: f32,
        create_image_bitmap: f32,
    ) -> DOMReference;
}

pub fn create_image_bitmap(
    instance: DOMReference,
    a_image: DOMReference,
    a_sx: f32,
    a_sy: f32,
    a_sw: f32,
    a_sh: f32,
) -> DOMReference {
    unsafe { window_create_image_bitmap(instance, a_image, a_sx, a_sy, a_sw, a_sh) }
}
