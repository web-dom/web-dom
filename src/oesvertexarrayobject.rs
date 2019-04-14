#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use alloc::string::String;
pub const VERTEX_ARRAY_BINDING_OES: f32 = 0x85B5 as f32;
extern "C" {
    fn oesvertexarrayobject_create_vertex_array_o_e_s(instance: DOMReference) -> DOMReference;
}

pub fn create_vertex_array_o_e_s(instance: DOMReference) -> DOMReference {
    unsafe { oesvertexarrayobject_create_vertex_array_o_e_s(instance) }
}
extern "C" {
    fn oesvertexarrayobject_delete_vertex_array_o_e_s(
        instance: DOMReference,
        delete_vertex_array_o_e_s: DOMReference,
    );
}

pub fn delete_vertex_array_o_e_s(instance: DOMReference, array_object: DOMReference) {
    unsafe { oesvertexarrayobject_delete_vertex_array_o_e_s(instance, array_object) }
}
extern "C" {
    fn oesvertexarrayobject_is_vertex_array_o_e_s(
        instance: DOMReference,
        is_vertex_array_o_e_s: DOMReference,
    ) -> i32;
}

pub fn is_vertex_array_o_e_s(instance: DOMReference, array_object: DOMReference) -> bool {
    unsafe { 0 != oesvertexarrayobject_is_vertex_array_o_e_s(instance, array_object) }
}
extern "C" {
    fn oesvertexarrayobject_bind_vertex_array_o_e_s(
        instance: DOMReference,
        bind_vertex_array_o_e_s: DOMReference,
    );
}

pub fn bind_vertex_array_o_e_s(instance: DOMReference, array_object: DOMReference) {
    unsafe { oesvertexarrayobject_bind_vertex_array_o_e_s(instance, array_object) }
}
