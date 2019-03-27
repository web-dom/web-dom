#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn canvas_get_canvas(instance: DOMReference) -> DOMReference;
    fn canvas_set_canvas(instance: DOMReference, value: DOMReference);
}

pub fn get_canvas(instance: DOMReference) -> DOMReference {
    unsafe { canvas_get_canvas(instance) }
}

pub fn set_canvas(instance: DOMReference, value: DOMReference) {
    unsafe {
        canvas_set_canvas(instance, value);
    }
}
extern "C" {
    fn canvas_get_moz_current_transform(instance: DOMReference) -> DOMReference;
    fn canvas_set_moz_current_transform(instance: DOMReference, value: DOMReference);
}

pub fn get_moz_current_transform(instance: DOMReference) -> DOMReference {
    unsafe { canvas_get_moz_current_transform(instance) }
}

pub fn set_moz_current_transform(instance: DOMReference, value: DOMReference) {
    unsafe {
        canvas_set_moz_current_transform(instance, value);
    }
}
extern "C" {
    fn canvas_get_moz_current_transform_inverse(instance: DOMReference) -> DOMReference;
    fn canvas_set_moz_current_transform_inverse(instance: DOMReference, value: DOMReference);
}

pub fn get_moz_current_transform_inverse(instance: DOMReference) -> DOMReference {
    unsafe { canvas_get_moz_current_transform_inverse(instance) }
}

pub fn set_moz_current_transform_inverse(instance: DOMReference, value: DOMReference) {
    unsafe {
        canvas_set_moz_current_transform_inverse(instance, value);
    }
}
extern "C" {
    fn canvas_get_moz_text_style(instance: DOMReference) -> CString;
    fn canvas_set_moz_text_style(instance: DOMReference, value: CString);
}

pub fn get_moz_text_style(instance: DOMReference) -> String {
    unsafe { to_string(canvas_get_moz_text_style(instance)) }
}

pub fn set_moz_text_style(instance: DOMReference, value: &str) {
    unsafe {
        canvas_set_moz_text_style(instance, to_cstring(value));
    }
}
extern "C" {
    fn canvas_get_moz_image_smoothing_enabled(instance: DOMReference) -> i32;
    fn canvas_set_moz_image_smoothing_enabled(instance: DOMReference, value: i32);
}

pub fn get_moz_image_smoothing_enabled(instance: DOMReference) -> bool {
    unsafe { 0 != canvas_get_moz_image_smoothing_enabled(instance) }
}

pub fn set_moz_image_smoothing_enabled(instance: DOMReference, value: bool) {
    unsafe {
        canvas_set_moz_image_smoothing_enabled(instance, if value == true { 1 } else { 0 });
    }
}
pub const DRAWWINDOW_DRAW_CARET: f32 = 0x01 as f32;
pub const DRAWWINDOW_DO_NOT_FLUSH: f32 = 0x02 as f32;
pub const DRAWWINDOW_DRAW_VIEW: f32 = 0x04 as f32;
pub const DRAWWINDOW_USE_WIDGET_LAYERS: f32 = 0x08 as f32;
pub const DRAWWINDOW_ASYNC_DECODE_IMAGES: f32 = 0x10 as f32;
extern "C" {
    fn canvas_draw_window(
        instance: DOMReference,
        draw_window: DOMReference,
        draw_window: f32,
        draw_window: f32,
        draw_window: f32,
        draw_window: f32,
        draw_window: CString,
        draw_window: f32,
    );
}

pub fn draw_window(
    instance: DOMReference,
    window: DOMReference,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    bg_color: &str,
    flags: f32,
) {
    unsafe { canvas_draw_window(instance, window, x, y, w, h, to_cstring(bg_color), flags) }
}
extern "C" {
    fn canvas_demote(instance: DOMReference);
}

pub fn demote(instance: DOMReference) {
    unsafe { canvas_demote(instance) }
}
extern "C" {
    fn canvas_save(instance: DOMReference);
}

