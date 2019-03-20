#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn htmlinputelement_get_accept(instance: i32) -> CString;
    fn htmlinputelement_set_accept(instance: i32, value: CString);
}

pub fn get_accept(instance: i32) -> String {
    unsafe { cstr_to_string(htmlinputelement_get_accept(instance)) }
}

pub fn set_accept(instance: i32, value: &str) {
    unsafe {
        htmlinputelement_set_accept(instance, cstr(value));
    }
}
extern "C" {
    fn htmlinputelement_get_alt(instance: i32) -> CString;
    fn htmlinputelement_set_alt(instance: i32, value: CString);
}

pub fn get_alt(instance: i32) -> String {
    unsafe { cstr_to_string(htmlinputelement_get_alt(instance)) }
}

pub fn set_alt(instance: i32, value: &str) {
    unsafe {
        htmlinputelement_set_alt(instance, cstr(value));
    }
}
extern "C" {
    fn htmlinputelement_get_autocomplete(instance: i32) -> CString;
    fn htmlinputelement_set_autocomplete(instance: i32, value: CString);
}

pub fn get_autocomplete(instance: i32) -> String {
    unsafe { cstr_to_string(htmlinputelement_get_autocomplete(instance)) }
}

pub fn set_autocomplete(instance: i32, value: &str) {
    unsafe {
        htmlinputelement_set_autocomplete(instance, cstr(value));
    }
}
extern "C" {
    fn htmlinputelement_get_autofocus(instance: i32) -> i32;
    fn htmlinputelement_set_autofocus(instance: i32, value: i32);
}

pub fn get_autofocus(instance: i32) -> i32 {
    unsafe { htmlinputelement_get_autofocus(instance) }
}

pub fn set_autofocus(instance: i32, value: i32) {
    unsafe {
        htmlinputelement_set_autofocus(instance, value);
    }
}
extern "C" {
    fn htmlinputelement_get_defaultChecked(instance: i32) -> i32;
    fn htmlinputelement_set_defaultChecked(instance: i32, value: i32);
}

pub fn get_default_checked(instance: i32) -> i32 {
    unsafe { htmlinputelement_get_defaultChecked(instance) }
}

pub fn set_default_checked(instance: i32, value: i32) {
    unsafe {
        htmlinputelement_set_defaultChecked(instance, value);
    }
}
extern "C" {
    fn htmlinputelement_get_checked(instance: i32) -> i32;
    fn htmlinputelement_set_checked(instance: i32, value: i32);
}

pub fn get_checked(instance: i32) -> i32 {
    unsafe { htmlinputelement_get_checked(instance) }
}

pub fn set_checked(instance: i32, value: i32) {
    unsafe {
        htmlinputelement_set_checked(instance, value);
    }
}
extern "C" {
    fn htmlinputelement_get_disabled(instance: i32) -> i32;
    fn htmlinputelement_set_disabled(instance: i32, value: i32);
}

pub fn get_disabled(instance: i32) -> i32 {
    unsafe { htmlinputelement_get_disabled(instance) }
}

pub fn set_disabled(instance: i32, value: i32) {
    unsafe {
        htmlinputelement_set_disabled(instance, value);
    }
}
extern "C" {
    fn htmlinputelement_get_form(instance: i32) -> i32;
    fn htmlinputelement_set_form(instance: i32, value: i32);
}

pub fn get_form(instance: i32) -> i32 {
    unsafe { htmlinputelement_get_form(instance) }
}

pub fn set_form(instance: i32, value: i32) {
    unsafe {
        htmlinputelement_set_form(instance, value);
    }
}
extern "C" {
    fn htmlinputelement_get_files(instance: i32) -> i32;
    fn htmlinputelement_set_files(instance: i32, value: i32);
}

pub fn get_files(instance: i32) -> i32 {
    unsafe { htmlinputelement_get_files(instance) }
}

pub fn set_files(instance: i32, value: i32) {
    unsafe {
        htmlinputelement_set_files(instance, value);
    }
}
extern "C" {
    fn htmlinputelement_get_formAction(instance: i32) -> CString;
    fn htmlinputelement_set_formAction(instance: i32, value: CString);
}

pub fn get_form_action(instance: i32) -> String {
    unsafe { cstr_to_string(htmlinputelement_get_formAction(instance)) }
}

pub fn set_form_action(instance: i32, value: &str) {
    unsafe {
        htmlinputelement_set_formAction(instance, cstr(value));
    }
}
extern "C" {
    fn htmlinputelement_get_formEnctype(instance: i32) -> CString;
    fn htmlinputelement_set_formEnctype(instance: i32, value: CString);
}

