#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use alloc::string::String;
extern "C" {
    fn path2d_add_path(instance: DOMReference, add_path: DOMReference);
}

pub fn add_path(instance: DOMReference, path: DOMReference) {
    unsafe { path2d_add_path(instance, path) }
}
extern "C" {
    fn path2d_add_path_1(
        instance: DOMReference,
        add_path_1: DOMReference,
        add_path_1: DOMReference,
    );
}

pub fn add_path_1(instance: DOMReference, path: DOMReference, transformation: DOMReference) {
    unsafe { path2d_add_path_1(instance, path, transformation) }
}
extern "C" {
    fn path2d_close_path(instance: DOMReference);
}

pub fn close_path(instance: DOMReference) {
    unsafe { path2d_close_path(instance) }
}
extern "C" {
    fn path2d_move_to(instance: DOMReference, move_to: f32, move_to: f32);
}

pub fn move_to(instance: DOMReference, x: f32, y: f32) {
    unsafe { path2d_move_to(instance, x, y) }
}
extern "C" {
    fn path2d_line_to(instance: DOMReference, line_to: f32, line_to: f32);
}

pub fn line_to(instance: DOMReference, x: f32, y: f32) {
    unsafe { path2d_line_to(instance, x, y) }
}
extern "C" {
    fn path2d_quadratic_curve_to(
        instance: DOMReference,
        quadratic_curve_to: f32,
        quadratic_curve_to: f32,
        quadratic_curve_to: f32,
        quadratic_curve_to: f32,
    );
}

pub fn quadratic_curve_to(instance: DOMReference, cpx: f32, cpy: f32, x: f32, y: f32) {
    unsafe { path2d_quadratic_curve_to(instance, cpx, cpy, x, y) }
}
extern "C" {
    fn path2d_bezier_curve_to(
        instance: DOMReference,
        bezier_curve_to: f32,
        bezier_curve_to: f32,
        bezier_curve_to: f32,
        bezier_curve_to: f32,
        bezier_curve_to: f32,
        bezier_curve_to: f32,
    );
}

pub fn bezier_curve_to(
    instance: DOMReference,
    cp1x: f32,
    cp1y: f32,
    cp2x: f32,
    cp2y: f32,
    x: f32,
    y: f32,
) {
    unsafe { path2d_bezier_curve_to(instance, cp1x, cp1y, cp2x, cp2y, x, y) }
}
extern "C" {
    fn path2d_arc_to(
        instance: DOMReference,
        arc_to: f32,
        arc_to: f32,
        arc_to: f32,
        arc_to: f32,
        arc_to: f32,
    );
}

pub fn arc_to(instance: DOMReference, x1: f32, y1: f32, x2: f32, y2: f32, radius: f32) {
    unsafe { path2d_arc_to(instance, x1, y1, x2, y2, radius) }
}
extern "C" {
    fn path2d_rect(instance: DOMReference, rect: f32, rect: f32, rect: f32, rect: f32);
}

pub fn rect(instance: DOMReference, x: f32, y: f32, w: f32, h: f32) {
    unsafe { path2d_rect(instance, x, y, w, h) }
}
extern "C" {
    fn path2d_arc(instance: DOMReference, arc: f32, arc: f32, arc: f32, arc: f32, arc: f32);
}

pub fn arc(instance: DOMReference, x: f32, y: f32, radius: f32, start_angle: f32, end_angle: f32) {
    unsafe { path2d_arc(instance, x, y, radius, start_angle, end_angle) }
}
extern "C" {
    fn path2d_arc_1(
        instance: DOMReference,
        arc_1: f32,
        arc_1: f32,
        arc_1: f32,
        arc_1: f32,
        arc_1: f32,
        arc_1: i32,
    );
}

pub fn arc_1(
    instance: DOMReference,
    x: f32,
    y: f32,
    radius: f32,
    start_angle: f32,
    end_angle: f32,
    anticlockwise: bool,
) {
    unsafe {
        path2d_arc_1(
            instance,
            x,
            y,
            radius,
            start_angle,
            end_angle,
            if anticlockwise { 1 } else { 0 },
        )
    }
}
extern "C" {
    fn path2d_ellipse(
        instance: DOMReference,
        ellipse: f32,
        ellipse: f32,
        ellipse: f32,
        ellipse: f32,
        ellipse: f32,
        ellipse: f32,
        ellipse: f32,
        ellipse: i32,
    );
}

pub fn ellipse(
    instance: DOMReference,
    x: f32,
    y: f32,
    radius_x: f32,
    radius_y: f32,
    rotation: f32,
    start_angle: f32,
    end_angle: f32,
    anticlockwise: bool,
) {
    unsafe {
        path2d_ellipse(
            instance,
            x,
            y,
            radius_x,
            radius_y,
            rotation,
            start_angle,
            end_angle,
            if anticlockwise { 1 } else { 0 },
        )
    }
}