pub fn save(instance: DOMReference) {
    unsafe { canvas_save(instance) }
}
extern "C" {
    fn canvas_restore(instance: DOMReference);
}

pub fn restore(instance: DOMReference) {
    unsafe { canvas_restore(instance) }
}
extern "C" {
    fn canvas_scale(instance: DOMReference, scale: f32, scale: f32);
}

pub fn scale(instance: DOMReference, x: f32, y: f32) {
    unsafe { canvas_scale(instance, x, y) }
}
extern "C" {
    fn canvas_rotate(instance: DOMReference, rotate: f32);
}

pub fn rotate(instance: DOMReference, angle: f32) {
    unsafe { canvas_rotate(instance, angle) }
}
extern "C" {
    fn canvas_translate(instance: DOMReference, translate: f32, translate: f32);
}

pub fn translate(instance: DOMReference, x: f32, y: f32) {
    unsafe { canvas_translate(instance, x, y) }
}
extern "C" {
    fn canvas_transform(
        instance: DOMReference,
        transform: f32,
        transform: f32,
        transform: f32,
        transform: f32,
        transform: f32,
        transform: f32,
    );
}

pub fn transform(instance: DOMReference, a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) {
    unsafe { canvas_transform(instance, a, b, c, d, e, f) }
}
extern "C" {
    fn canvas_set_transform(
        instance: DOMReference,
        set_transform: f32,
        set_transform: f32,
        set_transform: f32,
        set_transform: f32,
        set_transform: f32,
        set_transform: f32,
    );
}

pub fn set_transform(instance: DOMReference, a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) {
    unsafe { canvas_set_transform(instance, a, b, c, d, e, f) }
}
extern "C" {
    fn canvas_reset_transform(instance: DOMReference);
}

pub fn reset_transform(instance: DOMReference) {
    unsafe { canvas_reset_transform(instance) }
}
extern "C" {
    fn canvas_get_global_alpha(instance: DOMReference) -> f32;
    fn canvas_set_global_alpha(instance: DOMReference, value: f32);
}

pub fn get_global_alpha(instance: DOMReference) -> f32 {
    unsafe { canvas_get_global_alpha(instance) }
}

pub fn set_global_alpha(instance: DOMReference, value: f32) {
    unsafe {
        canvas_set_global_alpha(instance, value);
    }
}
extern "C" {
    fn canvas_get_global_composite_operation(instance: DOMReference) -> CString;
    fn canvas_set_global_composite_operation(instance: DOMReference, value: CString);
}

pub fn get_global_composite_operation(instance: DOMReference) -> String {
    unsafe { to_string(canvas_get_global_composite_operation(instance)) }
}

pub fn set_global_composite_operation(instance: DOMReference, value: &str) {
    unsafe {
        canvas_set_global_composite_operation(instance, to_cstring(value));
    }
}
extern "C" {
    fn canvas_get_image_smoothing_enabled(instance: DOMReference) -> i32;
    fn canvas_set_image_smoothing_enabled(instance: DOMReference, value: i32);
}

pub fn get_image_smoothing_enabled(instance: DOMReference) -> bool {
    unsafe { 0 != canvas_get_image_smoothing_enabled(instance) }
}

pub fn set_image_smoothing_enabled(instance: DOMReference, value: bool) {
    unsafe {
        canvas_set_image_smoothing_enabled(instance, if value == true { 1 } else { 0 });
    }
}
extern "C" {
    fn canvas_get_stroke_style(instance: DOMReference) -> CString;
    fn canvas_set_stroke_style(instance: DOMReference, value: CString);
}

pub fn get_stroke_style(instance: DOMReference) -> String {
    unsafe { to_string(canvas_get_stroke_style(instance)) }
}

pub fn set_stroke_style(instance: DOMReference, value: &str) {
    unsafe {
        canvas_set_stroke_style(instance, to_cstring(value));
    }
}
extern "C" {
    fn canvas_get_fill_style(instance: DOMReference) -> CString;
    fn canvas_set_fill_style(instance: DOMReference, value: CString);
}

