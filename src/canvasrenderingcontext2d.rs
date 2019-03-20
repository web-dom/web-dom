#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn canvasrenderingcontext2d_get_canvas(instance: i32) -> i32;
    fn canvasrenderingcontext2d_set_canvas(instance: i32, value: i32);
}

pub fn get_canvas(instance: i32) -> i32 {
    unsafe { canvasrenderingcontext2d_get_canvas(instance) }
}

pub fn set_canvas(instance: i32, value: i32) {
    unsafe {
        canvasrenderingcontext2d_set_canvas(instance, value);
    }
}
extern "C" {
    fn canvasrenderingcontext2d_draw_window(
        instance: i32,
        window: i32,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        bg_color: CString,
        flags: i32,
    );
}

pub fn draw_window(
    instance: i32,
    window: i32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    bg_color: &str,
    flags: i32,
) {
    unsafe {
        canvasrenderingcontext2d_draw_window(instance, window, x, y, w, h, cstr(bg_color), flags)
    }
}
extern "C" {
    fn canvasrenderingcontext2d_demote(instance: i32);
}

pub fn demote(instance: i32) {
    unsafe { canvasrenderingcontext2d_demote(instance) }
}
extern "C" {
    fn canvasrenderingcontext2d_save(instance: i32);
}

pub fn save(instance: i32) {
    unsafe { canvasrenderingcontext2d_save(instance) }
}
extern "C" {
    fn canvasrenderingcontext2d_restore(instance: i32);
}

pub fn restore(instance: i32) {
    unsafe { canvasrenderingcontext2d_restore(instance) }
}
extern "C" {
    fn canvasrenderingcontext2d_scale(instance: i32, x: i32, y: i32);
}

pub fn scale(instance: i32, x: i32, y: i32) {
    unsafe { canvasrenderingcontext2d_scale(instance, x, y) }
}
extern "C" {
    fn canvasrenderingcontext2d_rotate(instance: i32, angle: i32);
}

pub fn rotate(instance: i32, angle: i32) {
    unsafe { canvasrenderingcontext2d_rotate(instance, angle) }
}
extern "C" {
    fn canvasrenderingcontext2d_translate(instance: i32, x: i32, y: i32);
}

pub fn translate(instance: i32, x: i32, y: i32) {
    unsafe { canvasrenderingcontext2d_translate(instance, x, y) }
}
extern "C" {
    fn canvasrenderingcontext2d_transform(
        instance: i32,
        a: i32,
        b: i32,
        c: i32,
        d: i32,
        e: i32,
        f: i32,
    );
}

pub fn transform(instance: i32, a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) {
    unsafe { canvasrenderingcontext2d_transform(instance, a, b, c, d, e, f) }
}
extern "C" {
    fn canvasrenderingcontext2d_set_transform(
        instance: i32,
        a: i32,
        b: i32,
        c: i32,
        d: i32,
        e: i32,
        f: i32,
    );
}

pub fn set_transform(instance: i32, a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) {
    unsafe { canvasrenderingcontext2d_set_transform(instance, a, b, c, d, e, f) }
}
extern "C" {
    fn canvasrenderingcontext2d_reset_transform(instance: i32);
}

pub fn reset_transform(instance: i32) {
    unsafe { canvasrenderingcontext2d_reset_transform(instance) }
}
extern "C" {
    fn canvasrenderingcontext2d_get_global_alpha(instance: i32) -> i32;
    fn canvasrenderingcontext2d_set_global_alpha(instance: i32, value: i32);
}

pub fn get_global_alpha(instance: i32) -> i32 {
    unsafe { canvasrenderingcontext2d_get_global_alpha(instance) }
}

pub fn set_global_alpha(instance: i32, value: i32) {
    unsafe {
        canvasrenderingcontext2d_set_global_alpha(instance, value);
    }
}
extern "C" {
    fn canvasrenderingcontext2d_get_global_composite_operation(instance: i32) -> CString;
    fn canvasrenderingcontext2d_set_global_composite_operation(instance: i32, value: CString);
}

pub fn get_global_composite_operation(instance: i32) -> String {
    unsafe {
        cstr_to_string(canvasrenderingcontext2d_get_global_composite_operation(
            instance,
        ))
    }
}

