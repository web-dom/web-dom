#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn consoleinstance_assert(instance: i32, condition: i32, message: CString);
}

pub fn assert(instance: i32, condition: i32, message: &str) {
    unsafe { consoleinstance_assert(instance, condition, cstr(message)) }
}
extern "C" {
    fn consoleinstance_clear(instance: i32);
}

pub fn clear(instance: i32) {
    unsafe { consoleinstance_clear(instance) }
}
extern "C" {
    fn consoleinstance_count(instance: i32, label: CString);
}

pub fn count(instance: i32, label: &str) {
    unsafe { consoleinstance_count(instance, cstr(label)) }
}
extern "C" {
    fn consoleinstance_count_reset(instance: i32, label: CString);
}

pub fn count_reset(instance: i32, label: &str) {
    unsafe { consoleinstance_count_reset(instance, cstr(label)) }
}
extern "C" {
    fn consoleinstance_debug(instance: i32, message: CString);
}

pub fn debug(instance: i32, message: &str) {
    unsafe { consoleinstance_debug(instance, cstr(message)) }
}
extern "C" {
    fn consoleinstance_error(instance: i32, message: CString);
}

pub fn error(instance: i32, message: &str) {
    unsafe { consoleinstance_error(instance, cstr(message)) }
}
extern "C" {
    fn consoleinstance_info(instance: i32, message: CString);
}

pub fn info(instance: i32, message: &str) {
    unsafe { consoleinstance_info(instance, cstr(message)) }
}
extern "C" {
    fn consoleinstance_log(instance: i32, message: CString);
}

pub fn log(instance: i32, message: &str) {
    unsafe { consoleinstance_log(instance, cstr(message)) }
}
extern "C" {
    fn consoleinstance_table(instance: i32, message: CString);
}

pub fn table(instance: i32, message: &str) {
    unsafe { consoleinstance_table(instance, cstr(message)) }
}
extern "C" {
    fn consoleinstance_trace(instance: i32, message: CString);
}

pub fn trace(instance: i32, message: &str) {
    unsafe { consoleinstance_trace(instance, cstr(message)) }
}
extern "C" {
    fn consoleinstance_warn(instance: i32, message: CString);
}

pub fn warn(instance: i32, message: &str) {
    unsafe { consoleinstance_warn(instance, cstr(message)) }
}
extern "C" {
    fn consoleinstance_dir(instance: i32, message: CString);
}

pub fn dir(instance: i32, message: &str) {
    unsafe { consoleinstance_dir(instance, cstr(message)) }
}
extern "C" {
    fn consoleinstance_dirxml(instance: i32, message: CString);
}

pub fn dirxml(instance: i32, message: &str) {
    unsafe { consoleinstance_dirxml(instance, cstr(message)) }
}
extern "C" {
    fn consoleinstance_group(instance: i32, message: CString);
}

pub fn group(instance: i32, message: &str) {
    unsafe { consoleinstance_group(instance, cstr(message)) }
}
extern "C" {
    fn consoleinstance_group_collapsed(instance: i32, message: CString);
}

pub fn group_collapsed(instance: i32, message: &str) {
    unsafe { consoleinstance_group_collapsed(instance, cstr(message)) }
}
extern "C" {
    fn consoleinstance_group_end(instance: i32);
}

pub fn group_end(instance: i32) {
    unsafe { consoleinstance_group_end(instance) }
}
extern "C" {
    fn consoleinstance_time(instance: i32, label: CString);
}

pub fn time(instance: i32, label: &str) {
    unsafe { consoleinstance_time(instance, cstr(label)) }
}
extern "C" {
    fn consoleinstance_time_log(instance: i32, label: CString, message: CString);
}

pub fn time_log(instance: i32, label: &str, message: &str) {
    unsafe { consoleinstance_time_log(instance, cstr(label), cstr(message)) }
}
extern "C" {
    fn consoleinstance_time_end(instance: i32, label: CString);
}

pub fn time_end(instance: i32, label: &str) {
    unsafe { consoleinstance_time_end(instance, cstr(label)) }
}
extern "C" {
    fn consoleinstance_exception(instance: i32, message: CString);
}

pub fn exception(instance: i32, message: &str) {
    unsafe { consoleinstance_exception(instance, cstr(message)) }
}
extern "C" {
    fn consoleinstance_time_stamp(instance: i32, data: i32);
}

pub fn time_stamp(instance: i32, data: i32) {
    unsafe { consoleinstance_time_stamp(instance, data) }
}
extern "C" {
    fn consoleinstance_profile(instance: i32, message: CString);
}

pub fn profile(instance: i32, message: &str) {
    unsafe { consoleinstance_profile(instance, cstr(message)) }
}
extern "C" {
    fn consoleinstance_profile_end(instance: i32, message: CString);
}

pub fn profile_end(instance: i32, message: &str) {
    unsafe { consoleinstance_profile_end(instance, cstr(message)) }
}
extern "C" {
    fn consoleinstance_report_for_service_worker_scope(
        instance: i32,
        scope: CString,
        message: CString,
        filename: CString,
        line_number: i32,
        column_number: i32,
        level: i32,
    );
}

pub fn report_for_service_worker_scope(
    instance: i32,
    scope: &str,
    message: &str,
    filename: &str,
    line_number: i32,
    column_number: i32,
    level: i32,
) {
    unsafe {
        consoleinstance_report_for_service_worker_scope(
            instance,
            cstr(scope),
            cstr(message),
            cstr(filename),
            line_number,
            column_number,
            level,
        )
    }
}
