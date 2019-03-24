#[allow(unused_imports)]
use crate::*;
pub const QUERY_COUNTER_BITS_EXT: i32 = 0x8864;
pub const CURRENT_QUERY_EXT: i32 = 0x8865;
pub const QUERY_RESULT_EXT: i32 = 0x8866;
pub const QUERY_RESULT_AVAILABLE_EXT: i32 = 0x8867;
pub const TIME_ELAPSED_EXT: i32 = 0x88BF;
pub const TIMESTAMP_EXT: i32 = 0x8E28;
pub const GPU_DISJOINT_EXT: i32 = 0x8FBB;
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
        begin_query_e_x_t: DOMReference,
        begin_query_e_x_t: DOMReference,
    );
}

pub fn begin_query_e_x_t(instance: DOMReference, target: DOMReference, query: DOMReference) {
    unsafe { extdisjointtimerquery_begin_query_e_x_t(instance, target, query) }
}
extern "C" {
    fn extdisjointtimerquery_end_query_e_x_t(instance: DOMReference, end_query_e_x_t: DOMReference);
}

pub fn end_query_e_x_t(instance: DOMReference, target: DOMReference) {
    unsafe { extdisjointtimerquery_end_query_e_x_t(instance, target) }
}
extern "C" {
    fn extdisjointtimerquery_query_counter_e_x_t(
        instance: DOMReference,
        query_counter_e_x_t: DOMReference,
        query_counter_e_x_t: DOMReference,
    );
}

pub fn query_counter_e_x_t(instance: DOMReference, query: DOMReference, target: DOMReference) {
    unsafe { extdisjointtimerquery_query_counter_e_x_t(instance, query, target) }
}
extern "C" {
    fn extdisjointtimerquery_get_query_e_x_t(
        instance: DOMReference,
        get_query_e_x_t: DOMReference,
        get_query_e_x_t: DOMReference,
    ) -> DOMReference;
}

pub fn get_query_e_x_t(
    instance: DOMReference,
    target: DOMReference,
    pname: DOMReference,
) -> DOMReference {
    unsafe { extdisjointtimerquery_get_query_e_x_t(instance, target, pname) }
}
extern "C" {
    fn extdisjointtimerquery_get_query_object_e_x_t(
        instance: DOMReference,
        get_query_object_e_x_t: DOMReference,
        get_query_object_e_x_t: DOMReference,
    ) -> DOMReference;
}

pub fn get_query_object_e_x_t(
    instance: DOMReference,
    query: DOMReference,
    pname: DOMReference,
) -> DOMReference {
    unsafe { extdisjointtimerquery_get_query_object_e_x_t(instance, query, pname) }
}