pub fn get_fill_style(instance: DOMReference) -> String {
    unsafe { to_string(canvas_get_fill_style(instance)) }
}

pub fn set_fill_style(instance: DOMReference, value: &str) {
    unsafe {
        canvas_set_fill_style(instance, to_cstring(value));
    }
}
extern "C" {
    fn canvas_create_linear_gradient(
        instance: DOMReference,
        create_linear_gradient: f32,
        create_linear_gradient: f32,
        create_linear_gradient: f32,
        create_linear_gradient: f32,
    ) -> DOMReference;
}

pub fn create_linear_gradient(
    instance: DOMReference,
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
) -> DOMReference {
    unsafe { canvas_create_linear_gradient(instance, x0, y0, x1, y1) }
}
extern "C" {
    fn canvas_create_radial_gradient(
        instance: DOMReference,
        create_radial_gradient: f32,
        create_radial_gradient: f32,
        create_radial_gradient: f32,
        create_radial_gradient: f32,
        create_radial_gradient: f32,
        create_radial_gradient: f32,
    ) -> DOMReference;
}

pub fn create_radial_gradient(
    instance: DOMReference,
    x0: f32,
    y0: f32,
    r0: f32,
    x1: f32,
    y1: f32,
    r1: f32,
) -> DOMReference {
    unsafe { canvas_create_radial_gradient(instance, x0, y0, r0, x1, y1, r1) }
}
extern "C" {
    fn canvas_create_pattern(
        instance: DOMReference,
        create_pattern: DOMReference,
        create_pattern: CString,
    ) -> DOMReference;
}

pub fn create_pattern(
    instance: DOMReference,
    image: DOMReference,
    repetition: &str,
) -> DOMReference {
    unsafe { canvas_create_pattern(instance, image, to_cstring(repetition)) }
}
extern "C" {
    fn canvas_get_shadow_offset_x(instance: DOMReference) -> f32;
    fn canvas_set_shadow_offset_x(instance: DOMReference, value: f32);
}

pub fn get_shadow_offset_x(instance: DOMReference) -> f32 {
    unsafe { canvas_get_shadow_offset_x(instance) }
}

pub fn set_shadow_offset_x(instance: DOMReference, value: f32) {
    unsafe {
        canvas_set_shadow_offset_x(instance, value);
    }
}
extern "C" {
    fn canvas_get_shadow_offset_y(instance: DOMReference) -> f32;
    fn canvas_set_shadow_offset_y(instance: DOMReference, value: f32);
}

pub fn get_shadow_offset_y(instance: DOMReference) -> f32 {
    unsafe { canvas_get_shadow_offset_y(instance) }
}

pub fn set_shadow_offset_y(instance: DOMReference, value: f32) {
    unsafe {
        canvas_set_shadow_offset_y(instance, value);
    }
}
extern "C" {
    fn canvas_get_shadow_blur(instance: DOMReference) -> f32;
    fn canvas_set_shadow_blur(instance: DOMReference, value: f32);
}

pub fn get_shadow_blur(instance: DOMReference) -> f32 {
    unsafe { canvas_get_shadow_blur(instance) }
}

pub fn set_shadow_blur(instance: DOMReference, value: f32) {
    unsafe {
        canvas_set_shadow_blur(instance, value);
    }
}
extern "C" {
    fn canvas_get_shadow_color(instance: DOMReference) -> CString;
    fn canvas_set_shadow_color(instance: DOMReference, value: CString);
}

pub fn get_shadow_color(instance: DOMReference) -> String {
    unsafe { to_string(canvas_get_shadow_color(instance)) }
}

pub fn set_shadow_color(instance: DOMReference, value: &str) {
    unsafe {
        canvas_set_shadow_color(instance, to_cstring(value));
    }
}
extern "C" {
    fn canvas_get_filter(instance: DOMReference) -> CString;
    fn canvas_set_filter(instance: DOMReference, value: CString);
}