pub fn set_global_composite_operation(instance: i32, value: &str) {
    unsafe {
        canvasrenderingcontext2d_set_global_composite_operation(instance, cstr(value));
    }
}
extern "C" {
    fn canvasrenderingcontext2d_get_image_smoothing_enabled(instance: i32) -> i32;
    fn canvasrenderingcontext2d_set_image_smoothing_enabled(instance: i32, value: i32);
}

pub fn get_image_smoothing_enabled(instance: i32) -> i32 {
    unsafe { canvasrenderingcontext2d_get_image_smoothing_enabled(instance) }
}

pub fn set_image_smoothing_enabled(instance: i32, value: i32) {
    unsafe {
        canvasrenderingcontext2d_set_image_smoothing_enabled(instance, value);
    }
}
extern "C" {
    fn canvasrenderingcontext2d_get_stroke_style(instance: i32) -> CString;
    fn canvasrenderingcontext2d_set_stroke_style(instance: i32, value: CString);
}

pub fn get_stroke_style(instance: i32) -> String {
    unsafe { cstr_to_string(canvasrenderingcontext2d_get_stroke_style(instance)) }
}

pub fn set_stroke_style(instance: i32, value: &str) {
    unsafe {
        canvasrenderingcontext2d_set_stroke_style(instance, cstr(value));
    }
}
extern "C" {
    fn canvasrenderingcontext2d_get_fill_style(instance: i32) -> CString;
    fn canvasrenderingcontext2d_set_fill_style(instance: i32, value: CString);
}

pub fn get_fill_style(instance: i32) -> String {
    unsafe { cstr_to_string(canvasrenderingcontext2d_get_fill_style(instance)) }
}

pub fn set_fill_style(instance: i32, value: &str) {
    unsafe {
        canvasrenderingcontext2d_set_fill_style(instance, cstr(value));
    }
}
extern "C" {
    fn canvasrenderingcontext2d_create_linear_gradient(
        instance: i32,
        x0: i32,
        y0: i32,
        x1: i32,
        y1: i32,
    ) -> i32;
}

pub fn create_linear_gradient(instance: i32, x0: i32, y0: i32, x1: i32, y1: i32) -> i32 {
    unsafe { canvasrenderingcontext2d_create_linear_gradient(instance, x0, y0, x1, y1) }
}
extern "C" {
    fn canvasrenderingcontext2d_create_radial_gradient(
        instance: i32,
        x0: i32,
        y0: i32,
        r0: i32,
        x1: i32,
        y1: i32,
        r1: i32,
    ) -> i32;
}

pub fn create_radial_gradient(
    instance: i32,
    x0: i32,
    y0: i32,
    r0: i32,
    x1: i32,
    y1: i32,
    r1: i32,
) -> i32 {
    unsafe { canvasrenderingcontext2d_create_radial_gradient(instance, x0, y0, r0, x1, y1, r1) }
}
extern "C" {
    fn canvasrenderingcontext2d_create_pattern(
        instance: i32,
        image: i32,
        repetition: CString,
    ) -> i32;
}

pub fn create_pattern(instance: i32, image: i32, repetition: &str) -> i32 {
    unsafe { canvasrenderingcontext2d_create_pattern(instance, image, cstr(repetition)) }
}
extern "C" {
    fn canvasrenderingcontext2d_get_shadow_offset_x(instance: i32) -> i32;
    fn canvasrenderingcontext2d_set_shadow_offset_x(instance: i32, value: i32);
}

pub fn get_shadow_offset_x(instance: i32) -> i32 {
    unsafe { canvasrenderingcontext2d_get_shadow_offset_x(instance) }
}

pub fn set_shadow_offset_x(instance: i32, value: i32) {
    unsafe {
        canvasrenderingcontext2d_set_shadow_offset_x(instance, value);
    }
}
extern "C" {
    fn canvasrenderingcontext2d_get_shadow_offset_y(instance: i32) -> i32;
    fn canvasrenderingcontext2d_set_shadow_offset_y(instance: i32, value: i32);
}

pub fn get_shadow_offset_y(instance: i32) -> i32 {
    unsafe { canvasrenderingcontext2d_get_shadow_offset_y(instance) }
}

pub fn set_shadow_offset_y(instance: i32, value: i32) {
    unsafe {
        canvasrenderingcontext2d_set_shadow_offset_y(instance, value);
    }
}
extern "C" {
    fn canvasrenderingcontext2d_get_shadow_blur(instance: i32) -> i32;
    fn canvasrenderingcontext2d_set_shadow_blur(instance: i32, value: i32);
}

