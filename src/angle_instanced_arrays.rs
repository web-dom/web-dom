#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn angle_instanced_arrays_draw_arrays_instanced_angle(
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
        angle_instanced_arrays_draw_arrays_instanced_angle(instance, mode, first, count, primcount)
    }
}
extern "C" {
    fn angle_instanced_arrays_draw_elements_instanced_angle(
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
        angle_instanced_arrays_draw_elements_instanced_angle(
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
    fn angle_instanced_arrays_vertex_attrib_divisor_angle(
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
    unsafe { angle_instanced_arrays_vertex_attrib_divisor_angle(instance, index, divisor) }
}
