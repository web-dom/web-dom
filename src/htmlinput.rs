#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn htmlinput_get_accept(instance: DOMReference) -> CString;
    fn htmlinput_set_accept(instance: DOMReference, value: CString);
}

pub fn get_accept(instance: DOMReference) -> String {
    unsafe { to_string(htmlinput_get_accept(instance)) }
}

pub fn set_accept(instance: DOMReference, value: &str) {
    unsafe {
        htmlinput_set_accept(instance, to_cstring(value));
    }
}
extern "C" {
    fn htmlinput_get_alt(instance: DOMReference) -> CString;
    fn htmlinput_set_alt(instance: DOMReference, value: CString);
}

pub fn get_alt(instance: DOMReference) -> String {
    unsafe { to_string(htmlinput_get_alt(instance)) }
}

pub fn set_alt(instance: DOMReference, value: &str) {
    unsafe {
        htmlinput_set_alt(instance, to_cstring(value));
    }
}
extern "C" {
    fn htmlinput_get_autocomplete(instance: DOMReference) -> CString;
    fn htmlinput_set_autocomplete(instance: DOMReference, value: CString);
}

pub fn get_autocomplete(instance: DOMReference) -> String {
    unsafe { to_string(htmlinput_get_autocomplete(instance)) }
}

pub fn set_autocomplete(instance: DOMReference, value: &str) {
    unsafe {
        htmlinput_set_autocomplete(instance, to_cstring(value));
    }
}
extern "C" {
    fn htmlinput_get_autofocus(instance: DOMReference) -> i32;
    fn htmlinput_set_autofocus(instance: DOMReference, value: i32);
}

pub fn get_autofocus(instance: DOMReference) -> bool {
    unsafe { 0 != htmlinput_get_autofocus(instance) }
}

pub fn set_autofocus(instance: DOMReference, value: bool) {
    unsafe {
        htmlinput_set_autofocus(instance, if value == true { 1 } else { 0 });
    }
}
extern "C" {
    fn htmlinput_get_default_checked(instance: DOMReference) -> i32;
    fn htmlinput_set_default_checked(instance: DOMReference, value: i32);
}

pub fn get_default_checked(instance: DOMReference) -> bool {
    unsafe { 0 != htmlinput_get_default_checked(instance) }
}

pub fn set_default_checked(instance: DOMReference, value: bool) {
    unsafe {
        htmlinput_set_default_checked(instance, if value == true { 1 } else { 0 });
    }
}
extern "C" {
    fn htmlinput_get_checked(instance: DOMReference) -> i32;
    fn htmlinput_set_checked(instance: DOMReference, value: i32);
}

pub fn get_checked(instance: DOMReference) -> bool {
    unsafe { 0 != htmlinput_get_checked(instance) }
}

pub fn set_checked(instance: DOMReference, value: bool) {
    unsafe {
        htmlinput_set_checked(instance, if value == true { 1 } else { 0 });
    }
}
extern "C" {
    fn htmlinput_get_disabled(instance: DOMReference) -> i32;
    fn htmlinput_set_disabled(instance: DOMReference, value: i32);
}

pub fn get_disabled(instance: DOMReference) -> bool {
    unsafe { 0 != htmlinput_get_disabled(instance) }
}

pub fn set_disabled(instance: DOMReference, value: bool) {
    unsafe {
        htmlinput_set_disabled(instance, if value == true { 1 } else { 0 });
    }
}
extern "C" {
    fn htmlinput_get_form(instance: DOMReference) -> DOMReference;
    fn htmlinput_set_form(instance: DOMReference, value: DOMReference);
}

pub fn get_form(instance: DOMReference) -> DOMReference {
    unsafe { htmlinput_get_form(instance) }
}

pub fn set_form(instance: DOMReference, value: DOMReference) {
    unsafe {
        htmlinput_set_form(instance, value);
    }
}
extern "C" {
    fn htmlinput_get_files(instance: DOMReference) -> DOMReference;
    fn htmlinput_set_files(instance: DOMReference, value: DOMReference);
}