pub fn get_form_enctype(instance: i32) -> String {
    unsafe { cstr_to_string(htmlinputelement_get_formEnctype(instance)) }
}

pub fn set_form_enctype(instance: i32, value: &str) {
    unsafe {
        htmlinputelement_set_formEnctype(instance, cstr(value));
    }
}
extern "C" {
    fn htmlinputelement_get_formMethod(instance: i32) -> CString;
    fn htmlinputelement_set_formMethod(instance: i32, value: CString);
}

pub fn get_form_method(instance: i32) -> String {
    unsafe { cstr_to_string(htmlinputelement_get_formMethod(instance)) }
}

pub fn set_form_method(instance: i32, value: &str) {
    unsafe {
        htmlinputelement_set_formMethod(instance, cstr(value));
    }
}
extern "C" {
    fn htmlinputelement_get_formNoValidate(instance: i32) -> i32;
    fn htmlinputelement_set_formNoValidate(instance: i32, value: i32);
}

pub fn get_form_no_validate(instance: i32) -> i32 {
    unsafe { htmlinputelement_get_formNoValidate(instance) }
}

pub fn set_form_no_validate(instance: i32, value: i32) {
    unsafe {
        htmlinputelement_set_formNoValidate(instance, value);
    }
}
extern "C" {
    fn htmlinputelement_get_formTarget(instance: i32) -> CString;
    fn htmlinputelement_set_formTarget(instance: i32, value: CString);
}

pub fn get_form_target(instance: i32) -> String {
    unsafe { cstr_to_string(htmlinputelement_get_formTarget(instance)) }
}

pub fn set_form_target(instance: i32, value: &str) {
    unsafe {
        htmlinputelement_set_formTarget(instance, cstr(value));
    }
}
extern "C" {
    fn htmlinputelement_get_height(instance: i32) -> i32;
    fn htmlinputelement_set_height(instance: i32, value: i32);
}

pub fn get_height(instance: i32) -> i32 {
    unsafe { htmlinputelement_get_height(instance) }
}

pub fn set_height(instance: i32, value: i32) {
    unsafe {
        htmlinputelement_set_height(instance, value);
    }
}
extern "C" {
    fn htmlinputelement_get_indeterminate(instance: i32) -> i32;
    fn htmlinputelement_set_indeterminate(instance: i32, value: i32);
}

pub fn get_indeterminate(instance: i32) -> i32 {
    unsafe { htmlinputelement_get_indeterminate(instance) }
}

pub fn set_indeterminate(instance: i32, value: i32) {
    unsafe {
        htmlinputelement_set_indeterminate(instance, value);
    }
}
extern "C" {
    fn htmlinputelement_get_inputMode(instance: i32) -> CString;
    fn htmlinputelement_set_inputMode(instance: i32, value: CString);
}

pub fn get_input_mode(instance: i32) -> String {
    unsafe { cstr_to_string(htmlinputelement_get_inputMode(instance)) }
}

pub fn set_input_mode(instance: i32, value: &str) {
    unsafe {
        htmlinputelement_set_inputMode(instance, cstr(value));
    }
}
extern "C" {
    fn htmlinputelement_get_list(instance: i32) -> i32;
    fn htmlinputelement_set_list(instance: i32, value: i32);
}

pub fn get_list(instance: i32) -> i32 {
    unsafe { htmlinputelement_get_list(instance) }
}

pub fn set_list(instance: i32, value: i32) {
    unsafe {
        htmlinputelement_set_list(instance, value);
    }
}
extern "C" {
    fn htmlinputelement_get_max(instance: i32) -> CString;
    fn htmlinputelement_set_max(instance: i32, value: CString);
}

pub fn get_max(instance: i32) -> String {
    unsafe { cstr_to_string(htmlinputelement_get_max(instance)) }
}

pub fn set_max(instance: i32, value: &str) {
    unsafe {
        htmlinputelement_set_max(instance, cstr(value));
    }
}
extern "C" {
    fn htmlinputelement_get_maxLength(instance: i32) -> i32;
    fn htmlinputelement_set_maxLength(instance: i32, value: i32);
}

pub fn get_max_length(instance: i32) -> i32 {
    unsafe { htmlinputelement_get_maxLength(instance) }
}

pub fn set_max_length(instance: i32, value: i32) {
    unsafe {
        htmlinputelement_set_maxLength(instance, value);
    }
}
extern "C" {
    fn htmlinputelement_get_min(instance: i32) -> CString;
    fn htmlinputelement_set_min(instance: i32, value: CString);
}

