#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn htmlcanvaselement_get_width(instance: DOMReference) -> i32;
    fn htmlcanvaselement_set_width(instance: DOMReference, value: i32);
}

pub fn get_width(instance: DOMReference) -> i32 {
    unsafe { htmlcanvaselement_get_width(instance) }
}

pub fn set_width(instance: DOMReference, value: i32) {
    unsafe {
        htmlcanvaselement_set_width(instance, value);
    }
}
extern "C" {
    fn htmlcanvaselement_get_height(instance: DOMReference) -> i32;
    fn htmlcanvaselement_set_height(instance: DOMReference, value: i32);
}

pub fn get_height(instance: DOMReference) -> i32 {
    unsafe { htmlcanvaselement_get_height(instance) }
}

pub fn set_height(instance: DOMReference, value: i32) {
    unsafe {
        htmlcanvaselement_set_height(instance, value);
    }
}
extern "C" {
    fn htmlcanvaselement_get_context(instance: i32, context_id: CString) -> i32;
}

pub fn get_context(instance: i32, context_id: &str) -> i32 {
    unsafe { htmlcanvaselement_get_context(instance, to_cstring(context_id)) }
}
extern "C" {
    fn htmlcanvaselement_to_data_url(
        instance: i32,
        data_type: CString,
        encoder_options: i32,
    ) -> CString;
}

pub fn to_data_url(instance: i32, data_type: &str, encoder_options: i32) -> String {
    unsafe {
        to_string(htmlcanvaselement_to_data_url(
            instance,
            to_cstring(data_type),
            encoder_options,
        ))
    }
}
extern "C" {
    fn htmlcanvaselement_to_blob(
        instance: i32,
        callback: i32,
        blob_type: CString,
        encoder_options: i32,
    );
}

pub fn to_blob(instance: i32, callback: i32, blob_type: &str, encoder_options: i32) {
    unsafe { htmlcanvaselement_to_blob(instance, callback, to_cstring(blob_type), encoder_options) }
}
extern "C" {
    fn htmlcanvaselement_transfer_control_to_offscreen(instance: i32) -> i32;
}

pub fn transfer_control_to_offscreen(instance: i32) -> i32 {
    unsafe { htmlcanvaselement_transfer_control_to_offscreen(instance) }
}