pub fn get_filter(instance: DOMReference) -> String {
    unsafe { to_string(canvas_get_filter(instance)) }
}

pub fn set_filter(instance: DOMReference, value: &str) {
    unsafe {
        canvas_set_filter(instance, to_cstring(value));
    }
}
extern "C" {
    fn canvas_clear_rect(
        instance: DOMReference,
        clear_rect: f32,
        clear_rect: f32,
        clear_rect: f32,
        clear_rect: f32,
    );
}

pub fn clear_rect(instance: DOMReference, x: f32, y: f32, w: f32, h: f32) {
    unsafe { canvas_clear_rect(instance, x, y, w, h) }
}
extern "C" {
    fn canvas_fill_rect(
        instance: DOMReference,
        fill_rect: f32,
        fill_rect: f32,
        fill_rect: f32,
        fill_rect: f32,
    );
}

pub fn fill_rect(instance: DOMReference, x: f32, y: f32, w: f32, h: f32) {
    unsafe { canvas_fill_rect(instance, x, y, w, h) }
}
extern "C" {
    fn canvas_stroke_rect(
        instance: DOMReference,
        stroke_rect: f32,
        stroke_rect: f32,
        stroke_rect: f32,
        stroke_rect: f32,
    );
}

pub fn stroke_rect(instance: DOMReference, x: f32, y: f32, w: f32, h: f32) {
    unsafe { canvas_stroke_rect(instance, x, y, w, h) }
}
extern "C" {
    fn canvas_begin_path(instance: DOMReference);
}

pub fn begin_path(instance: DOMReference) {
    unsafe { canvas_begin_path(instance) }
}
extern "C" {
    fn canvas_fill(instance: DOMReference);
}

pub fn fill(instance: DOMReference) {
    unsafe { canvas_fill(instance) }
}
extern "C" {
    fn canvas_fill_1(instance: DOMReference, fill_1: DOMReference);
}

pub fn fill_1(instance: DOMReference, winding: DOMReference) {
    unsafe { canvas_fill_1(instance, winding) }
}
extern "C" {
    fn canvas_fill_2(instance: DOMReference, fill_2: DOMReference, fill_2: DOMReference);
}

pub fn fill_2(instance: DOMReference, path: DOMReference, winding: DOMReference) {
    unsafe { canvas_fill_2(instance, path, winding) }
}
extern "C" {
    fn canvas_stroke(instance: DOMReference);
}

pub fn stroke(instance: DOMReference) {
    unsafe { canvas_stroke(instance) }
}
extern "C" {
    fn canvas_stroke_1(instance: DOMReference, stroke_1: DOMReference);
}

pub fn stroke_1(instance: DOMReference, path: DOMReference) {
    unsafe { canvas_stroke_1(instance, path) }
}
extern "C" {
    fn canvas_clip(instance: DOMReference);
}

pub fn clip(instance: DOMReference) {
    unsafe { canvas_clip(instance) }
}
extern "C" {
    fn canvas_clip_1(instance: DOMReference, clip_1: DOMReference);
}

pub fn clip_1(instance: DOMReference, winding: DOMReference) {
    unsafe { canvas_clip_1(instance, winding) }
}
extern "C" {
    fn canvas_clip_2(instance: DOMReference, clip_2: DOMReference, clip_2: DOMReference);
}

pub fn clip_2(instance: DOMReference, path: DOMReference, winding: DOMReference) {
    unsafe { canvas_clip_2(instance, path, winding) }
}
extern "C" {
    fn canvas_is_point_in_path(
        instance: DOMReference,
        is_point_in_path: f32,
        is_point_in_path: f32,
    ) -> i32;
}

