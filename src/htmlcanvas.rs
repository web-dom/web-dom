#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn htmlcanvas_get_width(instance: DOMReference) -> f32;
    fn htmlcanvas_set_width(instance: DOMReference, value: f32);
}

pub fn get_width(instance: DOMReference) -> f32 {
    unsafe { htmlcanvas_get_width(instance) }
}

pub fn set_width(instance: DOMReference, value: f32) {
    unsafe {
        htmlcanvas_set_width(instance, value);
    }
}
extern "C" {
    fn htmlcanvas_get_height(instance: DOMReference) -> f32;
    fn htmlcanvas_set_height(instance: DOMReference, value: f32);
}

pub fn get_height(instance: DOMReference) -> f32 {
    unsafe { htmlcanvas_get_height(instance) }
}

pub fn set_height(instance: DOMReference, value: f32) {
    unsafe {
        htmlcanvas_set_height(instance, value);
    }
}
extern "C" {
    fn htmlcanvas_get_context(instance: DOMReference, get_context: CString) -> DOMReference;
}

pub fn get_context(instance: DOMReference, context_id: &str) -> DOMReference {
    unsafe { htmlcanvas_get_context(instance, to_cstring(context_id)) }
}
extern "C" {
    fn htmlcanvas_to_data_url(
        instance: DOMReference,
        to_data_url: CString,
        to_data_url: DOMReference,
    ) -> CString;
}

pub fn to_data_url(
    instance: DOMReference,
    data_type: &str,
    encoder_options: DOMReference,
) -> String {
    unsafe {
        to_string(htmlcanvas_to_data_url(
            instance,
            to_cstring(data_type),
            encoder_options,
        ))
    }
}
extern "C" {
    fn htmlcanvas_to_blob(
        instance: DOMReference,
        to_blob: DOMReference,
        to_blob: CString,
        to_blob: DOMReference,
    );
}

pub fn to_blob(
    instance: DOMReference,
    callback: DOMReference,
    blob_type: &str,
    encoder_options: DOMReference,
) {
    unsafe { htmlcanvas_to_blob(instance, callback, to_cstring(blob_type), encoder_options) }
}
extern "C" {
    fn htmlcanvas_transfer_control_to_offscreen(instance: DOMReference) -> DOMReference;
}

pub fn transfer_control_to_offscreen(instance: DOMReference) -> DOMReference {
    unsafe { htmlcanvas_transfer_control_to_offscreen(instance) }
}