pub fn get_files(instance: DOMReference) -> DOMReference {
    unsafe { htmlinput_get_files(instance) }
}

pub fn set_files(instance: DOMReference, value: DOMReference) {
    unsafe {
        htmlinput_set_files(instance, value);
    }
}
extern "C" {
    fn htmlinput_get_form_action(instance: DOMReference) -> CString;
    fn htmlinput_set_form_action(instance: DOMReference, value: CString);
}

pub fn get_form_action(instance: DOMReference) -> String {
    unsafe { to_string(htmlinput_get_form_action(instance)) }
}

pub fn set_form_action(instance: DOMReference, value: &str) {
    unsafe {
        htmlinput_set_form_action(instance, to_cstring(value));
    }
}
extern "C" {
    fn htmlinput_get_form_enctype(instance: DOMReference) -> CString;
    fn htmlinput_set_form_enctype(instance: DOMReference, value: CString);
}

pub fn get_form_enctype(instance: DOMReference) -> String {
    unsafe { to_string(htmlinput_get_form_enctype(instance)) }
}

pub fn set_form_enctype(instance: DOMReference, value: &str) {
    unsafe {
        htmlinput_set_form_enctype(instance, to_cstring(value));
    }
}
extern "C" {
    fn htmlinput_get_form_method(instance: DOMReference) -> CString;
    fn htmlinput_set_form_method(instance: DOMReference, value: CString);
}

pub fn get_form_method(instance: DOMReference) -> String {
    unsafe { to_string(htmlinput_get_form_method(instance)) }
}

pub fn set_form_method(instance: DOMReference, value: &str) {
    unsafe {
        htmlinput_set_form_method(instance, to_cstring(value));
    }
}
extern "C" {
    fn htmlinput_get_form_no_validate(instance: DOMReference) -> i32;
    fn htmlinput_set_form_no_validate(instance: DOMReference, value: i32);
}

pub fn get_form_no_validate(instance: DOMReference) -> bool {
    unsafe { 0 != htmlinput_get_form_no_validate(instance) }
}

pub fn set_form_no_validate(instance: DOMReference, value: bool) {
    unsafe {
        htmlinput_set_form_no_validate(instance, if value == true { 1 } else { 0 });
    }
}
extern "C" {
    fn htmlinput_get_form_target(instance: DOMReference) -> CString;
    fn htmlinput_set_form_target(instance: DOMReference, value: CString);
}

pub fn get_form_target(instance: DOMReference) -> String {
    unsafe { to_string(htmlinput_get_form_target(instance)) }
}

pub fn set_form_target(instance: DOMReference, value: &str) {
    unsafe {
        htmlinput_set_form_target(instance, to_cstring(value));
    }
}
extern "C" {
    fn htmlinput_get_height(instance: DOMReference) -> f32;
    fn htmlinput_set_height(instance: DOMReference, value: f32);
}

pub fn get_height(instance: DOMReference) -> f32 {
    unsafe { htmlinput_get_height(instance) }
}

pub fn set_height(instance: DOMReference, value: f32) {
    unsafe {
        htmlinput_set_height(instance, value);
    }
}
extern "C" {
    fn htmlinput_get_indeterminate(instance: DOMReference) -> i32;
    fn htmlinput_set_indeterminate(instance: DOMReference, value: i32);
}

pub fn get_indeterminate(instance: DOMReference) -> bool {
    unsafe { 0 != htmlinput_get_indeterminate(instance) }
}

pub fn set_indeterminate(instance: DOMReference, value: bool) {
    unsafe {
        htmlinput_set_indeterminate(instance, if value == true { 1 } else { 0 });
    }
}
extern "C" {
    fn htmlinput_get_input_mode(instance: DOMReference) -> CString;
    fn htmlinput_set_input_mode(instance: DOMReference, value: CString);
}

pub fn get_input_mode(instance: DOMReference) -> String {
    unsafe { to_string(htmlinput_get_input_mode(instance)) }
}

