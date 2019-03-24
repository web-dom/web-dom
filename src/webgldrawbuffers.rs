#[allow(unused_imports)]
use crate::*;
pub const COLOR_ATTACHMENT0_WEBGL: i32 = 0x8CE0;
pub const COLOR_ATTACHMENT1_WEBGL: i32 = 0x8CE1;
pub const COLOR_ATTACHMENT2_WEBGL: i32 = 0x8CE2;
pub const COLOR_ATTACHMENT3_WEBGL: i32 = 0x8CE3;
pub const COLOR_ATTACHMENT4_WEBGL: i32 = 0x8CE4;
pub const COLOR_ATTACHMENT5_WEBGL: i32 = 0x8CE5;
pub const COLOR_ATTACHMENT6_WEBGL: i32 = 0x8CE6;
pub const COLOR_ATTACHMENT7_WEBGL: i32 = 0x8CE7;
pub const COLOR_ATTACHMENT8_WEBGL: i32 = 0x8CE8;
pub const COLOR_ATTACHMENT9_WEBGL: i32 = 0x8CE9;
pub const COLOR_ATTACHMENT10_WEBGL: i32 = 0x8CEA;
pub const COLOR_ATTACHMENT11_WEBGL: i32 = 0x8CEB;
pub const COLOR_ATTACHMENT12_WEBGL: i32 = 0x8CEC;
pub const COLOR_ATTACHMENT13_WEBGL: i32 = 0x8CED;
pub const COLOR_ATTACHMENT14_WEBGL: i32 = 0x8CEE;
pub const COLOR_ATTACHMENT15_WEBGL: i32 = 0x8CEF;
pub const DRAW_BUFFER0_WEBGL: i32 = 0x8825;
pub const DRAW_BUFFER1_WEBGL: i32 = 0x8826;
pub const DRAW_BUFFER2_WEBGL: i32 = 0x8827;
pub const DRAW_BUFFER3_WEBGL: i32 = 0x8828;
pub const DRAW_BUFFER4_WEBGL: i32 = 0x8829;
pub const DRAW_BUFFER5_WEBGL: i32 = 0x882A;
pub const DRAW_BUFFER6_WEBGL: i32 = 0x882B;
pub const DRAW_BUFFER7_WEBGL: i32 = 0x882C;
pub const DRAW_BUFFER8_WEBGL: i32 = 0x882D;
pub const DRAW_BUFFER9_WEBGL: i32 = 0x882E;
pub const DRAW_BUFFER10_WEBGL: i32 = 0x882F;
pub const DRAW_BUFFER11_WEBGL: i32 = 0x8830;
pub const DRAW_BUFFER12_WEBGL: i32 = 0x8831;
pub const DRAW_BUFFER13_WEBGL: i32 = 0x8832;
pub const DRAW_BUFFER14_WEBGL: i32 = 0x8833;
pub const DRAW_BUFFER15_WEBGL: i32 = 0x8834;
pub const MAX_COLOR_ATTACHMENTS_WEBGL: i32 = 0x8CDF;
pub const MAX_DRAW_BUFFERS_WEBGL: i32 = 0x8824;
extern "C" {
    fn webgldrawbuffers_draw_buffers_w_e_b_g_l(
        instance: DOMReference,
        draw_buffers_w_e_b_g_l: DOMReference,
    );
}

pub fn draw_buffers_w_e_b_g_l(instance: DOMReference, buffers: DOMReference) {
    unsafe { webgldrawbuffers_draw_buffers_w_e_b_g_l(instance, buffers) }
}