pub fn get_shadow_blur(instance: i32) -> i32 {
    unsafe { canvasrenderingcontext2d_get_shadow_blur(instance) }
}

pub fn set_shadow_blur(instance: i32, value: i32) {
    unsafe {
        canvasrenderingcontext2d_set_shadow_blur(instance, value);
    }
}
extern "C" {
    fn canvasrenderingcontext2d_get_shadow_color(instance: i32) -> CString;
    fn canvasrenderingcontext2d_set_shadow_color(instance: i32, value: CString);
}

pub fn get_shadow_color(instance: i32) -> String {
    unsafe { cstr_to_string(canvasrenderingcontext2d_get_shadow_color(instance)) }
}

pub fn set_shadow_color(instance: i32, value: &str) {
    unsafe {
        canvasrenderingcontext2d_set_shadow_color(instance, cstr(value));
    }
}
extern "C" {
    fn canvasrenderingcontext2d_get_filter(instance: i32) -> CString;
    fn canvasrenderingcontext2d_set_filter(instance: i32, value: CString);
}

pub fn get_filter(instance: i32) -> String {
    unsafe { cstr_to_string(canvasrenderingcontext2d_get_filter(instance)) }
}

pub fn set_filter(instance: i32, value: &str) {
    unsafe {
        canvasrenderingcontext2d_set_filter(instance, cstr(value));
    }
}
extern "C" {
    fn canvasrenderingcontext2d_clear_rect(instance: i32, x: i32, y: i32, w: i32, h: i32);
}

pub fn clear_rect(instance: i32, x: i32, y: i32, w: i32, h: i32) {
    unsafe { canvasrenderingcontext2d_clear_rect(instance, x, y, w, h) }
}
extern "C" {
    fn canvasrenderingcontext2d_fill_rect(instance: i32, x: i32, y: i32, w: i32, h: i32);
}

pub fn fill_rect(instance: i32, x: i32, y: i32, w: i32, h: i32) {
    unsafe { canvasrenderingcontext2d_fill_rect(instance, x, y, w, h) }
}
extern "C" {
    fn canvasrenderingcontext2d_stroke_rect(instance: i32, x: i32, y: i32, w: i32, h: i32);
}

pub fn stroke_rect(instance: i32, x: i32, y: i32, w: i32, h: i32) {
    unsafe { canvasrenderingcontext2d_stroke_rect(instance, x, y, w, h) }
}
extern "C" {
    fn canvasrenderingcontext2d_begin_path(instance: i32);
}

pub fn begin_path(instance: i32) {
    unsafe { canvasrenderingcontext2d_begin_path(instance) }
}
extern "C" {
    fn canvasrenderingcontext2d_fill(instance: i32, path: i32, winding: i32);
}

pub fn fill(instance: i32, path: i32, winding: i32) {
    unsafe { canvasrenderingcontext2d_fill(instance, path, winding) }
}
extern "C" {
    fn canvasrenderingcontext2d_stroke(instance: i32, path: i32);
}

pub fn stroke(instance: i32, path: i32) {
    unsafe { canvasrenderingcontext2d_stroke(instance, path) }
}
extern "C" {
    fn canvasrenderingcontext2d_clip(instance: i32, path: i32, winding: i32);
}

pub fn clip(instance: i32, path: i32, winding: i32) {
    unsafe { canvasrenderingcontext2d_clip(instance, path, winding) }
}
extern "C" {
    fn canvasrenderingcontext2d_is_point_in_path(
        instance: i32,
        path: i32,
        x: i32,
        y: i32,
        winding: i32,
    ) -> i32;
}

pub fn is_point_in_path(instance: i32, path: i32, x: i32, y: i32, winding: i32) -> i32 {
    unsafe { canvasrenderingcontext2d_is_point_in_path(instance, path, x, y, winding) }
}
extern "C" {
    fn canvasrenderingcontext2d_is_point_in_stroke(instance: i32, path: i32, x: i32, y: i32)
        -> i32;
}

pub fn is_point_in_stroke(instance: i32, path: i32, x: i32, y: i32) -> i32 {
    unsafe { canvasrenderingcontext2d_is_point_in_stroke(instance, path, x, y) }
}
extern "C" {
    fn canvasrenderingcontext2d_draw_focus_if_needed(instance: i32, element: i32);
}