pub fn set_input_mode(instance: DOMReference, value: &str) {
    unsafe {
        htmlinput_set_input_mode(instance, to_cstring(value));
    }
}
extern "C" {
    fn htmlinput_get_list(instance: DOMReference) -> DOMReference;
    fn htmlinput_set_list(instance: DOMReference, value: DOMReference);
}

pub fn get_list(instance: DOMReference) -> DOMReference {
    unsafe { htmlinput_get_list(instance) }
}

pub fn set_list(instance: DOMReference, value: DOMReference) {
    unsafe {
        htmlinput_set_list(instance, value);
    }
}
extern "C" {
    fn htmlinput_get_max(instance: DOMReference) -> CString;
    fn htmlinput_set_max(instance: DOMReference, value: CString);
}

pub fn get_max(instance: DOMReference) -> String {
    unsafe { to_string(htmlinput_get_max(instance)) }
}

pub fn set_max(instance: DOMReference, value: &str) {
    unsafe {
        htmlinput_set_max(instance, to_cstring(value));
    }
}
extern "C" {
    fn htmlinput_get_max_length(instance: DOMReference) -> f32;
    fn htmlinput_set_max_length(instance: DOMReference, value: f32);
}

pub fn get_max_length(instance: DOMReference) -> f32 {
    unsafe { htmlinput_get_max_length(instance) }
}

pub fn set_max_length(instance: DOMReference, value: f32) {
    unsafe {
        htmlinput_set_max_length(instance, value);
    }
}
extern "C" {
    fn htmlinput_get_min(instance: DOMReference) -> CString;
    fn htmlinput_set_min(instance: DOMReference, value: CString);
}

pub fn get_min(instance: DOMReference) -> String {
    unsafe { to_string(htmlinput_get_min(instance)) }
}

pub fn set_min(instance: DOMReference, value: &str) {
    unsafe {
        htmlinput_set_min(instance, to_cstring(value));
    }
}
extern "C" {
    fn htmlinput_get_min_length(instance: DOMReference) -> f32;
    fn htmlinput_set_min_length(instance: DOMReference, value: f32);
}

pub fn get_min_length(instance: DOMReference) -> f32 {
    unsafe { htmlinput_get_min_length(instance) }
}

pub fn set_min_length(instance: DOMReference, value: f32) {
    unsafe {
        htmlinput_set_min_length(instance, value);
    }
}
extern "C" {
    fn htmlinput_get_multiple(instance: DOMReference) -> i32;
    fn htmlinput_set_multiple(instance: DOMReference, value: i32);
}

pub fn get_multiple(instance: DOMReference) -> bool {
    unsafe { 0 != htmlinput_get_multiple(instance) }
}

pub fn set_multiple(instance: DOMReference, value: bool) {
    unsafe {
        htmlinput_set_multiple(instance, if value == true { 1 } else { 0 });
    }
}
extern "C" {
    fn htmlinput_get_name(instance: DOMReference) -> CString;
    fn htmlinput_set_name(instance: DOMReference, value: CString);
}

pub fn get_name(instance: DOMReference) -> String {
    unsafe { to_string(htmlinput_get_name(instance)) }
}

pub fn set_name(instance: DOMReference, value: &str) {
    unsafe {
        htmlinput_set_name(instance, to_cstring(value));
    }
}
extern "C" {
    fn htmlinput_get_pattern(instance: DOMReference) -> CString;
    fn htmlinput_set_pattern(instance: DOMReference, value: CString);
}

pub fn get_pattern(instance: DOMReference) -> String {
    unsafe { to_string(htmlinput_get_pattern(instance)) }
}

pub fn set_pattern(instance: DOMReference, value: &str) {
    unsafe {
        htmlinput_set_pattern(instance, to_cstring(value));
    }
}
extern "C" {
    fn htmlinput_get_placeholder(instance: DOMReference) -> CString;
    fn htmlinput_set_placeholder(instance: DOMReference, value: CString);
}

pub fn get_placeholder(instance: DOMReference) -> String {
    unsafe { to_string(htmlinput_get_placeholder(instance)) }
}