pub fn is_point_in_path(instance: DOMReference, x: f32, y: f32) -> bool {
    unsafe { 0 != canvas_is_point_in_path(instance, x, y) }
}
extern "C" {
    fn canvas_is_point_in_path_1(
        instance: DOMReference,
        is_point_in_path_1: f32,
        is_point_in_path_1: f32,
        is_point_in_path_1: DOMReference,
    ) -> i32;
}

pub fn is_point_in_path_1(instance: DOMReference, x: f32, y: f32, winding: DOMReference) -> bool {
    unsafe { 0 != canvas_is_point_in_path_1(instance, x, y, winding) }
}
extern "C" {
    fn canvas_is_point_in_path_2(
        instance: DOMReference,
        is_point_in_path_2: DOMReference,
        is_point_in_path_2: f32,
        is_point_in_path_2: f32,
    ) -> i32;
}

pub fn is_point_in_path_2(instance: DOMReference, path: DOMReference, x: f32, y: f32) -> bool {
    unsafe { 0 != canvas_is_point_in_path_2(instance, path, x, y) }
}
extern "C" {
    fn canvas_is_point_in_path_3(
        instance: DOMReference,
        is_point_in_path_3: DOMReference,
        is_point_in_path_3: f32,
        is_point_in_path_3: f32,
        is_point_in_path_3: DOMReference,
    ) -> i32;
}

pub fn is_point_in_path_3(
    instance: DOMReference,
    path: DOMReference,
    x: f32,
    y: f32,
    winding: DOMReference,
) -> bool {
    unsafe { 0 != canvas_is_point_in_path_3(instance, path, x, y, winding) }
}
extern "C" {
    fn canvas_is_point_in_stroke(
        instance: DOMReference,
        is_point_in_stroke: f32,
        is_point_in_stroke: f32,
    ) -> i32;
}

pub fn is_point_in_stroke(instance: DOMReference, x: f32, y: f32) -> bool {
    unsafe { 0 != canvas_is_point_in_stroke(instance, x, y) }
}
extern "C" {
    fn canvas_is_point_in_stroke_1(
        instance: DOMReference,
        is_point_in_stroke_1: DOMReference,
        is_point_in_stroke_1: f32,
        is_point_in_stroke_1: f32,
    ) -> i32;
}

pub fn is_point_in_stroke_1(instance: DOMReference, path: DOMReference, x: f32, y: f32) -> bool {
    unsafe { 0 != canvas_is_point_in_stroke_1(instance, path, x, y) }
}
extern "C" {
    fn canvas_draw_focus_if_needed(instance: DOMReference, draw_focus_if_needed: DOMReference);
}

pub fn draw_focus_if_needed(instance: DOMReference, element: DOMReference) {
    unsafe { canvas_draw_focus_if_needed(instance, element) }
}
extern "C" {
    fn canvas_fill_text(instance: DOMReference, fill_text: CString, fill_text: f32, fill_text: f32);
}

pub fn fill_text(instance: DOMReference, text: &str, x: f32, y: f32) {
    unsafe { canvas_fill_text(instance, to_cstring(text), x, y) }
}
extern "C" {
    fn canvas_fill_text_1(
        instance: DOMReference,
        fill_text_1: CString,
        fill_text_1: f32,
        fill_text_1: f32,
        fill_text_1: f32,
    );
}

pub fn fill_text_1(instance: DOMReference, text: &str, x: f32, y: f32, max_width: f32) {
    unsafe { canvas_fill_text_1(instance, to_cstring(text), x, y, max_width) }
}
extern "C" {
    fn canvas_stroke_text(
        instance: DOMReference,
        stroke_text: CString,
        stroke_text: f32,
        stroke_text: f32,
    );
}

pub fn stroke_text(instance: DOMReference, text: &str, x: f32, y: f32) {
    unsafe { canvas_stroke_text(instance, to_cstring(text), x, y) }
}
extern "C" {
    fn canvas_stroke_text_1(
        instance: DOMReference,
        stroke_text_1: CString,
        stroke_text_1: f32,
        stroke_text_1: f32,
        stroke_text_1: f32,
    );
}

