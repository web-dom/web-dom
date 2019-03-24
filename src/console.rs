#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn console_assert(assert: i32, assert: CString);
}

pub fn assert(condition: bool, message: &str) {
    unsafe { console_assert(if condition { 1 } else { 0 }, to_cstring(message)) }
}
extern "C" {
    fn console_clear();
}

pub fn clear() {
    unsafe { console_clear() }
}
extern "C" {
    fn console_count(count: CString);
}

pub fn count(label: &str) {
    unsafe { console_count(to_cstring(label)) }
}
extern "C" {
    fn console_count_reset(count_reset: CString);
}

pub fn count_reset(label: &str) {
    unsafe { console_count_reset(to_cstring(label)) }
}
extern "C" {
    fn console_debug(debug: CString);
}

pub fn debug(message: &str) {
    unsafe { console_debug(to_cstring(message)) }
}
extern "C" {
    fn console_error(error: CString);
}

pub fn error(message: &str) {
    unsafe { console_error(to_cstring(message)) }
}
extern "C" {
    fn console_info(info: CString);
}

pub fn info(message: &str) {
    unsafe { console_info(to_cstring(message)) }
}
extern "C" {
    fn console_log(log: CString);
}

pub fn log(message: &str) {
    unsafe { console_log(to_cstring(message)) }
}
extern "C" {
    fn console_table(table: CString);
}

pub fn table(message: &str) {
    unsafe { console_table(to_cstring(message)) }
}
extern "C" {
    fn console_trace(trace: CString);
}

pub fn trace(message: &str) {
    unsafe { console_trace(to_cstring(message)) }
}
extern "C" {
    fn console_warn(warn: CString);
}

pub fn warn(message: &str) {
    unsafe { console_warn(to_cstring(message)) }
}
extern "C" {
    fn console_dir(dir: CString);
}

pub fn dir(message: &str) {
    unsafe { console_dir(to_cstring(message)) }
}
extern "C" {
    fn console_dirxml(dirxml: CString);
}

pub fn dirxml(message: &str) {
    unsafe { console_dirxml(to_cstring(message)) }
}
extern "C" {
    fn console_group(group: CString);
}

pub fn group(message: &str) {
    unsafe { console_group(to_cstring(message)) }
}
extern "C" {
    fn console_group_collapsed(group_collapsed: CString);
}

pub fn group_collapsed(message: &str) {
    unsafe { console_group_collapsed(to_cstring(message)) }
}
extern "C" {
    fn console_group_end();
}

pub fn group_end() {
    unsafe { console_group_end() }
}
extern "C" {
    fn console_time(time: CString);
}

pub fn time(label: &str) {
    unsafe { console_time(to_cstring(label)) }
}
extern "C" {
    fn console_time_log(time_log: CString, time_log: CString);
}

pub fn time_log(label: &str, message: &str) {
    unsafe { console_time_log(to_cstring(label), to_cstring(message)) }
}
extern "C" {
    fn console_time_end(time_end: CString);
}

pub fn time_end(label: &str) {
    unsafe { console_time_end(to_cstring(label)) }
}
extern "C" {
    fn console_exception(exception: CString);
}

pub fn exception(message: &str) {
    unsafe { console_exception(to_cstring(message)) }
}
extern "C" {
    fn console_time_stamp(time_stamp: CString);
}

pub fn time_stamp(message: &str) {
    unsafe { console_time_stamp(to_cstring(message)) }
}
extern "C" {
    fn console_profile(profile: CString);
}

pub fn profile(message: &str) {
    unsafe { console_profile(to_cstring(message)) }
}
extern "C" {
    fn console_profile_end(profile_end: CString);
}

pub fn profile_end(message: &str) {
    unsafe { console_profile_end(to_cstring(message)) }
}