pub fn get_min(instance: i32) -> String {
    unsafe { cstr_to_string(htmlinputelement_get_min(instance)) }
}

pub fn set_min(instance: i32, value: &str) {
    unsafe {
        htmlinputelement_set_min(instance, cstr(value));
    }
}
extern "C" {
    fn htmlinputelement_get_minLength(instance: i32) -> i32;
    fn htmlinputelement_set_minLength(instance: i32, value: i32);
}

pub fn get_min_length(instance: i32) -> i32 {
    unsafe { htmlinputelement_get_minLength(instance) }
}

pub fn set_min_length(instance: i32, value: i32) {
    unsafe {
        htmlinputelement_set_minLength(instance, value);
    }
}
extern "C" {
    fn htmlinputelement_get_multiple(instance: i32) -> i32;
    fn htmlinputelement_set_multiple(instance: i32, value: i32);
}

pub fn get_multiple(instance: i32) -> i32 {
    unsafe { htmlinputelement_get_multiple(instance) }
}

pub fn set_multiple(instance: i32, value: i32) {
    unsafe {
        htmlinputelement_set_multiple(instance, value);
    }
}
extern "C" {
    fn htmlinputelement_get_name(instance: i32) -> CString;
    fn htmlinputelement_set_name(instance: i32, value: CString);
}

pub fn get_name(instance: i32) -> String {
    unsafe { cstr_to_string(htmlinputelement_get_name(instance)) }
}

pub fn set_name(instance: i32, value: &str) {
    unsafe {
        htmlinputelement_set_name(instance, cstr(value));
    }
}
extern "C" {
    fn htmlinputelement_get_pattern(instance: i32) -> CString;
    fn htmlinputelement_set_pattern(instance: i32, value: CString);
}

pub fn get_pattern(instance: i32) -> String {
    unsafe { cstr_to_string(htmlinputelement_get_pattern(instance)) }
}

pub fn set_pattern(instance: i32, value: &str) {
    unsafe {
        htmlinputelement_set_pattern(instance, cstr(value));
    }
}
extern "C" {
    fn htmlinputelement_get_placeholder(instance: i32) -> CString;
    fn htmlinputelement_set_placeholder(instance: i32, value: CString);
}

pub fn get_placeholder(instance: i32) -> String {
    unsafe { cstr_to_string(htmlinputelement_get_placeholder(instance)) }
}

pub fn set_placeholder(instance: i32, value: &str) {
    unsafe {
        htmlinputelement_set_placeholder(instance, cstr(value));
    }
}
extern "C" {
    fn htmlinputelement_get_readOnly(instance: i32) -> i32;
    fn htmlinputelement_set_readOnly(instance: i32, value: i32);
}

pub fn get_read_only(instance: i32) -> i32 {
    unsafe { htmlinputelement_get_readOnly(instance) }
}

pub fn set_read_only(instance: i32, value: i32) {
    unsafe {
        htmlinputelement_set_readOnly(instance, value);
    }
}
extern "C" {
    fn htmlinputelement_get_required(instance: i32) -> i32;
    fn htmlinputelement_set_required(instance: i32, value: i32);
}

pub fn get_required(instance: i32) -> i32 {
    unsafe { htmlinputelement_get_required(instance) }
}

pub fn set_required(instance: i32, value: i32) {
    unsafe {
        htmlinputelement_set_required(instance, value);
    }
}
extern "C" {
    fn htmlinputelement_get_size(instance: i32) -> i32;
    fn htmlinputelement_set_size(instance: i32, value: i32);
}

pub fn get_size(instance: i32) -> i32 {
    unsafe { htmlinputelement_get_size(instance) }
}

pub fn set_size(instance: i32, value: i32) {
    unsafe {
        htmlinputelement_set_size(instance, value);
    }
}
extern "C" {
    fn htmlinputelement_get_src(instance: i32) -> CString;
    fn htmlinputelement_set_src(instance: i32, value: CString);
}

pub fn get_src(instance: i32) -> String {
    unsafe { cstr_to_string(htmlinputelement_get_src(instance)) }
}

pub fn set_src(instance: i32, value: &str) {
    unsafe {
        htmlinputelement_set_src(instance, cstr(value));
    }
}
extern "C" {
    fn htmlinputelement_get_step(instance: i32) -> CString;
    fn htmlinputelement_set_step(instance: i32, value: CString);
}

pub fn get_step(instance: i32) -> String {
    unsafe { cstr_to_string(htmlinputelement_get_step(instance)) }
}

