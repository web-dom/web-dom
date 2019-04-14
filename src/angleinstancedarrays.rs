#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use alloc::string::String;
pub const VERTEX_ATTRIB_ARRAY_DIVISOR_ANGLE: f32 = 0x88FE as f32;
extern "C" {
    fn angleinstancedarrays_draw_arrays_instanced_angle(
        instance: DOMReference,
        draw_arrays_instanced_angle: f32,
        draw_arrays_instanced_angle: f32,
        draw_arrays_instanced_angle: f32,
        draw_arrays_instanced_angle: f32,
    );
}

pub fn draw_arrays_instanced_angle(
    instance: DOMReference,
    mode: f32,
    first: f32,
    count: f32,
    primcount: f32,
) {
    unsafe {
        angleinstancedarrays_draw_arrays_instanced_angle(instance, mode, first, count, primcount)
    }
}
extern "C" {
    fn angleinstancedarrays_draw_elements_instanced_angle(
        instance: DOMReference,
        draw_elements_instanced_angle: f32,
        draw_elements_instanced_angle: f32,
        draw_elements_instanced_angle: f32,
        draw_elements_instanced_angle: f32,
        draw_elements_instanced_angle: f32,
    );
}

pub fn draw_elements_instanced_angle(
    instance: DOMReference,
    mode: f32,
    count: f32,
    element_type: f32,
    offset: f32,
    primcount: f32,
) {
    unsafe {
        angleinstancedarrays_draw_elements_instanced_angle(
            instance,
            mode,
            count,
            element_type,
            offset,
            primcount,
        )
    }
}
extern "C" {
    fn angleinstancedarrays_vertex_attrib_divisor_angle(
        instance: DOMReference,
        vertex_attrib_divisor_angle: f32,
        vertex_attrib_divisor_angle: f32,
    );
}

pub fn vertex_attrib_divisor_angle(instance: DOMReference, index: f32, divisor: f32) {
    unsafe { angleinstancedarrays_vertex_attrib_divisor_angle(instance, index, divisor) }
}
