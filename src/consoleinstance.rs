#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn consoleinstance_assert(instance: DOMReference, assert: i32, assert: CString);
}

pub fn assert(instance: DOMReference, condition: bool, message: &str) {
    unsafe { consoleinstance_assert(instance, if condition { 1 } else { 0 }, to_cstring(message)) }
}
extern "C" {
    fn consoleinstance_clear(instance: DOMReference);
}

pub fn clear(instance: DOMReference) {
    unsafe { consoleinstance_clear(instance) }
}
extern "C" {
    fn consoleinstance_count(instance: DOMReference, count: CString);
}

pub fn count(instance: DOMReference, label: &str) {
    unsafe { consoleinstance_count(instance, to_cstring(label)) }
}
extern "C" {
    fn consoleinstance_count_reset(instance: DOMReference, count_reset: CString);
}

pub fn count_reset(instance: DOMReference, label: &str) {
    unsafe { consoleinstance_count_reset(instance, to_cstring(label)) }
}
extern "C" {
    fn consoleinstance_debug(instance: DOMReference, debug: CString);
}

pub fn debug(instance: DOMReference, message: &str) {
    unsafe { consoleinstance_debug(instance, to_cstring(message)) }
}
extern "C" {
    fn consoleinstance_error(instance: DOMReference, error: CString);
}

pub fn error(instance: DOMReference, message: &str) {
    unsafe { consoleinstance_error(instance, to_cstring(message)) }
}
extern "C" {
    fn consoleinstance_info(instance: DOMReference, info: CString);
}

pub fn info(instance: DOMReference, message: &str) {
    unsafe { consoleinstance_info(instance, to_cstring(message)) }
}
extern "C" {
    fn consoleinstance_log(instance: DOMReference, log: CString);
}

pub fn log(instance: DOMReference, message: &str) {
    unsafe { consoleinstance_log(instance, to_cstring(message)) }
}
extern "C" {
    fn consoleinstance_table(instance: DOMReference, table: CString);
}

pub fn table(instance: DOMReference, message: &str) {
    unsafe { consoleinstance_table(instance, to_cstring(message)) }
}
extern "C" {
    fn consoleinstance_trace(instance: DOMReference, trace: CString);
}

pub fn trace(instance: DOMReference, message: &str) {
    unsafe { consoleinstance_trace(instance, to_cstring(message)) }
}
extern "C" {
    fn consoleinstance_warn(instance: DOMReference, warn: CString);
}

pub fn warn(instance: DOMReference, message: &str) {
    unsafe { consoleinstance_warn(instance, to_cstring(message)) }
}
extern "C" {
    fn consoleinstance_dir(instance: DOMReference, dir: CString);
}

pub fn dir(instance: DOMReference, message: &str) {
    unsafe { consoleinstance_dir(instance, to_cstring(message)) }
}
extern "C" {
    fn consoleinstance_dirxml(instance: DOMReference, dirxml: CString);
}

pub fn dirxml(instance: DOMReference, message: &str) {
    unsafe { consoleinstance_dirxml(instance, to_cstring(message)) }
}
extern "C" {
    fn consoleinstance_group(instance: DOMReference, group: CString);
}

pub fn group(instance: DOMReference, message: &str) {
    unsafe { consoleinstance_group(instance, to_cstring(message)) }
}
extern "C" {
    fn consoleinstance_group_collapsed(instance: DOMReference, group_collapsed: CString);
}

pub fn group_collapsed(instance: DOMReference, message: &str) {
    unsafe { consoleinstance_group_collapsed(instance, to_cstring(message)) }
}
extern "C" {
    fn consoleinstance_group_end(instance: DOMReference);
}

pub fn group_end(instance: DOMReference) {
    unsafe { consoleinstance_group_end(instance) }
}
extern "C" {
    fn consoleinstance_time(instance: DOMReference, time: CString);
}

pub fn time(instance: DOMReference, label: &str) {
    unsafe { consoleinstance_time(instance, to_cstring(label)) }
}
extern "C" {
    fn consoleinstance_time_log(instance: DOMReference, time_log: CString, time_log: CString);
}

pub fn time_log(instance: DOMReference, label: &str, message: &str) {
    unsafe { consoleinstance_time_log(instance, to_cstring(label), to_cstring(message)) }
}
extern "C" {
    fn consoleinstance_time_end(instance: DOMReference, time_end: CString);
}

pub fn time_end(instance: DOMReference, label: &str) {
    unsafe { consoleinstance_time_end(instance, to_cstring(label)) }
}
extern "C" {
    fn consoleinstance_exception(instance: DOMReference, exception: CString);
}

pub fn exception(instance: DOMReference, message: &str) {
    unsafe { consoleinstance_exception(instance, to_cstring(message)) }
}
extern "C" {
    fn consoleinstance_time_stamp(instance: DOMReference, time_stamp: DOMReference);
}

pub fn time_stamp(instance: DOMReference, data: DOMReference) {
    unsafe { consoleinstance_time_stamp(instance, data) }
}
extern "C" {
    fn consoleinstance_profile(instance: DOMReference, profile: CString);
}

pub fn profile(instance: DOMReference, message: &str) {
    unsafe { consoleinstance_profile(instance, to_cstring(message)) }
}
extern "C" {
    fn consoleinstance_profile_end(instance: DOMReference, profile_end: CString);
}

pub fn profile_end(instance: DOMReference, message: &str) {
    unsafe { consoleinstance_profile_end(instance, to_cstring(message)) }
}
extern "C" {
    fn consoleinstance_report_for_service_worker_scope(
        instance: DOMReference,
        report_for_service_worker_scope: CString,
        report_for_service_worker_scope: CString,
        report_for_service_worker_scope: CString,
        report_for_service_worker_scope: f32,
        report_for_service_worker_scope: f32,
        report_for_service_worker_scope: DOMReference,
    );
}

pub fn report_for_service_worker_scope(
    instance: DOMReference,
    scope: &str,
    message: &str,
    filename: &str,
    line_number: f32,
    column_number: f32,
    level: DOMReference,
) {
    unsafe {
        consoleinstance_report_for_service_worker_scope(
            instance,
            to_cstring(scope),
            to_cstring(message),
            to_cstring(filename),
            line_number,
            column_number,
            level,
        )
    }
}