pub fn set_placeholder(instance: DOMReference, value: &str) {
    unsafe {
        htmlinput_set_placeholder(instance, to_cstring(value));
    }
}
extern "C" {
    fn htmlinput_get_read_only(instance: DOMReference) -> i32;
    fn htmlinput_set_read_only(instance: DOMReference, value: i32);
}

pub fn get_read_only(instance: DOMReference) -> bool {
    unsafe { 0 != htmlinput_get_read_only(instance) }
}

pub fn set_read_only(instance: DOMReference, value: bool) {
    unsafe {
        htmlinput_set_read_only(instance, if value == true { 1 } else { 0 });
    }
}
extern "C" {
    fn htmlinput_get_required(instance: DOMReference) -> i32;
    fn htmlinput_set_required(instance: DOMReference, value: i32);
}

pub fn get_required(instance: DOMReference) -> bool {
    unsafe { 0 != htmlinput_get_required(instance) }
}

pub fn set_required(instance: DOMReference, value: bool) {
    unsafe {
        htmlinput_set_required(instance, if value == true { 1 } else { 0 });
    }
}
extern "C" {
    fn htmlinput_get_size(instance: DOMReference) -> f32;
    fn htmlinput_set_size(instance: DOMReference, value: f32);
}

pub fn get_size(instance: DOMReference) -> f32 {
    unsafe { htmlinput_get_size(instance) }
}

pub fn set_size(instance: DOMReference, value: f32) {
    unsafe {
        htmlinput_set_size(instance, value);
    }
}
extern "C" {
    fn htmlinput_get_src(instance: DOMReference) -> CString;
    fn htmlinput_set_src(instance: DOMReference, value: CString);
}

pub fn get_src(instance: DOMReference) -> String {
    unsafe { to_string(htmlinput_get_src(instance)) }
}

pub fn set_src(instance: DOMReference, value: &str) {
    unsafe {
        htmlinput_set_src(instance, to_cstring(value));
    }
}
extern "C" {
    fn htmlinput_get_step(instance: DOMReference) -> CString;
    fn htmlinput_set_step(instance: DOMReference, value: CString);
}

pub fn get_step(instance: DOMReference) -> String {
    unsafe { to_string(htmlinput_get_step(instance)) }
}

pub fn set_step(instance: DOMReference, value: &str) {
    unsafe {
        htmlinput_set_step(instance, to_cstring(value));
    }
}
extern "C" {
    fn htmlinput_get_type(instance: DOMReference) -> CString;
    fn htmlinput_set_type(instance: DOMReference, value: CString);
}

pub fn get_type(instance: DOMReference) -> String {
    unsafe { to_string(htmlinput_get_type(instance)) }
}

pub fn set_type(instance: DOMReference, value: &str) {
    unsafe {
        htmlinput_set_type(instance, to_cstring(value));
    }
}
extern "C" {
    fn htmlinput_get_default_value(instance: DOMReference) -> CString;
    fn htmlinput_set_default_value(instance: DOMReference, value: CString);
}

pub fn get_default_value(instance: DOMReference) -> String {
    unsafe { to_string(htmlinput_get_default_value(instance)) }
}

pub fn set_default_value(instance: DOMReference, value: &str) {
    unsafe {
        htmlinput_set_default_value(instance, to_cstring(value));
    }
}
extern "C" {
    fn htmlinput_get_value(instance: DOMReference) -> CString;
    fn htmlinput_set_value(instance: DOMReference, value: CString);
}

pub fn get_value(instance: DOMReference) -> String {
    unsafe { to_string(htmlinput_get_value(instance)) }
}

pub fn set_value(instance: DOMReference, value: &str) {
    unsafe {
        htmlinput_set_value(instance, to_cstring(value));
    }
}
extern "C" {
    fn htmlinput_get_value_as_date(instance: DOMReference) -> DOMReference;
    fn htmlinput_set_value_as_date(instance: DOMReference, value: DOMReference);
}

pub fn get_value_as_date(instance: DOMReference) -> DOMReference {
    unsafe { htmlinput_get_value_as_date(instance) }
}

