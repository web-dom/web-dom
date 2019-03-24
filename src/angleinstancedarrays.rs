#[allow(unused_imports)]
use crate::*;
pub const VERTEX_ATTRIB_ARRAY_DIVISOR_ANGLE: i32 = 0x88FE;
extern "C" {
    fn angleinstancedarrays_draw_arrays_instanced_angle(
        instance: DOMReference,
        draw_arrays_instanced_angle: DOMReference,
        draw_arrays_instanced_angle: DOMReference,
        draw_arrays_instanced_angle: DOMReference,
        draw_arrays_instanced_angle: DOMReference,
    );
}

pub fn draw_arrays_instanced_angle(
    instance: DOMReference,
    mode: DOMReference,
    first: DOMReference,
    count: DOMReference,
    primcount: DOMReference,
) {
    unsafe {
        angleinstancedarrays_draw_arrays_instanced_angle(instance, mode, first, count, primcount)
    }
}
extern "C" {
    fn angleinstancedarrays_draw_elements_instanced_angle(
        instance: DOMReference,
        draw_elements_instanced_angle: DOMReference,
        draw_elements_instanced_angle: DOMReference,
        draw_elements_instanced_angle: DOMReference,
        draw_elements_instanced_angle: DOMReference,
        draw_elements_instanced_angle: DOMReference,
    );
}

pub fn draw_elements_instanced_angle(
    instance: DOMReference,
    mode: DOMReference,
    count: DOMReference,
    element_type: DOMReference,
    offset: DOMReference,
    primcount: DOMReference,
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
        vertex_attrib_divisor_angle: DOMReference,
        vertex_attrib_divisor_angle: DOMReference,
    );
}

pub fn vertex_attrib_divisor_angle(
    instance: DOMReference,
    index: DOMReference,
    divisor: DOMReference,
) {
    unsafe { angleinstancedarrays_vertex_attrib_divisor_angle(instance, index, divisor) }
}