pub fn draw_focus_if_needed(instance: i32, element: i32) {
    unsafe { canvasrenderingcontext2d_draw_focus_if_needed(instance, element) }
}
extern "C" {
    fn canvasrenderingcontext2d_draw_custom_focus_ring(instance: i32, element: i32) -> i32;
}

pub fn draw_custom_focus_ring(instance: i32, element: i32) -> i32 {
    unsafe { canvasrenderingcontext2d_draw_custom_focus_ring(instance, element) }
}
extern "C" {
    fn canvasrenderingcontext2d_fill_text(
        instance: i32,
        text: CString,
        x: i32,
        y: i32,
        max_width: i32,
    );
}

pub fn fill_text(instance: i32, text: &str, x: i32, y: i32, max_width: i32) {
    unsafe { canvasrenderingcontext2d_fill_text(instance, cstr(text), x, y, max_width) }
}
extern "C" {
    fn canvasrenderingcontext2d_stroke_text(
        instance: i32,
        text: CString,
        x: i32,
        y: i32,
        max_width: i32,
    );
}

pub fn stroke_text(instance: i32, text: &str, x: i32, y: i32, max_width: i32) {
    unsafe { canvasrenderingcontext2d_stroke_text(instance, cstr(text), x, y, max_width) }
}
extern "C" {
    fn canvasrenderingcontext2d_measure_text(instance: i32, text: CString) -> i32;
}

pub fn measure_text(instance: i32, text: &str) -> i32 {
    unsafe { canvasrenderingcontext2d_measure_text(instance, cstr(text)) }
}
extern "C" {
    fn canvasrenderingcontext2d_draw_image(
        instance: i32,
        image: i32,
        sx: i32,
        sy: i32,
        sw: i32,
        sh: i32,
        dx: i32,
        dy: i32,
        dw: i32,
        dh: i32,
    );
}

pub fn draw_image(
    instance: i32,
    image: i32,
    sx: i32,
    sy: i32,
    sw: i32,
    sh: i32,
    dx: i32,
    dy: i32,
    dw: i32,
    dh: i32,
) {
    unsafe { canvasrenderingcontext2d_draw_image(instance, image, sx, sy, sw, sh, dx, dy, dw, dh) }
}
extern "C" {
    fn canvasrenderingcontext2d_create_image_data(instance: i32, sw: i32, sh: i32) -> i32;
}

pub fn create_image_data(instance: i32, sw: i32, sh: i32) -> i32 {
    unsafe { canvasrenderingcontext2d_create_image_data(instance, sw, sh) }
}
extern "C" {
    fn canvasrenderingcontext2d_get_image_data(
        instance: i32,
        sx: i32,
        sy: i32,
        sw: i32,
        sh: i32,
    ) -> i32;
}

pub fn get_image_data(instance: i32, sx: i32, sy: i32, sw: i32, sh: i32) -> i32 {
    unsafe { canvasrenderingcontext2d_get_image_data(instance, sx, sy, sw, sh) }
}
extern "C" {
    fn canvasrenderingcontext2d_put_image_data(
        instance: i32,
        imagedata: i32,
        dx: i32,
        dy: i32,
        dirty_x: i32,
        dirty_y: i32,
        dirty_width: i32,
        dirty_height: i32,
    );
}

pub fn put_image_data(
    instance: i32,
    imagedata: i32,
    dx: i32,
    dy: i32,
    dirty_x: i32,
    dirty_y: i32,
    dirty_width: i32,
    dirty_height: i32,
) {
    unsafe {
        canvasrenderingcontext2d_put_image_data(
            instance,
            imagedata,
            dx,
            dy,
            dirty_x,
            dirty_y,
            dirty_width,
            dirty_height,
        )
    }
}
extern "C" {
    fn canvasrenderingcontext2d_get_line_width(instance: i32) -> i32;
    fn canvasrenderingcontext2d_set_line_width(instance: i32, value: i32);
}

pub fn get_line_width(instance: i32) -> i32 {
    unsafe { canvasrenderingcontext2d_get_line_width(instance) }
}

pub fn set_line_width(instance: i32, value: i32) {
    unsafe {
        canvasrenderingcontext2d_set_line_width(instance, value);
    }
}
extern "C" {
    fn canvasrenderingcontext2d_get_line_cap(instance: i32) -> CString;
    fn canvasrenderingcontext2d_set_line_cap(instance: i32, value: CString);
}