pub fn set_value_as_date(instance: DOMReference, value: DOMReference) {
    unsafe {
        htmlinput_set_value_as_date(instance, value);
    }
}
extern "C" {
    fn htmlinput_get_value_as_number(instance: DOMReference) -> f32;
    fn htmlinput_set_value_as_number(instance: DOMReference, value: f32);
}

pub fn get_value_as_number(instance: DOMReference) -> f32 {
    unsafe { htmlinput_get_value_as_number(instance) }
}

pub fn set_value_as_number(instance: DOMReference, value: f32) {
    unsafe {
        htmlinput_set_value_as_number(instance, value);
    }
}
extern "C" {
    fn htmlinput_get_width(instance: DOMReference) -> f32;
    fn htmlinput_set_width(instance: DOMReference, value: f32);
}

pub fn get_width(instance: DOMReference) -> f32 {
    unsafe { htmlinput_get_width(instance) }
}

pub fn set_width(instance: DOMReference, value: f32) {
    unsafe {
        htmlinput_set_width(instance, value);
    }
}
extern "C" {
    fn htmlinput_get_will_validate(instance: DOMReference) -> i32;
    fn htmlinput_set_will_validate(instance: DOMReference, value: i32);
}

pub fn get_will_validate(instance: DOMReference) -> bool {
    unsafe { 0 != htmlinput_get_will_validate(instance) }
}

pub fn set_will_validate(instance: DOMReference, value: bool) {
    unsafe {
        htmlinput_set_will_validate(instance, if value == true { 1 } else { 0 });
    }
}
extern "C" {
    fn htmlinput_get_validity(instance: DOMReference) -> DOMReference;
    fn htmlinput_set_validity(instance: DOMReference, value: DOMReference);
}

pub fn get_validity(instance: DOMReference) -> DOMReference {
    unsafe { htmlinput_get_validity(instance) }
}

pub fn set_validity(instance: DOMReference, value: DOMReference) {
    unsafe {
        htmlinput_set_validity(instance, value);
    }
}
extern "C" {
    fn htmlinput_get_validation_message(instance: DOMReference) -> CString;
    fn htmlinput_set_validation_message(instance: DOMReference, value: CString);
}

pub fn get_validation_message(instance: DOMReference) -> String {
    unsafe { to_string(htmlinput_get_validation_message(instance)) }
}

pub fn set_validation_message(instance: DOMReference, value: &str) {
    unsafe {
        htmlinput_set_validation_message(instance, to_cstring(value));
    }
}
extern "C" {
    fn htmlinput_check_validity(instance: DOMReference) -> i32;
}

pub fn check_validity(instance: DOMReference) -> bool {
    unsafe { 0 != htmlinput_check_validity(instance) }
}
extern "C" {
    fn htmlinput_report_validity(instance: DOMReference) -> i32;
}

pub fn report_validity(instance: DOMReference) -> bool {
    unsafe { 0 != htmlinput_report_validity(instance) }
}
extern "C" {
    fn htmlinput_set_custom_validity(instance: DOMReference, set_custom_validity: CString);
}

pub fn set_custom_validity(instance: DOMReference, error: &str) {
    unsafe { htmlinput_set_custom_validity(instance, to_cstring(error)) }
}
extern "C" {
    fn htmlinput_get_labels(instance: DOMReference) -> DOMReference;
    fn htmlinput_set_labels(instance: DOMReference, value: DOMReference);
}

pub fn get_labels(instance: DOMReference) -> DOMReference {
    unsafe { htmlinput_get_labels(instance) }
}

pub fn set_labels(instance: DOMReference, value: DOMReference) {
    unsafe {
        htmlinput_set_labels(instance, value);
    }
}
extern "C" {
    fn htmlinput_select(instance: DOMReference);
}

pub fn select(instance: DOMReference) {
    unsafe { htmlinput_select(instance) }
}
extern "C" {
    fn htmlinput_get_selection_direction(instance: DOMReference) -> CString;
    fn htmlinput_set_selection_direction(instance: DOMReference, value: CString);
}

