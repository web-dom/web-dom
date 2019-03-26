#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn dynamic_load(value: CString, listener: DOMReference);
    fn dynamic_unload(dynamic_handle: DOMReference);
    fn dynamic_begin(dynamic_handle: DOMReference) -> DOMReference;
    fn dynamic_param_cstring(call_handle: DOMReference, cstr: CString);
    fn dynamic_param_memory(call_handle: DOMReference, start: i32, length: i32);
    fn dynamic_param_i32(call_handle: DOMReference, param: i32);
    fn dynamic_param_i64(call_handle: DOMReference, param: i64);
    fn dynamic_param_f32(call_handle: DOMReference, param: f32);
    fn dynamic_param_f64(call_handle: DOMReference, param: f64);
    fn dynamic_call(call_handle: DOMReference, methodName: CString);
    fn dynamic_result_i32(call_handle: DOMReference) -> i32;
    fn dynamic_result_i64(call_handle: DOMReference) -> i64;
    fn dynamic_result_f32(call_handle: DOMReference) -> f32;
    fn dynamic_result_f64(call_handle: DOMReference) -> f64;
    fn dynamic_result_cstring(call_handle: DOMReference) -> CString;
    fn dynamic_result_memory(call_handle: DOMReference) -> i32;
    fn dynamic_result_memory_len(call_handle: DOMReference) -> i32;
}

pub fn load(value: &str, listener: DOMReference) {
    unsafe { dynamic_load(to_cstring(value), listener) }
}
pub fn unload(dynamic_handle: DOMReference) {
    unsafe { dynamic_unload(dynamic_handle) }
}
pub fn begin(dynamic_handle: DOMReference) -> DOMReference {
    unsafe { dynamic_begin(dynamic_handle) }
}
pub fn param_cstring(call_handle: DOMReference, s: &str) {
    unsafe { dynamic_param_cstring(call_handle, to_cstring(s)) }
}
pub fn param_memory(call_handle: DOMReference, start: i32, len: i32) {
    unsafe { dynamic_param_memory(call_handle, start, len) }
}
pub fn param_i32(call_handle: DOMReference, param: i32) {
    unsafe { dynamic_param_i32(call_handle, param) }
}
pub fn param_i64(call_handle: DOMReference, param: i64) {
    unsafe { dynamic_param_i64(call_handle, param) }
}
pub fn param_f32(call_handle: DOMReference, param: f32) {
    unsafe { dynamic_param_f32(call_handle, param) }
}
pub fn param_f64(call_handle: DOMReference, param: f64) {
    unsafe { dynamic_param_f64(call_handle, param) }
}
pub fn call(call_handle: DOMReference, method_name: &str) {
    unsafe { dynamic_call(call_handle, to_cstring(method_name)) }
}
pub fn result_i32(call_handle: DOMReference) -> i32 {
    unsafe { dynamic_result_i32(call_handle) }
}
pub fn result_i64(call_handle: DOMReference) -> i64 {
    unsafe { dynamic_result_i64(call_handle) }
}
pub fn result_f32(call_handle: DOMReference) -> f32 {
    unsafe { dynamic_result_f32(call_handle) }
}
pub fn result_f64(call_handle: DOMReference) -> f64 {
    unsafe { dynamic_result_f64(call_handle) }
}
pub fn result_cstring(call_handle: DOMReference) -> String {
    unsafe { to_string(dynamic_result_cstring(call_handle)) }
}
pub fn result_memory(call_handle: DOMReference) -> i32 {
    unsafe { dynamic_result_memory(call_handle) }
}
pub fn result_memory_len(call_handle: DOMReference) -> i32 {
    unsafe { dynamic_result_memory_len(call_handle) }
}