pub fn get_line_cap(instance: i32) -> String {
    unsafe { cstr_to_string(canvasrenderingcontext2d_get_line_cap(instance)) }
}

pub fn set_line_cap(instance: i32, value: &str) {
    unsafe {
        canvasrenderingcontext2d_set_line_cap(instance, cstr(value));
    }
}
extern "C" {
    fn canvasrenderingcontext2d_get_line_join(instance: i32) -> CString;
    fn canvasrenderingcontext2d_set_line_join(instance: i32, value: CString);
}

pub fn get_line_join(instance: i32) -> String {
    unsafe { cstr_to_string(canvasrenderingcontext2d_get_line_join(instance)) }
}

pub fn set_line_join(instance: i32, value: &str) {
    unsafe {
        canvasrenderingcontext2d_set_line_join(instance, cstr(value));
    }
}
extern "C" {
    fn canvasrenderingcontext2d_get_miter_limit(instance: i32) -> i32;
    fn canvasrenderingcontext2d_set_miter_limit(instance: i32, value: i32);
}

pub fn get_miter_limit(instance: i32) -> i32 {
    unsafe { canvasrenderingcontext2d_get_miter_limit(instance) }
}

pub fn set_miter_limit(instance: i32, value: i32) {
    unsafe {
        canvasrenderingcontext2d_set_miter_limit(instance, value);
    }
}
extern "C" {
    fn canvasrenderingcontext2d_set_line_dash(instance: i32, segments: i32);
}

pub fn set_line_dash(instance: i32, segments: i32) {
    unsafe { canvasrenderingcontext2d_set_line_dash(instance, segments) }
}
extern "C" {
    fn canvasrenderingcontext2d_get_line_dash(instance: i32) -> i32;
}

pub fn get_line_dash(instance: i32) -> i32 {
    unsafe { canvasrenderingcontext2d_get_line_dash(instance) }
}
extern "C" {
    fn canvasrenderingcontext2d_get_line_dash_offset(instance: i32) -> i32;
    fn canvasrenderingcontext2d_set_line_dash_offset(instance: i32, value: i32);
}

pub fn get_line_dash_offset(instance: i32) -> i32 {
    unsafe { canvasrenderingcontext2d_get_line_dash_offset(instance) }
}

pub fn set_line_dash_offset(instance: i32, value: i32) {
    unsafe {
        canvasrenderingcontext2d_set_line_dash_offset(instance, value);
    }
}
extern "C" {
    fn canvasrenderingcontext2d_get_font(instance: i32) -> CString;
    fn canvasrenderingcontext2d_set_font(instance: i32, value: CString);
}

pub fn get_font(instance: i32) -> String {
    unsafe { cstr_to_string(canvasrenderingcontext2d_get_font(instance)) }
}

pub fn set_font(instance: i32, value: &str) {
    unsafe {
        canvasrenderingcontext2d_set_font(instance, cstr(value));
    }
}
extern "C" {
    fn canvasrenderingcontext2d_get_text_align(instance: i32) -> CString;
    fn canvasrenderingcontext2d_set_text_align(instance: i32, value: CString);
}

pub fn get_text_align(instance: i32) -> String {
    unsafe { cstr_to_string(canvasrenderingcontext2d_get_text_align(instance)) }
}

pub fn set_text_align(instance: i32, value: &str) {
    unsafe {
        canvasrenderingcontext2d_set_text_align(instance, cstr(value));
    }
}
extern "C" {
    fn canvasrenderingcontext2d_get_text_baseline(instance: i32) -> CString;
    fn canvasrenderingcontext2d_set_text_baseline(instance: i32, value: CString);
}

pub fn get_text_baseline(instance: i32) -> String {
    unsafe { cstr_to_string(canvasrenderingcontext2d_get_text_baseline(instance)) }
}

pub fn set_text_baseline(instance: i32, value: &str) {
    unsafe {
        canvasrenderingcontext2d_set_text_baseline(instance, cstr(value));
    }
}
extern "C" {
    fn canvasrenderingcontext2d_close_path(instance: i32);
}

pub fn close_path(instance: i32) {
    unsafe { canvasrenderingcontext2d_close_path(instance) }
}
extern "C" {
    fn canvasrenderingcontext2d_move_to(instance: i32, x: i32, y: i32);
}