pub fn get_selection_direction(instance: DOMReference) -> String {
    unsafe { to_string(htmlinput_get_selection_direction(instance)) }
}

pub fn set_selection_direction(instance: DOMReference, value: &str) {
    unsafe {
        htmlinput_set_selection_direction(instance, to_cstring(value));
    }
}
extern "C" {
    fn htmlinput_set_range_text(
        instance: DOMReference,
        set_range_text: CString,
        set_range_text: f32,
        set_range_text: f32,
        set_range_text: DOMReference,
    );
}

pub fn set_range_text(
    instance: DOMReference,
    replacement: &str,
    start: f32,
    end: f32,
    selection_mode: DOMReference,
) {
    unsafe {
        htmlinput_set_range_text(
            instance,
            to_cstring(replacement),
            start,
            end,
            selection_mode,
        )
    }
}
extern "C" {
    fn htmlinput_set_selection_range(
        instance: DOMReference,
        set_selection_range: f32,
        set_selection_range: f32,
        set_selection_range: CString,
    );
}

pub fn set_selection_range(instance: DOMReference, start: f32, end: f32, direction: &str) {
    unsafe { htmlinput_set_selection_range(instance, start, end, to_cstring(direction)) }
}
extern "C" {
    fn htmlinput_get_align(instance: DOMReference) -> CString;
    fn htmlinput_set_align(instance: DOMReference, value: CString);
}

pub fn get_align(instance: DOMReference) -> String {
    unsafe { to_string(htmlinput_get_align(instance)) }
}

pub fn set_align(instance: DOMReference, value: &str) {
    unsafe {
        htmlinput_set_align(instance, to_cstring(value));
    }
}
extern "C" {
    fn htmlinput_get_use_map(instance: DOMReference) -> CString;
    fn htmlinput_set_use_map(instance: DOMReference, value: CString);
}

pub fn get_use_map(instance: DOMReference) -> String {
    unsafe { to_string(htmlinput_get_use_map(instance)) }
}

pub fn set_use_map(instance: DOMReference, value: &str) {
    unsafe {
        htmlinput_set_use_map(instance, to_cstring(value));
    }
}
extern "C" {
    fn htmlinput_get_date_time_input_box_value(instance: DOMReference) -> DOMReference;
}

pub fn get_date_time_input_box_value(instance: DOMReference) -> DOMReference {
    unsafe { htmlinput_get_date_time_input_box_value(instance) }
}
extern "C" {
    fn htmlinput_update_date_time_input_box(
        instance: DOMReference,
        update_date_time_input_box: DOMReference,
    );
}

pub fn update_date_time_input_box(instance: DOMReference, value: DOMReference) {
    unsafe { htmlinput_update_date_time_input_box(instance, value) }
}
extern "C" {
    fn htmlinput_set_date_time_picker_state(
        instance: DOMReference,
        set_date_time_picker_state: i32,
    );
}

pub fn set_date_time_picker_state(instance: DOMReference, open: bool) {
    unsafe { htmlinput_set_date_time_picker_state(instance, if open { 1 } else { 0 }) }
}
extern "C" {
    fn htmlinput_get_minimum(instance: DOMReference) -> f32;
}

pub fn get_minimum(instance: DOMReference) -> f32 {
    unsafe { htmlinput_get_minimum(instance) }
}
extern "C" {
    fn htmlinput_get_maximum(instance: DOMReference) -> f32;
}

pub fn get_maximum(instance: DOMReference) -> f32 {
    unsafe { htmlinput_get_maximum(instance) }
}
extern "C" {
    fn htmlinput_get_preview_value(instance: DOMReference) -> CString;
    fn htmlinput_set_preview_value(instance: DOMReference, value: CString);
}

pub fn get_preview_value(instance: DOMReference) -> String {
    unsafe { to_string(htmlinput_get_preview_value(instance)) }
}

pub fn set_preview_value(instance: DOMReference, value: &str) {
    unsafe {
        htmlinput_set_preview_value(instance, to_cstring(value));
    }
}
