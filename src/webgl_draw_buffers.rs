#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn webgl_draw_buffers_draw_buffers_w_e_b_g_l(
        instance: DOMReference,
        draw_buffers_w_e_b_g_l: DOMReference,
    );
}

pub fn draw_buffers_w_e_b_g_l(instance: DOMReference, buffers: DOMReference) {
    unsafe { webgl_draw_buffers_draw_buffers_w_e_b_g_l(instance, buffers) }
}