pub fn move_to(instance: i32, x: i32, y: i32) {
    unsafe { canvasrenderingcontext2d_move_to(instance, x, y) }
}
extern "C" {
    fn canvasrenderingcontext2d_line_to(instance: i32, x: i32, y: i32);
}

pub fn line_to(instance: i32, x: i32, y: i32) {
    unsafe { canvasrenderingcontext2d_line_to(instance, x, y) }
}
extern "C" {
    fn canvasrenderingcontext2d_quadratic_curve_to(
        instance: i32,
        cpx: i32,
        cpy: i32,
        x: i32,
        y: i32,
    );
}

pub fn quadratic_curve_to(instance: i32, cpx: i32, cpy: i32, x: i32, y: i32) {
    unsafe { canvasrenderingcontext2d_quadratic_curve_to(instance, cpx, cpy, x, y) }
}
extern "C" {
    fn canvasrenderingcontext2d_bezier_curve_to(
        instance: i32,
        cp1x: i32,
        cp1y: i32,
        cp2x: i32,
        cp2y: i32,
        x: i32,
        y: i32,
    );
}

pub fn bezier_curve_to(instance: i32, cp1x: i32, cp1y: i32, cp2x: i32, cp2y: i32, x: i32, y: i32) {
    unsafe { canvasrenderingcontext2d_bezier_curve_to(instance, cp1x, cp1y, cp2x, cp2y, x, y) }
}
extern "C" {
    fn canvasrenderingcontext2d_arc_to(
        instance: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        radius: i32,
    );
}

pub fn arc_to(instance: i32, x1: i32, y1: i32, x2: i32, y2: i32, radius: i32) {
    unsafe { canvasrenderingcontext2d_arc_to(instance, x1, y1, x2, y2, radius) }
}
extern "C" {
    fn canvasrenderingcontext2d_rect(instance: i32, x: i32, y: i32, w: i32, h: i32);
}

pub fn rect(instance: i32, x: i32, y: i32, w: i32, h: i32) {
    unsafe { canvasrenderingcontext2d_rect(instance, x, y, w, h) }
}
extern "C" {
    fn canvasrenderingcontext2d_arc(
        instance: i32,
        x: i32,
        y: i32,
        radius: i32,
        start_angle: i32,
        end_angle: i32,
        anticlockwise: i32,
    );
}

pub fn arc(
    instance: i32,
    x: i32,
    y: i32,
    radius: i32,
    start_angle: i32,
    end_angle: i32,
    anticlockwise: i32,
) {
    unsafe {
        canvasrenderingcontext2d_arc(
            instance,
            x,
            y,
            radius,
            start_angle,
            end_angle,
            anticlockwise,
        )
    }
}
extern "C" {
    fn canvasrenderingcontext2d_ellipse(
        instance: i32,
        x: i32,
        y: i32,
        radius_x: i32,
        radius_y: i32,
        rotation: i32,
        start_angle: i32,
        end_angle: i32,
        anticlockwise: i32,
    );
}

pub fn ellipse(
    instance: i32,
    x: i32,
    y: i32,
    radius_x: i32,
    radius_y: i32,
    rotation: i32,
    start_angle: i32,
    end_angle: i32,
    anticlockwise: i32,
) {
    unsafe {
        canvasrenderingcontext2d_ellipse(
            instance,
            x,
            y,
            radius_x,
            radius_y,
            rotation,
            start_angle,
            end_angle,
            anticlockwise,
        )
    }
}
extern "C" {
    fn canvasrenderingcontext2d_add_hit_region(instance: i32, options: i32);
}

pub fn add_hit_region(instance: i32, options: i32) {
    unsafe { canvasrenderingcontext2d_add_hit_region(instance, options) }
}
extern "C" {
    fn canvasrenderingcontext2d_remove_hit_region(instance: i32, id: CString);
}

pub fn remove_hit_region(instance: i32, id: &str) {
    unsafe { canvasrenderingcontext2d_remove_hit_region(instance, cstr(id)) }
}
extern "C" {
    fn canvasrenderingcontext2d_clear_hit_regions(instance: i32);
}

pub fn clear_hit_regions(instance: i32) {
    unsafe { canvasrenderingcontext2d_clear_hit_regions(instance) }
}