pub fn stroke_text_1(instance: DOMReference, text: &str, x: f32, y: f32, max_width: f32) {
    unsafe { canvas_stroke_text_1(instance, to_cstring(text), x, y, max_width) }
}
extern "C" {
    fn canvas_measure_text(instance: DOMReference, measure_text: CString) -> DOMReference;
}

pub fn measure_text(instance: DOMReference, text: &str) -> DOMReference {
    unsafe { canvas_measure_text(instance, to_cstring(text)) }
}
extern "C" {
    fn canvas_draw_image(
        instance: DOMReference,
        draw_image: DOMReference,
        draw_image: f32,
        draw_image: f32,
    );
}

pub fn draw_image(instance: DOMReference, image: DOMReference, dx: f32, dy: f32) {
    unsafe { canvas_draw_image(instance, image, dx, dy) }
}
extern "C" {
    fn canvas_draw_image_1(
        instance: DOMReference,
        draw_image_1: DOMReference,
        draw_image_1: f32,
        draw_image_1: f32,
        draw_image_1: f32,
        draw_image_1: f32,
    );
}

pub fn draw_image_1(
    instance: DOMReference,
    image: DOMReference,
    dx: f32,
    dy: f32,
    dw: f32,
    dh: f32,
) {
    unsafe { canvas_draw_image_1(instance, image, dx, dy, dw, dh) }
}
extern "C" {
    fn canvas_draw_image_2(
        instance: DOMReference,
        draw_image_2: DOMReference,
        draw_image_2: f32,
        draw_image_2: f32,
        draw_image_2: f32,
        draw_image_2: f32,
        draw_image_2: f32,
        draw_image_2: f32,
        draw_image_2: f32,
        draw_image_2: f32,
    );
}

pub fn draw_image_2(
    instance: DOMReference,
    image: DOMReference,
    sx: f32,
    sy: f32,
    sw: f32,
    sh: f32,
    dx: f32,
    dy: f32,
    dw: f32,
    dh: f32,
) {
    unsafe { canvas_draw_image_2(instance, image, sx, sy, sw, sh, dx, dy, dw, dh) }
}
extern "C" {
    fn canvas_create_image_data(
        instance: DOMReference,
        create_image_data: f32,
        create_image_data: f32,
    ) -> DOMReference;
}

pub fn create_image_data(instance: DOMReference, sw: f32, sh: f32) -> DOMReference {
    unsafe { canvas_create_image_data(instance, sw, sh) }
}
extern "C" {
    fn canvas_create_image_data_1(
        instance: DOMReference,
        create_image_data_1: DOMReference,
    ) -> DOMReference;
}

pub fn create_image_data_1(instance: DOMReference, imagedata: DOMReference) -> DOMReference {
    unsafe { canvas_create_image_data_1(instance, imagedata) }
}
extern "C" {
    fn canvas_get_image_data(
        instance: DOMReference,
        get_image_data: f32,
        get_image_data: f32,
        get_image_data: f32,
        get_image_data: f32,
    ) -> DOMReference;
}

pub fn get_image_data(instance: DOMReference, sx: f32, sy: f32, sw: f32, sh: f32) -> DOMReference {
    unsafe { canvas_get_image_data(instance, sx, sy, sw, sh) }
}
extern "C" {
    fn canvas_put_image_data(
        instance: DOMReference,
        put_image_data: DOMReference,
        put_image_data: f32,
        put_image_data: f32,
    );
}

pub fn put_image_data(instance: DOMReference, imagedata: DOMReference, dx: f32, dy: f32) {
    unsafe { canvas_put_image_data(instance, imagedata, dx, dy) }
}
extern "C" {
    fn canvas_put_image_data_1(
        instance: DOMReference,
        put_image_data_1: DOMReference,
        put_image_data_1: f32,
        put_image_data_1: f32,
        put_image_data_1: f32,
        put_image_data_1: f32,
        put_image_data_1: f32,
        put_image_data_1: f32,
    );
}

