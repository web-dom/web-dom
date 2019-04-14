#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use alloc::string::String;
pub const QUERY_COUNTER_BITS_EXT: f32 = 0x8864 as f32;
pub const CURRENT_QUERY_EXT: f32 = 0x8865 as f32;
pub const QUERY_RESULT_EXT: f32 = 0x8866 as f32;
pub const QUERY_RESULT_AVAILABLE_EXT: f32 = 0x8867 as f32;
pub const TIME_ELAPSED_EXT: f32 = 0x88BF as f32;
pub const TIMESTAMP_EXT: f32 = 0x8E28 as f32;
pub const GPU_DISJOINT_EXT: f32 = 0x8FBB as f32;
extern "C" {
    fn extdisjointtimerquery_create_query_e_x_t(instance: DOMReference) -> DOMReference;
}

pub fn create_query_e_x_t(instance: DOMReference) -> DOMReference {
    unsafe { extdisjointtimerquery_create_query_e_x_t(instance) }
}
extern "C" {
    fn extdisjointtimerquery_delete_query_e_x_t(
        instance: DOMReference,
        delete_query_e_x_t: DOMReference,
    );
}

pub fn delete_query_e_x_t(instance: DOMReference, query: DOMReference) {
    unsafe { extdisjointtimerquery_delete_query_e_x_t(instance, query) }
}
extern "C" {
    fn extdisjointtimerquery_is_query_e_x_t(
        instance: DOMReference,
        is_query_e_x_t: DOMReference,
    ) -> i32;
}

pub fn is_query_e_x_t(instance: DOMReference, query: DOMReference) -> bool {
    unsafe { 0 != extdisjointtimerquery_is_query_e_x_t(instance, query) }
}
extern "C" {
    fn extdisjointtimerquery_begin_query_e_x_t(
        instance: DOMReference,
        begin_query_e_x_t: f32,
        begin_query_e_x_t: DOMReference,
    );
}

pub fn begin_query_e_x_t(instance: DOMReference, target: f32, query: DOMReference) {
    unsafe { extdisjointtimerquery_begin_query_e_x_t(instance, target, query) }
}
extern "C" {
    fn extdisjointtimerquery_end_query_e_x_t(instance: DOMReference, end_query_e_x_t: f32);
}

pub fn end_query_e_x_t(instance: DOMReference, target: f32) {
    unsafe { extdisjointtimerquery_end_query_e_x_t(instance, target) }
}
extern "C" {
    fn extdisjointtimerquery_query_counter_e_x_t(
        instance: DOMReference,
        query_counter_e_x_t: DOMReference,
        query_counter_e_x_t: f32,
    );
}

pub fn query_counter_e_x_t(instance: DOMReference, query: DOMReference, target: f32) {
    unsafe { extdisjointtimerquery_query_counter_e_x_t(instance, query, target) }
}
extern "C" {
    fn extdisjointtimerquery_get_query_e_x_t(
        instance: DOMReference,
        get_query_e_x_t: f32,
        get_query_e_x_t: f32,
    ) -> DOMReference;
}

pub fn get_query_e_x_t(instance: DOMReference, target: f32, pname: f32) -> DOMReference {
    unsafe { extdisjointtimerquery_get_query_e_x_t(instance, target, pname) }
}
extern "C" {
    fn extdisjointtimerquery_get_query_object_e_x_t(
        instance: DOMReference,
        get_query_object_e_x_t: DOMReference,
        get_query_object_e_x_t: f32,
    ) -> DOMReference;
}

pub fn get_query_object_e_x_t(
    instance: DOMReference,
    query: DOMReference,
    pname: f32,
) -> DOMReference {
    unsafe { extdisjointtimerquery_get_query_object_e_x_t(instance, query, pname) }
}