pub fn set_step(instance: i32, value: &str) {
    unsafe {
        htmlinputelement_set_step(instance, cstr(value));
    }
}
extern "C" {
    fn htmlinputelement_get_type(instance: i32) -> CString;
    fn htmlinputelement_set_type(instance: i32, value: CString);
}

pub fn get_type(instance: i32) -> String {
    unsafe { cstr_to_string(htmlinputelement_get_type(instance)) }
}

pub fn set_type(instance: i32, value: &str) {
    unsafe {
        htmlinputelement_set_type(instance, cstr(value));
    }
}
extern "C" {
    fn htmlinputelement_get_defaultValue(instance: i32) -> CString;
    fn htmlinputelement_set_defaultValue(instance: i32, value: CString);
}

pub fn get_default_value(instance: i32) -> String {
    unsafe { cstr_to_string(htmlinputelement_get_defaultValue(instance)) }
}

pub fn set_default_value(instance: i32, value: &str) {
    unsafe {
        htmlinputelement_set_defaultValue(instance, cstr(value));
    }
}
extern "C" {
    fn htmlinputelement_get_value(instance: i32) -> CString;
    fn htmlinputelement_set_value(instance: i32, value: CString);
}

pub fn get_value(instance: i32) -> String {
    unsafe { cstr_to_string(htmlinputelement_get_value(instance)) }
}

pub fn set_value(instance: i32, value: &str) {
    unsafe {
        htmlinputelement_set_value(instance, cstr(value));
    }
}
extern "C" {
    fn htmlinputelement_get_valueAsDate(instance: i32) -> i32;
    fn htmlinputelement_set_valueAsDate(instance: i32, value: i32);
}

pub fn get_value_as_date(instance: i32) -> i32 {
    unsafe { htmlinputelement_get_valueAsDate(instance) }
}

pub fn set_value_as_date(instance: i32, value: i32) {
    unsafe {
        htmlinputelement_set_valueAsDate(instance, value);
    }
}
extern "C" {
    fn htmlinputelement_get_valueAsNumber(instance: i32) -> i32;
    fn htmlinputelement_set_valueAsNumber(instance: i32, value: i32);
}

pub fn get_value_as_number(instance: i32) -> i32 {
    unsafe { htmlinputelement_get_valueAsNumber(instance) }
}

pub fn set_value_as_number(instance: i32, value: i32) {
    unsafe {
        htmlinputelement_set_valueAsNumber(instance, value);
    }
}
extern "C" {
    fn htmlinputelement_get_width(instance: i32) -> i32;
    fn htmlinputelement_set_width(instance: i32, value: i32);
}

pub fn get_width(instance: i32) -> i32 {
    unsafe { htmlinputelement_get_width(instance) }
}

pub fn set_width(instance: i32, value: i32) {
    unsafe {
        htmlinputelement_set_width(instance, value);
    }
}
extern "C" {
    fn htmlinputelement_get_willValidate(instance: i32) -> i32;
    fn htmlinputelement_set_willValidate(instance: i32, value: i32);
}

pub fn get_will_validate(instance: i32) -> i32 {
    unsafe { htmlinputelement_get_willValidate(instance) }
}

pub fn set_will_validate(instance: i32, value: i32) {
    unsafe {
        htmlinputelement_set_willValidate(instance, value);
    }
}
extern "C" {
    fn htmlinputelement_get_validity(instance: i32) -> i32;
    fn htmlinputelement_set_validity(instance: i32, value: i32);
}

pub fn get_validity(instance: i32) -> i32 {
    unsafe { htmlinputelement_get_validity(instance) }
}

pub fn set_validity(instance: i32, value: i32) {
    unsafe {
        htmlinputelement_set_validity(instance, value);
    }
}
extern "C" {
    fn htmlinputelement_get_validationMessage(instance: i32) -> CString;
    fn htmlinputelement_set_validationMessage(instance: i32, value: CString);
}

pub fn get_validation_message(instance: i32) -> String {
    unsafe { cstr_to_string(htmlinputelement_get_validationMessage(instance)) }
}

pub fn set_validation_message(instance: i32, value: &str) {
    unsafe {
        htmlinputelement_set_validationMessage(instance, cstr(value));
    }
}
extern "C" {
    fn htmlinputelement_check_validity(instance: i32) -> i32;
}

pub fn check_validity(instance: i32) -> i32 {
    unsafe { htmlinputelement_check_validity(instance) }
}
extern "C" {
    fn htmlinputelement_report_validity(instance: i32) -> i32;
}

pub fn report_validity(instance: i32) -> i32 {
    unsafe { htmlinputelement_report_validity(instance) }
}
extern "C" {
    fn htmlinputelement_set_custom_validity(instance: i32, error: CString);
}