pub fn put_image_data_1(
    instance: DOMReference,
    imagedata: DOMReference,
    dx: f32,
    dy: f32,
    dirty_x: f32,
    dirty_y: f32,
    dirty_width: f32,
    dirty_height: f32,
) {
    unsafe {
        canvas_put_image_data_1(
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
    fn canvas_get_line_width(instance: DOMReference) -> f32;
    fn canvas_set_line_width(instance: DOMReference, value: f32);
}

pub fn get_line_width(instance: DOMReference) -> f32 {
    unsafe { canvas_get_line_width(instance) }
}

pub fn set_line_width(instance: DOMReference, value: f32) {
    unsafe {
        canvas_set_line_width(instance, value);
    }
}
extern "C" {
    fn canvas_get_line_cap(instance: DOMReference) -> CString;
    fn canvas_set_line_cap(instance: DOMReference, value: CString);
}

pub fn get_line_cap(instance: DOMReference) -> String {
    unsafe { to_string(canvas_get_line_cap(instance)) }
}

pub fn set_line_cap(instance: DOMReference, value: &str) {
    unsafe {
        canvas_set_line_cap(instance, to_cstring(value));
    }
}
extern "C" {
    fn canvas_get_line_join(instance: DOMReference) -> CString;
    fn canvas_set_line_join(instance: DOMReference, value: CString);
}

pub fn get_line_join(instance: DOMReference) -> String {
    unsafe { to_string(canvas_get_line_join(instance)) }
}

pub fn set_line_join(instance: DOMReference, value: &str) {
    unsafe {
        canvas_set_line_join(instance, to_cstring(value));
    }
}
extern "C" {
    fn canvas_get_miter_limit(instance: DOMReference) -> f32;
    fn canvas_set_miter_limit(instance: DOMReference, value: f32);
}

pub fn get_miter_limit(instance: DOMReference) -> f32 {
    unsafe { canvas_get_miter_limit(instance) }
}

pub fn set_miter_limit(instance: DOMReference, value: f32) {
    unsafe {
        canvas_set_miter_limit(instance, value);
    }
}
extern "C" {
    fn canvas_set_line_dash(instance: DOMReference, set_line_dash: DOMReference);
}

pub fn set_line_dash(instance: DOMReference, segments: DOMReference) {
    unsafe { canvas_set_line_dash(instance, segments) }
}
extern "C" {
    fn canvas_get_line_dash(instance: DOMReference) -> DOMReference;
}

pub fn get_line_dash(instance: DOMReference) -> DOMReference {
    unsafe { canvas_get_line_dash(instance) }
}
extern "C" {
    fn canvas_get_line_dash_offset(instance: DOMReference) -> f32;
    fn canvas_set_line_dash_offset(instance: DOMReference, value: f32);
}

pub fn get_line_dash_offset(instance: DOMReference) -> f32 {
    unsafe { canvas_get_line_dash_offset(instance) }
}

pub fn set_line_dash_offset(instance: DOMReference, value: f32) {
    unsafe {
        canvas_set_line_dash_offset(instance, value);
    }
}
extern "C" {
    fn canvas_get_font(instance: DOMReference) -> CString;
    fn canvas_set_font(instance: DOMReference, value: CString);
}

pub fn get_font(instance: DOMReference) -> String {
    unsafe { to_string(canvas_get_font(instance)) }
}

pub fn set_font(instance: DOMReference, value: &str) {
    unsafe {
        canvas_set_font(instance, to_cstring(value));
    }
}
extern "C" {
    fn canvas_get_text_align(instance: DOMReference) -> CString;
    fn canvas_set_text_align(instance: DOMReference, value: CString);
}

pub fn get_text_align(instance: DOMReference) -> String {
    unsafe { to_string(canvas_get_text_align(instance)) }
}

pub fn set_text_align(instance: DOMReference, value: &str) {
    unsafe {
        canvas_set_text_align(instance, to_cstring(value));
    }
}
extern "C" {
    fn canvas_get_text_baseline(instance: DOMReference) -> CString;
    fn canvas_set_text_baseline(instance: DOMReference, value: CString);
}

pub fn get_text_baseline(instance: DOMReference) -> String {
    unsafe { to_string(canvas_get_text_baseline(instance)) }
}

pub fn set_text_baseline(instance: DOMReference, value: &str) {
    unsafe {
        canvas_set_text_baseline(instance, to_cstring(value));
    }
}
extern "C" {
    fn canvas_close_path(instance: DOMReference);
}

pub fn close_path(instance: DOMReference) {
    unsafe { canvas_close_path(instance) }
}
extern "C" {
    fn canvas_move_to(instance: DOMReference, move_to: f32, move_to: f32);
}

pub fn move_to(instance: DOMReference, x: f32, y: f32) {
    unsafe { canvas_move_to(instance, x, y) }
}
extern "C" {
    fn canvas_line_to(instance: DOMReference, line_to: f32, line_to: f32);
}

pub fn line_to(instance: DOMReference, x: f32, y: f32) {
    unsafe { canvas_line_to(instance, x, y) }
}
extern "C" {
    fn canvas_quadratic_curve_to(
        instance: DOMReference,
        quadratic_curve_to: f32,
        quadratic_curve_to: f32,
        quadratic_curve_to: f32,
        quadratic_curve_to: f32,
    );
}

pub fn quadratic_curve_to(instance: DOMReference, cpx: f32, cpy: f32, x: f32, y: f32) {
    unsafe { canvas_quadratic_curve_to(instance, cpx, cpy, x, y) }
}
extern "C" {
    fn canvas_bezier_curve_to(
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
    unsafe { canvas_bezier_curve_to(instance, cp1x, cp1y, cp2x, cp2y, x, y) }
}
extern "C" {
    fn canvas_arc_to(
        instance: DOMReference,
        arc_to: f32,
        arc_to: f32,
        arc_to: f32,
        arc_to: f32,
        arc_to: f32,
    );
}

pub fn arc_to(instance: DOMReference, x1: f32, y1: f32, x2: f32, y2: f32, radius: f32) {
    unsafe { canvas_arc_to(instance, x1, y1, x2, y2, radius) }
}
extern "C" {
    fn canvas_rect(instance: DOMReference, rect: f32, rect: f32, rect: f32, rect: f32);
}

pub fn rect(instance: DOMReference, x: f32, y: f32, w: f32, h: f32) {
    unsafe { canvas_rect(instance, x, y, w, h) }
}
extern "C" {
    fn canvas_arc(instance: DOMReference, arc: f32, arc: f32, arc: f32, arc: f32, arc: f32);
}

pub fn arc(instance: DOMReference, x: f32, y: f32, radius: f32, start_angle: f32, end_angle: f32) {
    unsafe { canvas_arc(instance, x, y, radius, start_angle, end_angle) }
}
extern "C" {
    fn canvas_arc_1(
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
        canvas_arc_1(
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
    fn canvas_ellipse(
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
        canvas_ellipse(
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
extern "C" {
    fn canvas_add_hit_region(instance: DOMReference, add_hit_region: DOMReference);
}

pub fn add_hit_region(instance: DOMReference, options: DOMReference) {
    unsafe { canvas_add_hit_region(instance, options) }
}
extern "C" {
    fn canvas_remove_hit_region(instance: DOMReference, remove_hit_region: CString);
}

pub fn remove_hit_region(instance: DOMReference, id: &str) {
    unsafe { canvas_remove_hit_region(instance, to_cstring(id)) }
}
extern "C" {
    fn canvas_clear_hit_regions(instance: DOMReference);
}

pub fn clear_hit_regions(instance: DOMReference) {
    unsafe { canvas_clear_hit_regions(instance) }
}
