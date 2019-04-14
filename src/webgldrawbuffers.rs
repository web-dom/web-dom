#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use alloc::string::String;
pub const COLOR_ATTACHMENT0_WEBGL: f32 = 0x8CE0 as f32;
pub const COLOR_ATTACHMENT1_WEBGL: f32 = 0x8CE1 as f32;
pub const COLOR_ATTACHMENT2_WEBGL: f32 = 0x8CE2 as f32;
pub const COLOR_ATTACHMENT3_WEBGL: f32 = 0x8CE3 as f32;
pub const COLOR_ATTACHMENT4_WEBGL: f32 = 0x8CE4 as f32;
pub const COLOR_ATTACHMENT5_WEBGL: f32 = 0x8CE5 as f32;
pub const COLOR_ATTACHMENT6_WEBGL: f32 = 0x8CE6 as f32;
pub const COLOR_ATTACHMENT7_WEBGL: f32 = 0x8CE7 as f32;
pub const COLOR_ATTACHMENT8_WEBGL: f32 = 0x8CE8 as f32;
pub const COLOR_ATTACHMENT9_WEBGL: f32 = 0x8CE9 as f32;
pub const COLOR_ATTACHMENT10_WEBGL: f32 = 0x8CEA as f32;
pub const COLOR_ATTACHMENT11_WEBGL: f32 = 0x8CEB as f32;
pub const COLOR_ATTACHMENT12_WEBGL: f32 = 0x8CEC as f32;
pub const COLOR_ATTACHMENT13_WEBGL: f32 = 0x8CED as f32;
pub const COLOR_ATTACHMENT14_WEBGL: f32 = 0x8CEE as f32;
pub const COLOR_ATTACHMENT15_WEBGL: f32 = 0x8CEF as f32;
pub const DRAW_BUFFER0_WEBGL: f32 = 0x8825 as f32;
pub const DRAW_BUFFER1_WEBGL: f32 = 0x8826 as f32;
pub const DRAW_BUFFER2_WEBGL: f32 = 0x8827 as f32;
pub const DRAW_BUFFER3_WEBGL: f32 = 0x8828 as f32;
pub const DRAW_BUFFER4_WEBGL: f32 = 0x8829 as f32;
pub const DRAW_BUFFER5_WEBGL: f32 = 0x882A as f32;
pub const DRAW_BUFFER6_WEBGL: f32 = 0x882B as f32;
pub const DRAW_BUFFER7_WEBGL: f32 = 0x882C as f32;
pub const DRAW_BUFFER8_WEBGL: f32 = 0x882D as f32;
pub const DRAW_BUFFER9_WEBGL: f32 = 0x882E as f32;
pub const DRAW_BUFFER10_WEBGL: f32 = 0x882F as f32;
pub const DRAW_BUFFER11_WEBGL: f32 = 0x8830 as f32;
pub const DRAW_BUFFER12_WEBGL: f32 = 0x8831 as f32;
pub const DRAW_BUFFER13_WEBGL: f32 = 0x8832 as f32;
pub const DRAW_BUFFER14_WEBGL: f32 = 0x8833 as f32;
pub const DRAW_BUFFER15_WEBGL: f32 = 0x8834 as f32;
pub const MAX_COLOR_ATTACHMENTS_WEBGL: f32 = 0x8CDF as f32;
pub const MAX_DRAW_BUFFERS_WEBGL: f32 = 0x8824 as f32;
extern "C" {
    fn webgldrawbuffers_draw_buffers_w_e_b_g_l(
        instance: DOMReference,
        draw_buffers_w_e_b_g_l: DOMReference,
    );
}

pub fn draw_buffers_w_e_b_g_l(instance: DOMReference, buffers: DOMReference) {
    unsafe { webgldrawbuffers_draw_buffers_w_e_b_g_l(instance, buffers) }
}
