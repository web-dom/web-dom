#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn ext_disjoint_timer_query_create_query_e_x_t(instance: DOMReference) -> DOMReference;
}

pub fn create_query_e_x_t(instance: DOMReference) -> DOMReference {
    unsafe { ext_disjoint_timer_query_create_query_e_x_t(instance) }
}
extern "C" {
    fn ext_disjoint_timer_query_delete_query_e_x_t(
        instance: DOMReference,
        delete_query_e_x_t: DOMReference,
    );
}

pub fn delete_query_e_x_t(instance: DOMReference, query: DOMReference) {
    unsafe { ext_disjoint_timer_query_delete_query_e_x_t(instance, query) }
}
extern "C" {
    fn ext_disjoint_timer_query_is_query_e_x_t(
        instance: DOMReference,
        is_query_e_x_t: DOMReference,
    ) -> i32;
}

pub fn is_query_e_x_t(instance: DOMReference, query: DOMReference) -> bool {
    unsafe { 0 != ext_disjoint_timer_query_is_query_e_x_t(instance, query) }
}
extern "C" {
    fn ext_disjoint_timer_query_begin_query_e_x_t(
        instance: DOMReference,
        begin_query_e_x_t: DOMReference,
        begin_query_e_x_t: DOMReference,
    );
}

pub fn begin_query_e_x_t(instance: DOMReference, target: DOMReference, query: DOMReference) {
    unsafe { ext_disjoint_timer_query_begin_query_e_x_t(instance, target, query) }
}
extern "C" {
    fn ext_disjoint_timer_query_end_query_e_x_t(
        instance: DOMReference,
        end_query_e_x_t: DOMReference,
    );
}

pub fn end_query_e_x_t(instance: DOMReference, target: DOMReference) {
    unsafe { ext_disjoint_timer_query_end_query_e_x_t(instance, target) }
}
extern "C" {
    fn ext_disjoint_timer_query_query_counter_e_x_t(
        instance: DOMReference,
        query_counter_e_x_t: DOMReference,
        query_counter_e_x_t: DOMReference,
    );
}

pub fn query_counter_e_x_t(instance: DOMReference, query: DOMReference, target: DOMReference) {
    unsafe { ext_disjoint_timer_query_query_counter_e_x_t(instance, query, target) }
}
extern "C" {
    fn ext_disjoint_timer_query_get_query_e_x_t(
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
    unsafe { ext_disjoint_timer_query_get_query_e_x_t(instance, target, pname) }
}
extern "C" {
    fn ext_disjoint_timer_query_get_query_object_e_x_t(
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
    unsafe { ext_disjoint_timer_query_get_query_object_e_x_t(instance, query, pname) }
}