pub fn set_custom_validity(instance: i32, error: &str) {
    unsafe { htmlinputelement_set_custom_validity(instance, cstr(error)) }
}
extern "C" {
    fn htmlinputelement_get_labels(instance: i32) -> i32;
    fn htmlinputelement_set_labels(instance: i32, value: i32);
}

pub fn get_labels(instance: i32) -> i32 {
    unsafe { htmlinputelement_get_labels(instance) }
}

pub fn set_labels(instance: i32, value: i32) {
    unsafe {
        htmlinputelement_set_labels(instance, value);
    }
}
extern "C" {
    fn htmlinputelement_select(instance: i32);
}

pub fn select(instance: i32) {
    unsafe { htmlinputelement_select(instance) }
}
extern "C" {
    fn htmlinputelement_get_selectionDirection(instance: i32) -> CString;
    fn htmlinputelement_set_selectionDirection(instance: i32, value: CString);
}

pub fn get_selection_direction(instance: i32) -> String {
    unsafe { cstr_to_string(htmlinputelement_get_selectionDirection(instance)) }
}

pub fn set_selection_direction(instance: i32, value: &str) {
    unsafe {
        htmlinputelement_set_selectionDirection(instance, cstr(value));
    }
}
extern "C" {
    fn htmlinputelement_set_range_text(
        instance: i32,
        replacement: CString,
        start: i32,
        end: i32,
        selection_mode: i32,
    );
}

pub fn set_range_text(instance: i32, replacement: &str, start: i32, end: i32, selection_mode: i32) {
    unsafe {
        htmlinputelement_set_range_text(instance, cstr(replacement), start, end, selection_mode)
    }
}
extern "C" {
    fn htmlinputelement_set_selection_range(
        instance: i32,
        start: i32,
        end: i32,
        direction: CString,
    );
}

pub fn set_selection_range(instance: i32, start: i32, end: i32, direction: &str) {
    unsafe { htmlinputelement_set_selection_range(instance, start, end, cstr(direction)) }
}
extern "C" {
    fn htmlinputelement_get_align(instance: i32) -> CString;
    fn htmlinputelement_set_align(instance: i32, value: CString);
}

pub fn get_align(instance: i32) -> String {
    unsafe { cstr_to_string(htmlinputelement_get_align(instance)) }
}

pub fn set_align(instance: i32, value: &str) {
    unsafe {
        htmlinputelement_set_align(instance, cstr(value));
    }
}
extern "C" {
    fn htmlinputelement_get_useMap(instance: i32) -> CString;
    fn htmlinputelement_set_useMap(instance: i32, value: CString);
}

pub fn get_use_map(instance: i32) -> String {
    unsafe { cstr_to_string(htmlinputelement_get_useMap(instance)) }
}

pub fn set_use_map(instance: i32, value: &str) {
    unsafe {
        htmlinputelement_set_useMap(instance, cstr(value));
    }
}
extern "C" {
    fn htmlinputelement_get_date_time_input_box_value(instance: i32) -> i32;
}

pub fn get_date_time_input_box_value(instance: i32) -> i32 {
    unsafe { htmlinputelement_get_date_time_input_box_value(instance) }
}
extern "C" {
    fn htmlinputelement_update_date_time_input_box(instance: i32, value: i32);
}

pub fn update_date_time_input_box(instance: i32, value: i32) {
    unsafe { htmlinputelement_update_date_time_input_box(instance, value) }
}
extern "C" {
    fn htmlinputelement_set_date_time_picker_state(instance: i32, open: i32);
}

pub fn set_date_time_picker_state(instance: i32, open: i32) {
    unsafe { htmlinputelement_set_date_time_picker_state(instance, open) }
}
extern "C" {
    fn htmlinputelement_get_minimum(instance: i32) -> i32;
}

pub fn get_minimum(instance: i32) -> i32 {
    unsafe { htmlinputelement_get_minimum(instance) }
}
extern "C" {
    fn htmlinputelement_get_maximum(instance: i32) -> i32;
}

pub fn get_maximum(instance: i32) -> i32 {
    unsafe { htmlinputelement_get_maximum(instance) }
}
extern "C" {
    fn htmlinputelement_get_previewValue(instance: i32) -> CString;
    fn htmlinputelement_set_previewValue(instance: i32, value: CString);
}

pub fn get_preview_value(instance: i32) -> String {
    unsafe { cstr_to_string(htmlinputelement_get_previewValue(instance)) }
}

pub fn set_preview_value(instance: i32, value: &str) {
    unsafe {
        htmlinputelement_set_previewValue(instance, cstr(value));
    }
}
