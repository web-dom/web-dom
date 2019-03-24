#[allow(unused_imports)]
use crate::*;
pub const DEPTH_BUFFER_BIT: i32 = 0x00000100;
pub const STENCIL_BUFFER_BIT: i32 = 0x00000400;
pub const COLOR_BUFFER_BIT: i32 = 0x00004000;
pub const POINTS: i32 = 0x0000;
pub const LINES: i32 = 0x0001;
pub const LINE_LOOP: i32 = 0x0002;
pub const LINE_STRIP: i32 = 0x0003;
pub const TRIANGLES: i32 = 0x0004;
pub const TRIANGLE_STRIP: i32 = 0x0005;
pub const TRIANGLE_FAN: i32 = 0x0006;
pub const ZERO: i32 = 0;
pub const ONE: i32 = 1;
pub const SRC_COLOR: i32 = 0x0300;
pub const ONE_MINUS_SRC_COLOR: i32 = 0x0301;
pub const SRC_ALPHA: i32 = 0x0302;
pub const ONE_MINUS_SRC_ALPHA: i32 = 0x0303;
pub const DST_ALPHA: i32 = 0x0304;
pub const ONE_MINUS_DST_ALPHA: i32 = 0x0305;
pub const DST_COLOR: i32 = 0x0306;
pub const ONE_MINUS_DST_COLOR: i32 = 0x0307;
pub const SRC_ALPHA_SATURATE: i32 = 0x0308;
pub const FUNC_ADD: i32 = 0x8006;
pub const BLEND_EQUATION: i32 = 0x8009;
pub const BLEND_EQUATION_RGB: i32 = 0x8009;
pub const BLEND_EQUATION_ALPHA: i32 = 0x883D;
pub const FUNC_SUBTRACT: i32 = 0x800A;
pub const FUNC_REVERSE_SUBTRACT: i32 = 0x800B;
pub const BLEND_DST_RGB: i32 = 0x80C8;
pub const BLEND_SRC_RGB: i32 = 0x80C9;
pub const BLEND_DST_ALPHA: i32 = 0x80CA;
pub const BLEND_SRC_ALPHA: i32 = 0x80CB;
pub const CONSTANT_COLOR: i32 = 0x8001;
pub const ONE_MINUS_CONSTANT_COLOR: i32 = 0x8002;
pub const CONSTANT_ALPHA: i32 = 0x8003;
pub const ONE_MINUS_CONSTANT_ALPHA: i32 = 0x8004;
pub const BLEND_COLOR: i32 = 0x8005;
pub const ARRAY_BUFFER: i32 = 0x8892;
pub const ELEMENT_ARRAY_BUFFER: i32 = 0x8893;
pub const ARRAY_BUFFER_BINDING: i32 = 0x8894;
pub const ELEMENT_ARRAY_BUFFER_BINDING: i32 = 0x8895;
pub const STREAM_DRAW: i32 = 0x88E0;
pub const STATIC_DRAW: i32 = 0x88E4;
pub const DYNAMIC_DRAW: i32 = 0x88E8;
pub const BUFFER_SIZE: i32 = 0x8764;
pub const BUFFER_USAGE: i32 = 0x8765;
pub const CURRENT_VERTEX_ATTRIB: i32 = 0x8626;
pub const FRONT: i32 = 0x0404;
pub const BACK: i32 = 0x0405;
pub const FRONT_AND_BACK: i32 = 0x0408;
pub const CULL_FACE: i32 = 0x0B44;
pub const BLEND: i32 = 0x0BE2;
pub const DITHER: i32 = 0x0BD0;
pub const STENCIL_TEST: i32 = 0x0B90;
pub const DEPTH_TEST: i32 = 0x0B71;
pub const SCISSOR_TEST: i32 = 0x0C11;
pub const POLYGON_OFFSET_FILL: i32 = 0x8037;
pub const SAMPLE_ALPHA_TO_COVERAGE: i32 = 0x809E;
pub const SAMPLE_COVERAGE: i32 = 0x80A0;
pub const NO_ERROR: i32 = 0;
pub const INVALID_ENUM: i32 = 0x0500;
pub const INVALID_VALUE: i32 = 0x0501;
pub const INVALID_OPERATION: i32 = 0x0502;
pub const OUT_OF_MEMORY: i32 = 0x0505;
pub const CW: i32 = 0x0900;
pub const CCW: i32 = 0x0901;
pub const LINE_WIDTH: i32 = 0x0B21;
pub const ALIASED_POINT_SIZE_RANGE: i32 = 0x846D;
pub const ALIASED_LINE_WIDTH_RANGE: i32 = 0x846E;
pub const CULL_FACE_MODE: i32 = 0x0B45;
pub const FRONT_FACE: i32 = 0x0B46;
pub const DEPTH_RANGE: i32 = 0x0B70;
pub const DEPTH_WRITEMASK: i32 = 0x0B72;
pub const DEPTH_CLEAR_VALUE: i32 = 0x0B73;
pub const DEPTH_FUNC: i32 = 0x0B74;
pub const STENCIL_CLEAR_VALUE: i32 = 0x0B91;
pub const STENCIL_FUNC: i32 = 0x0B92;
pub const STENCIL_FAIL: i32 = 0x0B94;
pub const STENCIL_PASS_DEPTH_FAIL: i32 = 0x0B95;
pub const STENCIL_PASS_DEPTH_PASS: i32 = 0x0B96;
pub const STENCIL_REF: i32 = 0x0B97;
pub const STENCIL_VALUE_MASK: i32 = 0x0B93;
pub const STENCIL_WRITEMASK: i32 = 0x0B98;
pub const STENCIL_BACK_FUNC: i32 = 0x8800;
pub const STENCIL_BACK_FAIL: i32 = 0x8801;
pub const STENCIL_BACK_PASS_DEPTH_FAIL: i32 = 0x8802;
pub const STENCIL_BACK_PASS_DEPTH_PASS: i32 = 0x8803;
pub const STENCIL_BACK_REF: i32 = 0x8CA3;
pub const STENCIL_BACK_VALUE_MASK: i32 = 0x8CA4;
pub const STENCIL_BACK_WRITEMASK: i32 = 0x8CA5;
pub const VIEWPORT: i32 = 0x0BA2;
pub const SCISSOR_BOX: i32 = 0x0C10;
pub const COLOR_CLEAR_VALUE: i32 = 0x0C22;
pub const COLOR_WRITEMASK: i32 = 0x0C23;
pub const UNPACK_ALIGNMENT: i32 = 0x0CF5;
pub const PACK_ALIGNMENT: i32 = 0x0D05;
pub const MAX_TEXTURE_SIZE: i32 = 0x0D33;
pub const MAX_VIEWPORT_DIMS: i32 = 0x0D3A;
pub const SUBPIXEL_BITS: i32 = 0x0D50;
pub const RED_BITS: i32 = 0x0D52;
pub const GREEN_BITS: i32 = 0x0D53;
pub const BLUE_BITS: i32 = 0x0D54;
pub const ALPHA_BITS: i32 = 0x0D55;
pub const DEPTH_BITS: i32 = 0x0D56;
pub const STENCIL_BITS: i32 = 0x0D57;
pub const POLYGON_OFFSET_UNITS: i32 = 0x2A00;
pub const POLYGON_OFFSET_FACTOR: i32 = 0x8038;
pub const TEXTURE_BINDING_2D: i32 = 0x8069;
pub const SAMPLE_BUFFERS: i32 = 0x80A8;
pub const SAMPLES: i32 = 0x80A9;
pub const SAMPLE_COVERAGE_VALUE: i32 = 0x80AA;
pub const SAMPLE_COVERAGE_INVERT: i32 = 0x80AB;
pub const COMPRESSED_TEXTURE_FORMATS: i32 = 0x86A3;
pub const DONT_CARE: i32 = 0x1100;
pub const FASTEST: i32 = 0x1101;
pub const NICEST: i32 = 0x1102;
pub const GENERATE_MIPMAP_HINT: i32 = 0x8192;
pub const BYTE: i32 = 0x1400;
pub const UNSIGNED_BYTE: i32 = 0x1401;
pub const SHORT: i32 = 0x1402;
pub const UNSIGNED_SHORT: i32 = 0x1403;
pub const INT: i32 = 0x1404;
pub const UNSIGNED_INT: i32 = 0x1405;
pub const FLOAT: i32 = 0x1406;
pub const DEPTH_COMPONENT: i32 = 0x1902;
pub const ALPHA: i32 = 0x1906;
pub const RGB: i32 = 0x1907;
pub const RGBA: i32 = 0x1908;
pub const LUMINANCE: i32 = 0x1909;
pub const LUMINANCE_ALPHA: i32 = 0x190A;
pub const UNSIGNED_SHORT_4_4_4_4: i32 = 0x8033;
pub const UNSIGNED_SHORT_5_5_5_1: i32 = 0x8034;
pub const UNSIGNED_SHORT_5_6_5: i32 = 0x8363;
pub const FRAGMENT_SHADER: i32 = 0x8B30;
pub const VERTEX_SHADER: i32 = 0x8B31;
pub const MAX_VERTEX_ATTRIBS: i32 = 0x8869;
pub const MAX_VERTEX_UNIFORM_VECTORS: i32 = 0x8DFB;
pub const MAX_VARYING_VECTORS: i32 = 0x8DFC;
pub const MAX_COMBINED_TEXTURE_IMAGE_UNITS: i32 = 0x8B4D;
pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS: i32 = 0x8B4C;
pub const MAX_TEXTURE_IMAGE_UNITS: i32 = 0x8872;
pub const MAX_FRAGMENT_UNIFORM_VECTORS: i32 = 0x8DFD;
pub const SHADER_TYPE: i32 = 0x8B4F;
pub const DELETE_STATUS: i32 = 0x8B80;
pub const LINK_STATUS: i32 = 0x8B82;
pub const VALIDATE_STATUS: i32 = 0x8B83;
pub const ATTACHED_SHADERS: i32 = 0x8B85;
pub const ACTIVE_UNIFORMS: i32 = 0x8B86;
pub const ACTIVE_ATTRIBUTES: i32 = 0x8B89;
pub const SHADING_LANGUAGE_VERSION: i32 = 0x8B8C;
pub const CURRENT_PROGRAM: i32 = 0x8B8D;
pub const NEVER: i32 = 0x0200;
pub const LESS: i32 = 0x0201;
pub const EQUAL: i32 = 0x0202;
pub const LEQUAL: i32 = 0x0203;
pub const GREATER: i32 = 0x0204;
pub const NOTEQUAL: i32 = 0x0205;
pub const GEQUAL: i32 = 0x0206;
pub const ALWAYS: i32 = 0x0207;
pub const KEEP: i32 = 0x1E00;
pub const REPLACE: i32 = 0x1E01;
pub const INCR: i32 = 0x1E02;
pub const DECR: i32 = 0x1E03;
pub const INVERT: i32 = 0x150A;
pub const INCR_WRAP: i32 = 0x8507;
pub const DECR_WRAP: i32 = 0x8508;
pub const VENDOR: i32 = 0x1F00;
pub const RENDERER: i32 = 0x1F01;
pub const VERSION: i32 = 0x1F02;
pub const NEAREST: i32 = 0x2600;
pub const LINEAR: i32 = 0x2601;
pub const NEAREST_MIPMAP_NEAREST: i32 = 0x2700;
pub const LINEAR_MIPMAP_NEAREST: i32 = 0x2701;
pub const NEAREST_MIPMAP_LINEAR: i32 = 0x2702;
pub const LINEAR_MIPMAP_LINEAR: i32 = 0x2703;
pub const TEXTURE_MAG_FILTER: i32 = 0x2800;
pub const TEXTURE_MIN_FILTER: i32 = 0x2801;
pub const TEXTURE_WRAP_S: i32 = 0x2802;
pub const TEXTURE_WRAP_T: i32 = 0x2803;
pub const TEXTURE_2D: i32 = 0x0DE1;
pub const TEXTURE: i32 = 0x1702;
pub const TEXTURE_CUBE_MAP: i32 = 0x8513;
pub const TEXTURE_BINDING_CUBE_MAP: i32 = 0x8514;
pub const TEXTURE_CUBE_MAP_POSITIVE_X: i32 = 0x8515;
pub const TEXTURE_CUBE_MAP_NEGATIVE_X: i32 = 0x8516;
pub const TEXTURE_CUBE_MAP_POSITIVE_Y: i32 = 0x8517;
pub const TEXTURE_CUBE_MAP_NEGATIVE_Y: i32 = 0x8518;
pub const TEXTURE_CUBE_MAP_POSITIVE_Z: i32 = 0x8519;
pub const TEXTURE_CUBE_MAP_NEGATIVE_Z: i32 = 0x851A;
pub const MAX_CUBE_MAP_TEXTURE_SIZE: i32 = 0x851C;
pub const TEXTURE0: i32 = 0x84C0;
pub const TEXTURE1: i32 = 0x84C1;
pub const TEXTURE2: i32 = 0x84C2;
pub const TEXTURE3: i32 = 0x84C3;
pub const TEXTURE4: i32 = 0x84C4;
pub const TEXTURE5: i32 = 0x84C5;
pub const TEXTURE6: i32 = 0x84C6;
pub const TEXTURE7: i32 = 0x84C7;
pub const TEXTURE8: i32 = 0x84C8;
pub const TEXTURE9: i32 = 0x84C9;
pub const TEXTURE10: i32 = 0x84CA;
pub const TEXTURE11: i32 = 0x84CB;
pub const TEXTURE12: i32 = 0x84CC;
pub const TEXTURE13: i32 = 0x84CD;
pub const TEXTURE14: i32 = 0x84CE;
pub const TEXTURE15: i32 = 0x84CF;
pub const TEXTURE16: i32 = 0x84D0;
pub const TEXTURE17: i32 = 0x84D1;
pub const TEXTURE18: i32 = 0x84D2;
pub const TEXTURE19: i32 = 0x84D3;
pub const TEXTURE20: i32 = 0x84D4;
pub const TEXTURE21: i32 = 0x84D5;
pub const TEXTURE22: i32 = 0x84D6;
pub const TEXTURE23: i32 = 0x84D7;
pub const TEXTURE24: i32 = 0x84D8;
pub const TEXTURE25: i32 = 0x84D9;
pub const TEXTURE26: i32 = 0x84DA;
pub const TEXTURE27: i32 = 0x84DB;
pub const TEXTURE28: i32 = 0x84DC;
pub const TEXTURE29: i32 = 0x84DD;
pub const TEXTURE30: i32 = 0x84DE;
pub const TEXTURE31: i32 = 0x84DF;
pub const ACTIVE_TEXTURE: i32 = 0x84E0;
pub const REPEAT: i32 = 0x2901;
pub const CLAMP_TO_EDGE: i32 = 0x812F;
pub const MIRRORED_REPEAT: i32 = 0x8370;
pub const FLOAT_VEC2: i32 = 0x8B50;
pub const FLOAT_VEC3: i32 = 0x8B51;
pub const FLOAT_VEC4: i32 = 0x8B52;
pub const INT_VEC2: i32 = 0x8B53;
pub const INT_VEC3: i32 = 0x8B54;
pub const INT_VEC4: i32 = 0x8B55;
pub const BOOL: i32 = 0x8B56;
pub const BOOL_VEC2: i32 = 0x8B57;
pub const BOOL_VEC3: i32 = 0x8B58;
pub const BOOL_VEC4: i32 = 0x8B59;
pub const FLOAT_MAT2: i32 = 0x8B5A;
pub const FLOAT_MAT3: i32 = 0x8B5B;
pub const FLOAT_MAT4: i32 = 0x8B5C;
pub const SAMPLER_2D: i32 = 0x8B5E;
pub const SAMPLER_CUBE: i32 = 0x8B60;
pub const VERTEX_ATTRIB_ARRAY_ENABLED: i32 = 0x8622;
pub const VERTEX_ATTRIB_ARRAY_SIZE: i32 = 0x8623;
pub const VERTEX_ATTRIB_ARRAY_STRIDE: i32 = 0x8624;
pub const VERTEX_ATTRIB_ARRAY_TYPE: i32 = 0x8625;
pub const VERTEX_ATTRIB_ARRAY_NORMALIZED: i32 = 0x886A;
pub const VERTEX_ATTRIB_ARRAY_POINTER: i32 = 0x8645;
pub const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: i32 = 0x889F;
pub const IMPLEMENTATION_COLOR_READ_TYPE: i32 = 0x8B9A;
pub const IMPLEMENTATION_COLOR_READ_FORMAT: i32 = 0x8B9B;
pub const COMPILE_STATUS: i32 = 0x8B81;
pub const LOW_FLOAT: i32 = 0x8DF0;
pub const MEDIUM_FLOAT: i32 = 0x8DF1;
pub const HIGH_FLOAT: i32 = 0x8DF2;
pub const LOW_INT: i32 = 0x8DF3;
pub const MEDIUM_INT: i32 = 0x8DF4;
pub const HIGH_INT: i32 = 0x8DF5;
pub const FRAMEBUFFER: i32 = 0x8D40;
pub const RENDERBUFFER: i32 = 0x8D41;
pub const RGBA4: i32 = 0x8056;
pub const RGB5_A1: i32 = 0x8057;
pub const RGB565: i32 = 0x8D62;
pub const DEPTH_COMPONENT16: i32 = 0x81A5;
pub const STENCIL_INDEX8: i32 = 0x8D48;
pub const DEPTH_STENCIL: i32 = 0x84F9;
pub const RENDERBUFFER_WIDTH: i32 = 0x8D42;
pub const RENDERBUFFER_HEIGHT: i32 = 0x8D43;
pub const RENDERBUFFER_INTERNAL_FORMAT: i32 = 0x8D44;
pub const RENDERBUFFER_RED_SIZE: i32 = 0x8D50;
pub const RENDERBUFFER_GREEN_SIZE: i32 = 0x8D51;
pub const RENDERBUFFER_BLUE_SIZE: i32 = 0x8D52;
pub const RENDERBUFFER_ALPHA_SIZE: i32 = 0x8D53;
pub const RENDERBUFFER_DEPTH_SIZE: i32 = 0x8D54;
pub const RENDERBUFFER_STENCIL_SIZE: i32 = 0x8D55;
pub const FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: i32 = 0x8CD0;
pub const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: i32 = 0x8CD1;
pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: i32 = 0x8CD2;
pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: i32 = 0x8CD3;
pub const COLOR_ATTACHMENT0: i32 = 0x8CE0;
pub const DEPTH_ATTACHMENT: i32 = 0x8D00;
pub const STENCIL_ATTACHMENT: i32 = 0x8D20;
pub const DEPTH_STENCIL_ATTACHMENT: i32 = 0x821A;
pub const NONE: i32 = 0;
pub const FRAMEBUFFER_COMPLETE: i32 = 0x8CD5;
pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT: i32 = 0x8CD6;
pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: i32 = 0x8CD7;
pub const FRAMEBUFFER_INCOMPLETE_DIMENSIONS: i32 = 0x8CD9;
pub const FRAMEBUFFER_UNSUPPORTED: i32 = 0x8CDD;
pub const FRAMEBUFFER_BINDING: i32 = 0x8CA6;
pub const RENDERBUFFER_BINDING: i32 = 0x8CA7;
pub const MAX_RENDERBUFFER_SIZE: i32 = 0x84E8;
pub const INVALID_FRAMEBUFFER_OPERATION: i32 = 0x0506;
pub const UNPACK_FLIP_Y_WEBGL: i32 = 0x9240;
pub const UNPACK_PREMULTIPLY_ALPHA_WEBGL: i32 = 0x9241;
pub const CONTEXT_LOST_WEBGL: i32 = 0x9242;
pub const UNPACK_COLORSPACE_CONVERSION_WEBGL: i32 = 0x9243;
pub const BROWSER_DEFAULT_WEBGL: i32 = 0x9244;
extern "C" {
    fn webgl_get_canvas(instance: DOMReference) -> DOMReference;
    fn webgl_set_canvas(instance: DOMReference, value: DOMReference);
}

pub fn get_canvas(instance: DOMReference) -> DOMReference {
    unsafe { webgl_get_canvas(instance) }
}

pub fn set_canvas(instance: DOMReference, value: DOMReference) {
    unsafe {
        webgl_set_canvas(instance, value);
    }
}
extern "C" {
    fn webgl_get_drawing_buffer_width(instance: DOMReference) -> DOMReference;
    fn webgl_set_drawing_buffer_width(instance: DOMReference, value: DOMReference);
}

pub fn get_drawing_buffer_width(instance: DOMReference) -> DOMReference {
    unsafe { webgl_get_drawing_buffer_width(instance) }
}

pub fn set_drawing_buffer_width(instance: DOMReference, value: DOMReference) {
    unsafe {
        webgl_set_drawing_buffer_width(instance, value);
    }
}
extern "C" {
    fn webgl_get_drawing_buffer_height(instance: DOMReference) -> DOMReference;
    fn webgl_set_drawing_buffer_height(instance: DOMReference, value: DOMReference);
}

pub fn get_drawing_buffer_height(instance: DOMReference) -> DOMReference {
    unsafe { webgl_get_drawing_buffer_height(instance) }
}

pub fn set_drawing_buffer_height(instance: DOMReference, value: DOMReference) {
    unsafe {
        webgl_set_drawing_buffer_height(instance, value);
    }
}
extern "C" {
    fn webgl_get_context_attributes(instance: DOMReference) -> DOMReference;
}

pub fn get_context_attributes(instance: DOMReference) -> DOMReference {
    unsafe { webgl_get_context_attributes(instance) }
}
extern "C" {
    fn webgl_is_context_lost(instance: DOMReference) -> i32;
}

pub fn is_context_lost(instance: DOMReference) -> bool {
    unsafe { 0 != webgl_is_context_lost(instance) }
}
extern "C" {
    fn webgl_get_supported_extensions(instance: DOMReference) -> DOMReference;
}

pub fn get_supported_extensions(instance: DOMReference) -> DOMReference {
    unsafe { webgl_get_supported_extensions(instance) }
}
extern "C" {
    fn webgl_get_extension(instance: DOMReference, get_extension: CString) -> DOMReference;
}

pub fn get_extension(instance: DOMReference, name: &str) -> DOMReference {
    unsafe { webgl_get_extension(instance, to_cstring(name)) }
}
extern "C" {
    fn webgl_active_texture(instance: DOMReference, active_texture: DOMReference);
}

pub fn active_texture(instance: DOMReference, texture: DOMReference) {
    unsafe { webgl_active_texture(instance, texture) }
}
extern "C" {
    fn webgl_attach_shader(
        instance: DOMReference,
        attach_shader: DOMReference,
        attach_shader: DOMReference,
    );
}

pub fn attach_shader(instance: DOMReference, program: DOMReference, shader: DOMReference) {
    unsafe { webgl_attach_shader(instance, program, shader) }
}
extern "C" {
    fn webgl_bind_attrib_location(
        instance: DOMReference,
        bind_attrib_location: DOMReference,
        bind_attrib_location: DOMReference,
        bind_attrib_location: CString,
    );
}

pub fn bind_attrib_location(
    instance: DOMReference,
    program: DOMReference,
    index: DOMReference,
    name: &str,
) {
    unsafe { webgl_bind_attrib_location(instance, program, index, to_cstring(name)) }
}
extern "C" {
    fn webgl_bind_buffer(
        instance: DOMReference,
        bind_buffer: DOMReference,
        bind_buffer: DOMReference,
    );
}

pub fn bind_buffer(instance: DOMReference, target: DOMReference, buffer: DOMReference) {
    unsafe { webgl_bind_buffer(instance, target, buffer) }
}
extern "C" {
    fn webgl_bind_framebuffer(
        instance: DOMReference,
        bind_framebuffer: DOMReference,
        bind_framebuffer: DOMReference,
    );
}

pub fn bind_framebuffer(instance: DOMReference, target: DOMReference, framebuffer: DOMReference) {
    unsafe { webgl_bind_framebuffer(instance, target, framebuffer) }
}
extern "C" {
    fn webgl_bind_renderbuffer(
        instance: DOMReference,
        bind_renderbuffer: DOMReference,
        bind_renderbuffer: DOMReference,
    );
}

pub fn bind_renderbuffer(instance: DOMReference, target: DOMReference, renderbuffer: DOMReference) {
    unsafe { webgl_bind_renderbuffer(instance, target, renderbuffer) }
}
extern "C" {
    fn webgl_bind_texture(
        instance: DOMReference,
        bind_texture: DOMReference,
        bind_texture: DOMReference,
    );
}

pub fn bind_texture(instance: DOMReference, target: DOMReference, texture: DOMReference) {
    unsafe { webgl_bind_texture(instance, target, texture) }
}
extern "C" {
    fn webgl_blend_color(
        instance: DOMReference,
        blend_color: DOMReference,
        blend_color: DOMReference,
        blend_color: DOMReference,
        blend_color: DOMReference,
    );
}

pub fn blend_color(
    instance: DOMReference,
    red: DOMReference,
    green: DOMReference,
    blue: DOMReference,
    alpha: DOMReference,
) {
    unsafe { webgl_blend_color(instance, red, green, blue, alpha) }
}
extern "C" {
    fn webgl_blend_equation(instance: DOMReference, blend_equation: DOMReference);
}

pub fn blend_equation(instance: DOMReference, mode: DOMReference) {
    unsafe { webgl_blend_equation(instance, mode) }
}
extern "C" {
    fn webgl_blend_equation_separate(
        instance: DOMReference,
        blend_equation_separate: DOMReference,
        blend_equation_separate: DOMReference,
    );
}

pub fn blend_equation_separate(
    instance: DOMReference,
    mode_r_g_b: DOMReference,
    mode_alpha: DOMReference,
) {
    unsafe { webgl_blend_equation_separate(instance, mode_r_g_b, mode_alpha) }
}
extern "C" {
    fn webgl_blend_func(instance: DOMReference, blend_func: DOMReference, blend_func: DOMReference);
}

pub fn blend_func(instance: DOMReference, sfactor: DOMReference, dfactor: DOMReference) {
    unsafe { webgl_blend_func(instance, sfactor, dfactor) }
}
extern "C" {
    fn webgl_blend_func_separate(
        instance: DOMReference,
        blend_func_separate: DOMReference,
        blend_func_separate: DOMReference,
        blend_func_separate: DOMReference,
        blend_func_separate: DOMReference,
    );
}

pub fn blend_func_separate(
    instance: DOMReference,
    src_r_g_b: DOMReference,
    dst_r_g_b: DOMReference,
    src_alpha: DOMReference,
    dst_alpha: DOMReference,
) {
    unsafe { webgl_blend_func_separate(instance, src_r_g_b, dst_r_g_b, src_alpha, dst_alpha) }
}
extern "C" {
    fn webgl_check_framebuffer_status(
        instance: DOMReference,
        check_framebuffer_status: DOMReference,
    ) -> DOMReference;
}

pub fn check_framebuffer_status(instance: DOMReference, target: DOMReference) -> DOMReference {
    unsafe { webgl_check_framebuffer_status(instance, target) }
}
extern "C" {
    fn webgl_clear(instance: DOMReference, clear: DOMReference);
}

pub fn clear(instance: DOMReference, mask: DOMReference) {
    unsafe { webgl_clear(instance, mask) }
}
extern "C" {
    fn webgl_clear_color(
        instance: DOMReference,
        clear_color: DOMReference,
        clear_color: DOMReference,
        clear_color: DOMReference,
        clear_color: DOMReference,
    );
}

pub fn clear_color(
    instance: DOMReference,
    red: DOMReference,
    green: DOMReference,
    blue: DOMReference,
    alpha: DOMReference,
) {
    unsafe { webgl_clear_color(instance, red, green, blue, alpha) }
}
extern "C" {
    fn webgl_clear_depth(instance: DOMReference, clear_depth: DOMReference);
}

pub fn clear_depth(instance: DOMReference, depth: DOMReference) {
    unsafe { webgl_clear_depth(instance, depth) }
}
extern "C" {
    fn webgl_clear_stencil(instance: DOMReference, clear_stencil: DOMReference);
}

pub fn clear_stencil(instance: DOMReference, s: DOMReference) {
    unsafe { webgl_clear_stencil(instance, s) }
}
extern "C" {
    fn webgl_color_mask(
        instance: DOMReference,
        color_mask: DOMReference,
        color_mask: DOMReference,
        color_mask: DOMReference,
        color_mask: DOMReference,
    );
}

pub fn color_mask(
    instance: DOMReference,
    red: DOMReference,
    green: DOMReference,
    blue: DOMReference,
    alpha: DOMReference,
) {
    unsafe { webgl_color_mask(instance, red, green, blue, alpha) }
}
extern "C" {
    fn webgl_compile_shader(instance: DOMReference, compile_shader: DOMReference);
}

pub fn compile_shader(instance: DOMReference, shader: DOMReference) {
    unsafe { webgl_compile_shader(instance, shader) }
}
extern "C" {
    fn webgl_copy_tex_image2_d(
        instance: DOMReference,
        copy_tex_image2_d: DOMReference,
        copy_tex_image2_d: DOMReference,
        copy_tex_image2_d: DOMReference,
        copy_tex_image2_d: DOMReference,
        copy_tex_image2_d: DOMReference,
        copy_tex_image2_d: DOMReference,
        copy_tex_image2_d: DOMReference,
        copy_tex_image2_d: DOMReference,
    );
}

pub fn copy_tex_image2_d(
    instance: DOMReference,
    target: DOMReference,
    level: DOMReference,
    internalformat: DOMReference,
    x: DOMReference,
    y: DOMReference,
    width: DOMReference,
    height: DOMReference,
    border: DOMReference,
) {
    unsafe {
        webgl_copy_tex_image2_d(
            instance,
            target,
            level,
            internalformat,
            x,
            y,
            width,
            height,
            border,
        )
    }
}
extern "C" {
    fn webgl_copy_tex_sub_image2_d(
        instance: DOMReference,
        copy_tex_sub_image2_d: DOMReference,
        copy_tex_sub_image2_d: DOMReference,
        copy_tex_sub_image2_d: DOMReference,
        copy_tex_sub_image2_d: DOMReference,
        copy_tex_sub_image2_d: DOMReference,
        copy_tex_sub_image2_d: DOMReference,
        copy_tex_sub_image2_d: DOMReference,
        copy_tex_sub_image2_d: DOMReference,
    );
}

pub fn copy_tex_sub_image2_d(
    instance: DOMReference,
    target: DOMReference,
    level: DOMReference,
    xoffset: DOMReference,
    yoffset: DOMReference,
    x: DOMReference,
    y: DOMReference,
    width: DOMReference,
    height: DOMReference,
) {
    unsafe {
        webgl_copy_tex_sub_image2_d(
            instance, target, level, xoffset, yoffset, x, y, width, height,
        )
    }
}
extern "C" {
    fn webgl_create_buffer(instance: DOMReference) -> DOMReference;
}

pub fn create_buffer(instance: DOMReference) -> DOMReference {
    unsafe { webgl_create_buffer(instance) }
}
extern "C" {
    fn webgl_create_framebuffer(instance: DOMReference) -> DOMReference;
}

pub fn create_framebuffer(instance: DOMReference) -> DOMReference {
    unsafe { webgl_create_framebuffer(instance) }
}
extern "C" {
    fn webgl_create_program(instance: DOMReference) -> DOMReference;
}

pub fn create_program(instance: DOMReference) -> DOMReference {
    unsafe { webgl_create_program(instance) }
}
extern "C" {
    fn webgl_create_renderbuffer(instance: DOMReference) -> DOMReference;
}

pub fn create_renderbuffer(instance: DOMReference) -> DOMReference {
    unsafe { webgl_create_renderbuffer(instance) }
}
extern "C" {
    fn webgl_create_shader(instance: DOMReference, create_shader: DOMReference) -> DOMReference;
}

pub fn create_shader(instance: DOMReference, shader_type: DOMReference) -> DOMReference {
    unsafe { webgl_create_shader(instance, shader_type) }
}
extern "C" {
    fn webgl_create_texture(instance: DOMReference) -> DOMReference;
}

pub fn create_texture(instance: DOMReference) -> DOMReference {
    unsafe { webgl_create_texture(instance) }
}
extern "C" {
    fn webgl_cull_face(instance: DOMReference, cull_face: DOMReference);
}

pub fn cull_face(instance: DOMReference, mode: DOMReference) {
    unsafe { webgl_cull_face(instance, mode) }
}
extern "C" {
    fn webgl_delete_buffer(instance: DOMReference, delete_buffer: DOMReference);
}

pub fn delete_buffer(instance: DOMReference, buffer: DOMReference) {
    unsafe { webgl_delete_buffer(instance, buffer) }
}
extern "C" {
    fn webgl_delete_framebuffer(instance: DOMReference, delete_framebuffer: DOMReference);
}

pub fn delete_framebuffer(instance: DOMReference, framebuffer: DOMReference) {
    unsafe { webgl_delete_framebuffer(instance, framebuffer) }
}
extern "C" {
    fn webgl_delete_program(instance: DOMReference, delete_program: DOMReference);
}

pub fn delete_program(instance: DOMReference, program: DOMReference) {
    unsafe { webgl_delete_program(instance, program) }
}
extern "C" {
    fn webgl_delete_renderbuffer(instance: DOMReference, delete_renderbuffer: DOMReference);
}

pub fn delete_renderbuffer(instance: DOMReference, renderbuffer: DOMReference) {
    unsafe { webgl_delete_renderbuffer(instance, renderbuffer) }
}
extern "C" {
    fn webgl_delete_shader(instance: DOMReference, delete_shader: DOMReference);
}

pub fn delete_shader(instance: DOMReference, shader: DOMReference) {
    unsafe { webgl_delete_shader(instance, shader) }
}
extern "C" {
    fn webgl_delete_texture(instance: DOMReference, delete_texture: DOMReference);
}

pub fn delete_texture(instance: DOMReference, texture: DOMReference) {
    unsafe { webgl_delete_texture(instance, texture) }
}
extern "C" {
    fn webgl_depth_func(instance: DOMReference, depth_func: DOMReference);
}

pub fn depth_func(instance: DOMReference, func: DOMReference) {
    unsafe { webgl_depth_func(instance, func) }
}
extern "C" {
    fn webgl_depth_mask(instance: DOMReference, depth_mask: DOMReference);
}

pub fn depth_mask(instance: DOMReference, flag: DOMReference) {
    unsafe { webgl_depth_mask(instance, flag) }
}
extern "C" {
    fn webgl_depth_range(
        instance: DOMReference,
        depth_range: DOMReference,
        depth_range: DOMReference,
    );
}

pub fn depth_range(instance: DOMReference, z_near: DOMReference, z_far: DOMReference) {
    unsafe { webgl_depth_range(instance, z_near, z_far) }
}
extern "C" {
    fn webgl_detach_shader(
        instance: DOMReference,
        detach_shader: DOMReference,
        detach_shader: DOMReference,
    );
}

pub fn detach_shader(instance: DOMReference, program: DOMReference, shader: DOMReference) {
    unsafe { webgl_detach_shader(instance, program, shader) }
}
extern "C" {
    fn webgl_disable(instance: DOMReference, disable: DOMReference);
}

pub fn disable(instance: DOMReference, cap: DOMReference) {
    unsafe { webgl_disable(instance, cap) }
}
extern "C" {
    fn webgl_disable_vertex_attrib_array(
        instance: DOMReference,
        disable_vertex_attrib_array: DOMReference,
    );
}

pub fn disable_vertex_attrib_array(instance: DOMReference, index: DOMReference) {
    unsafe { webgl_disable_vertex_attrib_array(instance, index) }
}
extern "C" {
    fn webgl_draw_arrays(
        instance: DOMReference,
        draw_arrays: DOMReference,
        draw_arrays: DOMReference,
        draw_arrays: DOMReference,
    );
}

pub fn draw_arrays(
    instance: DOMReference,
    mode: DOMReference,
    first: DOMReference,
    count: DOMReference,
) {
    unsafe { webgl_draw_arrays(instance, mode, first, count) }
}
extern "C" {
    fn webgl_draw_elements(
        instance: DOMReference,
        draw_elements: DOMReference,
        draw_elements: DOMReference,
        draw_elements: DOMReference,
        draw_elements: DOMReference,
    );
}

pub fn draw_elements(
    instance: DOMReference,
    mode: DOMReference,
    count: DOMReference,
    element_type: DOMReference,
    offset: DOMReference,
) {
    unsafe { webgl_draw_elements(instance, mode, count, element_type, offset) }
}
extern "C" {
    fn webgl_enable(instance: DOMReference, enable: DOMReference);
}

pub fn enable(instance: DOMReference, cap: DOMReference) {
    unsafe { webgl_enable(instance, cap) }
}
extern "C" {
    fn webgl_enable_vertex_attrib_array(
        instance: DOMReference,
        enable_vertex_attrib_array: DOMReference,
    );
}

pub fn enable_vertex_attrib_array(instance: DOMReference, index: DOMReference) {
    unsafe { webgl_enable_vertex_attrib_array(instance, index) }
}
extern "C" {
    fn webgl_finish(instance: DOMReference);
}

pub fn finish(instance: DOMReference) {
    unsafe { webgl_finish(instance) }
}
extern "C" {
    fn webgl_flush(instance: DOMReference);
}

pub fn flush(instance: DOMReference) {
    unsafe { webgl_flush(instance) }
}
extern "C" {
    fn webgl_framebuffer_renderbuffer(
        instance: DOMReference,
        framebuffer_renderbuffer: DOMReference,
        framebuffer_renderbuffer: DOMReference,
        framebuffer_renderbuffer: DOMReference,
        framebuffer_renderbuffer: DOMReference,
    );
}

pub fn framebuffer_renderbuffer(
    instance: DOMReference,
    target: DOMReference,
    attachment: DOMReference,
    renderbuffertarget: DOMReference,
    renderbuffer: DOMReference,
) {
    unsafe {
        webgl_framebuffer_renderbuffer(
            instance,
            target,
            attachment,
            renderbuffertarget,
            renderbuffer,
        )
    }
}
extern "C" {
    fn webgl_framebuffer_texture2_d(
        instance: DOMReference,
        framebuffer_texture2_d: DOMReference,
        framebuffer_texture2_d: DOMReference,
        framebuffer_texture2_d: DOMReference,
        framebuffer_texture2_d: DOMReference,
        framebuffer_texture2_d: DOMReference,
    );
}

pub fn framebuffer_texture2_d(
    instance: DOMReference,
    target: DOMReference,
    attachment: DOMReference,
    textarget: DOMReference,
    texture: DOMReference,
    level: DOMReference,
) {
    unsafe { webgl_framebuffer_texture2_d(instance, target, attachment, textarget, texture, level) }
}
extern "C" {
    fn webgl_front_face(instance: DOMReference, front_face: DOMReference);
}

pub fn front_face(instance: DOMReference, mode: DOMReference) {
    unsafe { webgl_front_face(instance, mode) }
}
extern "C" {
    fn webgl_generate_mipmap(instance: DOMReference, generate_mipmap: DOMReference);
}

pub fn generate_mipmap(instance: DOMReference, target: DOMReference) {
    unsafe { webgl_generate_mipmap(instance, target) }
}
extern "C" {
    fn webgl_get_active_attrib(
        instance: DOMReference,
        get_active_attrib: DOMReference,
        get_active_attrib: DOMReference,
    ) -> DOMReference;
}

pub fn get_active_attrib(
    instance: DOMReference,
    program: DOMReference,
    index: DOMReference,
) -> DOMReference {
    unsafe { webgl_get_active_attrib(instance, program, index) }
}
extern "C" {
    fn webgl_get_active_uniform(
        instance: DOMReference,
        get_active_uniform: DOMReference,
        get_active_uniform: DOMReference,
    ) -> DOMReference;
}

pub fn get_active_uniform(
    instance: DOMReference,
    program: DOMReference,
    index: DOMReference,
) -> DOMReference {
    unsafe { webgl_get_active_uniform(instance, program, index) }
}
extern "C" {
    fn webgl_get_attached_shaders(
        instance: DOMReference,
        get_attached_shaders: DOMReference,
    ) -> DOMReference;
}

pub fn get_attached_shaders(instance: DOMReference, program: DOMReference) -> DOMReference {
    unsafe { webgl_get_attached_shaders(instance, program) }
}
extern "C" {
    fn webgl_get_attrib_location(
        instance: DOMReference,
        get_attrib_location: DOMReference,
        get_attrib_location: CString,
    ) -> DOMReference;
}

pub fn get_attrib_location(
    instance: DOMReference,
    program: DOMReference,
    name: &str,
) -> DOMReference {
    unsafe { webgl_get_attrib_location(instance, program, to_cstring(name)) }
}
extern "C" {
    fn webgl_get_buffer_parameter(
        instance: DOMReference,
        get_buffer_parameter: DOMReference,
        get_buffer_parameter: DOMReference,
    ) -> DOMReference;
}

pub fn get_buffer_parameter(
    instance: DOMReference,
    target: DOMReference,
    pname: DOMReference,
) -> DOMReference {
    unsafe { webgl_get_buffer_parameter(instance, target, pname) }
}
extern "C" {
    fn webgl_get_parameter(instance: DOMReference, get_parameter: DOMReference) -> DOMReference;
}

pub fn get_parameter(instance: DOMReference, pname: DOMReference) -> DOMReference {
    unsafe { webgl_get_parameter(instance, pname) }
}
extern "C" {
    fn webgl_get_error(instance: DOMReference) -> DOMReference;
}

pub fn get_error(instance: DOMReference) -> DOMReference {
    unsafe { webgl_get_error(instance) }
}
extern "C" {
    fn webgl_get_framebuffer_attachment_parameter(
        instance: DOMReference,
        get_framebuffer_attachment_parameter: DOMReference,
        get_framebuffer_attachment_parameter: DOMReference,
        get_framebuffer_attachment_parameter: DOMReference,
    ) -> DOMReference;
}

pub fn get_framebuffer_attachment_parameter(
    instance: DOMReference,
    target: DOMReference,
    attachment: DOMReference,
    pname: DOMReference,
) -> DOMReference {
    unsafe { webgl_get_framebuffer_attachment_parameter(instance, target, attachment, pname) }
}
extern "C" {
    fn webgl_get_program_parameter(
        instance: DOMReference,
        get_program_parameter: DOMReference,
        get_program_parameter: DOMReference,
    ) -> DOMReference;
}

pub fn get_program_parameter(
    instance: DOMReference,
    program: DOMReference,
    pname: DOMReference,
) -> DOMReference {
    unsafe { webgl_get_program_parameter(instance, program, pname) }
}
extern "C" {
    fn webgl_get_program_info_log(
        instance: DOMReference,
        get_program_info_log: DOMReference,
    ) -> DOMReference;
}

pub fn get_program_info_log(instance: DOMReference, program: DOMReference) -> DOMReference {
    unsafe { webgl_get_program_info_log(instance, program) }
}
extern "C" {
    fn webgl_get_renderbuffer_parameter(
        instance: DOMReference,
        get_renderbuffer_parameter: DOMReference,
        get_renderbuffer_parameter: DOMReference,
    ) -> DOMReference;
}

pub fn get_renderbuffer_parameter(
    instance: DOMReference,
    target: DOMReference,
    pname: DOMReference,
) -> DOMReference {
    unsafe { webgl_get_renderbuffer_parameter(instance, target, pname) }
}
extern "C" {
    fn webgl_get_shader_parameter(
        instance: DOMReference,
        get_shader_parameter: DOMReference,
        get_shader_parameter: DOMReference,
    ) -> DOMReference;
}

pub fn get_shader_parameter(
    instance: DOMReference,
    shader: DOMReference,
    pname: DOMReference,
) -> DOMReference {
    unsafe { webgl_get_shader_parameter(instance, shader, pname) }
}
extern "C" {
    fn webgl_get_shader_precision_format(
        instance: DOMReference,
        get_shader_precision_format: DOMReference,
        get_shader_precision_format: DOMReference,
    ) -> DOMReference;
}

pub fn get_shader_precision_format(
    instance: DOMReference,
    shadertype: DOMReference,
    precisiontype: DOMReference,
) -> DOMReference {
    unsafe { webgl_get_shader_precision_format(instance, shadertype, precisiontype) }
}
extern "C" {
    fn webgl_get_shader_info_log(
        instance: DOMReference,
        get_shader_info_log: DOMReference,
    ) -> DOMReference;
}

pub fn get_shader_info_log(instance: DOMReference, shader: DOMReference) -> DOMReference {
    unsafe { webgl_get_shader_info_log(instance, shader) }
}
extern "C" {
    fn webgl_get_shader_source(
        instance: DOMReference,
        get_shader_source: DOMReference,
    ) -> DOMReference;
}

pub fn get_shader_source(instance: DOMReference, shader: DOMReference) -> DOMReference {
    unsafe { webgl_get_shader_source(instance, shader) }
}
extern "C" {
    fn webgl_get_tex_parameter(
        instance: DOMReference,
        get_tex_parameter: DOMReference,
        get_tex_parameter: DOMReference,
    ) -> DOMReference;
}

pub fn get_tex_parameter(
    instance: DOMReference,
    target: DOMReference,
    pname: DOMReference,
) -> DOMReference {
    unsafe { webgl_get_tex_parameter(instance, target, pname) }
}
extern "C" {
    fn webgl_get_uniform(
        instance: DOMReference,
        get_uniform: DOMReference,
        get_uniform: DOMReference,
    ) -> DOMReference;
}

pub fn get_uniform(
    instance: DOMReference,
    program: DOMReference,
    location: DOMReference,
) -> DOMReference {
    unsafe { webgl_get_uniform(instance, program, location) }
}
extern "C" {
    fn webgl_get_uniform_location(
        instance: DOMReference,
        get_uniform_location: DOMReference,
        get_uniform_location: CString,
    ) -> DOMReference;
}

pub fn get_uniform_location(
    instance: DOMReference,
    program: DOMReference,
    name: &str,
) -> DOMReference {
    unsafe { webgl_get_uniform_location(instance, program, to_cstring(name)) }
}
extern "C" {
    fn webgl_get_vertex_attrib(
        instance: DOMReference,
        get_vertex_attrib: DOMReference,
        get_vertex_attrib: DOMReference,
    ) -> DOMReference;
}

pub fn get_vertex_attrib(
    instance: DOMReference,
    index: DOMReference,
    pname: DOMReference,
) -> DOMReference {
    unsafe { webgl_get_vertex_attrib(instance, index, pname) }
}
extern "C" {
    fn webgl_get_vertex_attrib_offset(
        instance: DOMReference,
        get_vertex_attrib_offset: DOMReference,
        get_vertex_attrib_offset: DOMReference,
    ) -> DOMReference;
}

pub fn get_vertex_attrib_offset(
    instance: DOMReference,
    index: DOMReference,
    pname: DOMReference,
) -> DOMReference {
    unsafe { webgl_get_vertex_attrib_offset(instance, index, pname) }
}
extern "C" {
    fn webgl_hint(instance: DOMReference, hint: DOMReference, hint: DOMReference);
}

pub fn hint(instance: DOMReference, target: DOMReference, mode: DOMReference) {
    unsafe { webgl_hint(instance, target, mode) }
}
extern "C" {
    fn webgl_is_buffer(instance: DOMReference, is_buffer: DOMReference) -> DOMReference;
}

pub fn is_buffer(instance: DOMReference, buffer: DOMReference) -> DOMReference {
    unsafe { webgl_is_buffer(instance, buffer) }
}
extern "C" {
    fn webgl_is_enabled(instance: DOMReference, is_enabled: DOMReference) -> DOMReference;
}

pub fn is_enabled(instance: DOMReference, cap: DOMReference) -> DOMReference {
    unsafe { webgl_is_enabled(instance, cap) }
}
extern "C" {
    fn webgl_is_framebuffer(instance: DOMReference, is_framebuffer: DOMReference) -> DOMReference;
}

pub fn is_framebuffer(instance: DOMReference, framebuffer: DOMReference) -> DOMReference {
    unsafe { webgl_is_framebuffer(instance, framebuffer) }
}
extern "C" {
    fn webgl_is_program(instance: DOMReference, is_program: DOMReference) -> DOMReference;
}

pub fn is_program(instance: DOMReference, program: DOMReference) -> DOMReference {
    unsafe { webgl_is_program(instance, program) }
}
extern "C" {
    fn webgl_is_renderbuffer(instance: DOMReference, is_renderbuffer: DOMReference)
        -> DOMReference;
}

pub fn is_renderbuffer(instance: DOMReference, renderbuffer: DOMReference) -> DOMReference {
    unsafe { webgl_is_renderbuffer(instance, renderbuffer) }
}
extern "C" {
    fn webgl_is_shader(instance: DOMReference, is_shader: DOMReference) -> DOMReference;
}

pub fn is_shader(instance: DOMReference, shader: DOMReference) -> DOMReference {
    unsafe { webgl_is_shader(instance, shader) }
}
extern "C" {
    fn webgl_is_texture(instance: DOMReference, is_texture: DOMReference) -> DOMReference;
}

pub fn is_texture(instance: DOMReference, texture: DOMReference) -> DOMReference {
    unsafe { webgl_is_texture(instance, texture) }
}
extern "C" {
    fn webgl_line_width(instance: DOMReference, line_width: DOMReference);
}

pub fn line_width(instance: DOMReference, width: DOMReference) {
    unsafe { webgl_line_width(instance, width) }
}
extern "C" {
    fn webgl_link_program(instance: DOMReference, link_program: DOMReference);
}

pub fn link_program(instance: DOMReference, program: DOMReference) {
    unsafe { webgl_link_program(instance, program) }
}
extern "C" {
    fn webgl_pixel_storei(
        instance: DOMReference,
        pixel_storei: DOMReference,
        pixel_storei: DOMReference,
    );
}

pub fn pixel_storei(instance: DOMReference, pname: DOMReference, param: DOMReference) {
    unsafe { webgl_pixel_storei(instance, pname, param) }
}
extern "C" {
    fn webgl_polygon_offset(
        instance: DOMReference,
        polygon_offset: DOMReference,
        polygon_offset: DOMReference,
    );
}

pub fn polygon_offset(instance: DOMReference, factor: DOMReference, units: DOMReference) {
    unsafe { webgl_polygon_offset(instance, factor, units) }
}
extern "C" {
    fn webgl_renderbuffer_storage(
        instance: DOMReference,
        renderbuffer_storage: DOMReference,
        renderbuffer_storage: DOMReference,
        renderbuffer_storage: DOMReference,
        renderbuffer_storage: DOMReference,
    );
}

pub fn renderbuffer_storage(
    instance: DOMReference,
    target: DOMReference,
    internalformat: DOMReference,
    width: DOMReference,
    height: DOMReference,
) {
    unsafe { webgl_renderbuffer_storage(instance, target, internalformat, width, height) }
}
extern "C" {
    fn webgl_sample_coverage(
        instance: DOMReference,
        sample_coverage: DOMReference,
        sample_coverage: DOMReference,
    );
}

pub fn sample_coverage(instance: DOMReference, value: DOMReference, invert: DOMReference) {
    unsafe { webgl_sample_coverage(instance, value, invert) }
}
extern "C" {
    fn webgl_scissor(
        instance: DOMReference,
        scissor: DOMReference,
        scissor: DOMReference,
        scissor: DOMReference,
        scissor: DOMReference,
    );
}

pub fn scissor(
    instance: DOMReference,
    x: DOMReference,
    y: DOMReference,
    width: DOMReference,
    height: DOMReference,
) {
    unsafe { webgl_scissor(instance, x, y, width, height) }
}
extern "C" {
    fn webgl_shader_source(
        instance: DOMReference,
        shader_source: DOMReference,
        shader_source: CString,
    );
}

pub fn shader_source(instance: DOMReference, shader: DOMReference, source: &str) {
    unsafe { webgl_shader_source(instance, shader, to_cstring(source)) }
}
extern "C" {
    fn webgl_stencil_func(
        instance: DOMReference,
        stencil_func: DOMReference,
        stencil_func: DOMReference,
        stencil_func: DOMReference,
    );
}

pub fn stencil_func(
    instance: DOMReference,
    func: DOMReference,
    setencel_ref: DOMReference,
    mask: DOMReference,
) {
    unsafe { webgl_stencil_func(instance, func, setencel_ref, mask) }
}
extern "C" {
    fn webgl_stencil_func_separate(
        instance: DOMReference,
        stencil_func_separate: DOMReference,
        stencil_func_separate: DOMReference,
        stencil_func_separate: DOMReference,
        stencil_func_separate: DOMReference,
    );
}

pub fn stencil_func_separate(
    instance: DOMReference,
    face: DOMReference,
    func: DOMReference,
    setencel_ref: DOMReference,
    mask: DOMReference,
) {
    unsafe { webgl_stencil_func_separate(instance, face, func, setencel_ref, mask) }
}
extern "C" {
    fn webgl_stencil_mask(instance: DOMReference, stencil_mask: DOMReference);
}

pub fn stencil_mask(instance: DOMReference, mask: DOMReference) {
    unsafe { webgl_stencil_mask(instance, mask) }
}
extern "C" {
    fn webgl_stencil_mask_separate(
        instance: DOMReference,
        stencil_mask_separate: DOMReference,
        stencil_mask_separate: DOMReference,
    );
}

pub fn stencil_mask_separate(instance: DOMReference, face: DOMReference, mask: DOMReference) {
    unsafe { webgl_stencil_mask_separate(instance, face, mask) }
}
extern "C" {
    fn webgl_stencil_op(
        instance: DOMReference,
        stencil_op: DOMReference,
        stencil_op: DOMReference,
        stencil_op: DOMReference,
    );
}

pub fn stencil_op(
    instance: DOMReference,
    fail: DOMReference,
    zfail: DOMReference,
    zpass: DOMReference,
) {
    unsafe { webgl_stencil_op(instance, fail, zfail, zpass) }
}
extern "C" {
    fn webgl_stencil_op_separate(
        instance: DOMReference,
        stencil_op_separate: DOMReference,
        stencil_op_separate: DOMReference,
        stencil_op_separate: DOMReference,
        stencil_op_separate: DOMReference,
    );
}

pub fn stencil_op_separate(
    instance: DOMReference,
    face: DOMReference,
    fail: DOMReference,
    zfail: DOMReference,
    zpass: DOMReference,
) {
    unsafe { webgl_stencil_op_separate(instance, face, fail, zfail, zpass) }
}
extern "C" {
    fn webgl_tex_parameterf(
        instance: DOMReference,
        tex_parameterf: DOMReference,
        tex_parameterf: DOMReference,
        tex_parameterf: DOMReference,
    );
}

pub fn tex_parameterf(
    instance: DOMReference,
    target: DOMReference,
    pname: DOMReference,
    param: DOMReference,
) {
    unsafe { webgl_tex_parameterf(instance, target, pname, param) }
}
extern "C" {
    fn webgl_tex_parameteri(
        instance: DOMReference,
        tex_parameteri: DOMReference,
        tex_parameteri: DOMReference,
        tex_parameteri: DOMReference,
    );
}

pub fn tex_parameteri(
    instance: DOMReference,
    target: DOMReference,
    pname: DOMReference,
    param: DOMReference,
) {
    unsafe { webgl_tex_parameteri(instance, target, pname, param) }
}
extern "C" {
    fn webgl_uniform1f(instance: DOMReference, uniform1f: DOMReference, uniform1f: DOMReference);
}

pub fn uniform1f(instance: DOMReference, location: DOMReference, x: DOMReference) {
    unsafe { webgl_uniform1f(instance, location, x) }
}
extern "C" {
    fn webgl_uniform2f(
        instance: DOMReference,
        uniform2f: DOMReference,
        uniform2f: DOMReference,
        uniform2f: DOMReference,
    );
}

pub fn uniform2f(instance: DOMReference, location: DOMReference, x: DOMReference, y: DOMReference) {
    unsafe { webgl_uniform2f(instance, location, x, y) }
}
extern "C" {
    fn webgl_uniform3f(
        instance: DOMReference,
        uniform3f: DOMReference,
        uniform3f: DOMReference,
        uniform3f: DOMReference,
        uniform3f: DOMReference,
    );
}

pub fn uniform3f(
    instance: DOMReference,
    location: DOMReference,
    x: DOMReference,
    y: DOMReference,
    z: DOMReference,
) {
    unsafe { webgl_uniform3f(instance, location, x, y, z) }
}
extern "C" {
    fn webgl_uniform4f(
        instance: DOMReference,
        uniform4f: DOMReference,
        uniform4f: DOMReference,
        uniform4f: DOMReference,
        uniform4f: DOMReference,
        uniform4f: DOMReference,
    );
}

pub fn uniform4f(
    instance: DOMReference,
    location: DOMReference,
    x: DOMReference,
    y: DOMReference,
    z: DOMReference,
    w: DOMReference,
) {
    unsafe { webgl_uniform4f(instance, location, x, y, z, w) }
}
extern "C" {
    fn webgl_uniform1i(instance: DOMReference, uniform1i: DOMReference, uniform1i: DOMReference);
}

pub fn uniform1i(instance: DOMReference, location: DOMReference, x: DOMReference) {
    unsafe { webgl_uniform1i(instance, location, x) }
}
extern "C" {
    fn webgl_uniform2i(
        instance: DOMReference,
        uniform2i: DOMReference,
        uniform2i: DOMReference,
        uniform2i: DOMReference,
    );
}

pub fn uniform2i(instance: DOMReference, location: DOMReference, x: DOMReference, y: DOMReference) {
    unsafe { webgl_uniform2i(instance, location, x, y) }
}
extern "C" {
    fn webgl_uniform3i(
        instance: DOMReference,
        uniform3i: DOMReference,
        uniform3i: DOMReference,
        uniform3i: DOMReference,
        uniform3i: DOMReference,
    );
}

pub fn uniform3i(
    instance: DOMReference,
    location: DOMReference,
    x: DOMReference,
    y: DOMReference,
    z: DOMReference,
) {
    unsafe { webgl_uniform3i(instance, location, x, y, z) }
}
extern "C" {
    fn webgl_uniform4i(
        instance: DOMReference,
        uniform4i: DOMReference,
        uniform4i: DOMReference,
        uniform4i: DOMReference,
        uniform4i: DOMReference,
        uniform4i: DOMReference,
    );
}

pub fn uniform4i(
    instance: DOMReference,
    location: DOMReference,
    x: DOMReference,
    y: DOMReference,
    z: DOMReference,
    w: DOMReference,
) {
    unsafe { webgl_uniform4i(instance, location, x, y, z, w) }
}
extern "C" {
    fn webgl_use_program(instance: DOMReference, use_program: DOMReference);
}

pub fn use_program(instance: DOMReference, program: DOMReference) {
    unsafe { webgl_use_program(instance, program) }
}
extern "C" {
    fn webgl_validate_program(instance: DOMReference, validate_program: DOMReference);
}

pub fn validate_program(instance: DOMReference, program: DOMReference) {
    unsafe { webgl_validate_program(instance, program) }
}
extern "C" {
    fn webgl_vertex_attrib1f(
        instance: DOMReference,
        vertex_attrib1f: DOMReference,
        vertex_attrib1f: DOMReference,
    );
}

pub fn vertex_attrib1f(instance: DOMReference, indx: DOMReference, x: DOMReference) {
    unsafe { webgl_vertex_attrib1f(instance, indx, x) }
}
extern "C" {
    fn webgl_vertex_attrib1fv(
        instance: DOMReference,
        vertex_attrib1fv: DOMReference,
        vertex_attrib1fv: DOMReference,
    );
}

pub fn vertex_attrib1fv(instance: DOMReference, indx: DOMReference, values: DOMReference) {
    unsafe { webgl_vertex_attrib1fv(instance, indx, values) }
}
extern "C" {
    fn webgl_vertex_attrib2f(
        instance: DOMReference,
        vertex_attrib2f: DOMReference,
        vertex_attrib2f: DOMReference,
        vertex_attrib2f: DOMReference,
    );
}

pub fn vertex_attrib2f(
    instance: DOMReference,
    indx: DOMReference,
    x: DOMReference,
    y: DOMReference,
) {
    unsafe { webgl_vertex_attrib2f(instance, indx, x, y) }
}
extern "C" {
    fn webgl_vertex_attrib2fv(
        instance: DOMReference,
        vertex_attrib2fv: DOMReference,
        vertex_attrib2fv: DOMReference,
    );
}

pub fn vertex_attrib2fv(instance: DOMReference, indx: DOMReference, values: DOMReference) {
    unsafe { webgl_vertex_attrib2fv(instance, indx, values) }
}
extern "C" {
    fn webgl_vertex_attrib3f(
        instance: DOMReference,
        vertex_attrib3f: DOMReference,
        vertex_attrib3f: DOMReference,
        vertex_attrib3f: DOMReference,
        vertex_attrib3f: DOMReference,
    );
}

pub fn vertex_attrib3f(
    instance: DOMReference,
    indx: DOMReference,
    x: DOMReference,
    y: DOMReference,
    z: DOMReference,
) {
    unsafe { webgl_vertex_attrib3f(instance, indx, x, y, z) }
}
extern "C" {
    fn webgl_vertex_attrib3fv(
        instance: DOMReference,
        vertex_attrib3fv: DOMReference,
        vertex_attrib3fv: DOMReference,
    );
}

pub fn vertex_attrib3fv(instance: DOMReference, indx: DOMReference, values: DOMReference) {
    unsafe { webgl_vertex_attrib3fv(instance, indx, values) }
}
extern "C" {
    fn webgl_vertex_attrib4f(
        instance: DOMReference,
        vertex_attrib4f: DOMReference,
        vertex_attrib4f: DOMReference,
        vertex_attrib4f: DOMReference,
        vertex_attrib4f: DOMReference,
        vertex_attrib4f: DOMReference,
    );
}

pub fn vertex_attrib4f(
    instance: DOMReference,
    indx: DOMReference,
    x: DOMReference,
    y: DOMReference,
    z: DOMReference,
    w: DOMReference,
) {
    unsafe { webgl_vertex_attrib4f(instance, indx, x, y, z, w) }
}
extern "C" {
    fn webgl_vertex_attrib4fv(
        instance: DOMReference,
        vertex_attrib4fv: DOMReference,
        vertex_attrib4fv: DOMReference,
    );
}

pub fn vertex_attrib4fv(instance: DOMReference, indx: DOMReference, values: DOMReference) {
    unsafe { webgl_vertex_attrib4fv(instance, indx, values) }
}
extern "C" {
    fn webgl_vertex_attrib_pointer(
        instance: DOMReference,
        vertex_attrib_pointer: DOMReference,
        vertex_attrib_pointer: DOMReference,
        vertex_attrib_pointer: DOMReference,
        vertex_attrib_pointer: DOMReference,
        vertex_attrib_pointer: DOMReference,
        vertex_attrib_pointer: DOMReference,
    );
}

pub fn vertex_attrib_pointer(
    instance: DOMReference,
    indx: DOMReference,
    size: DOMReference,
    pointer_type: DOMReference,
    normalized: DOMReference,
    stride: DOMReference,
    offset: DOMReference,
) {
    unsafe {
        webgl_vertex_attrib_pointer(
            instance,
            indx,
            size,
            pointer_type,
            normalized,
            stride,
            offset,
        )
    }
}
extern "C" {
    fn webgl_viewport(
        instance: DOMReference,
        viewport: DOMReference,
        viewport: DOMReference,
        viewport: DOMReference,
        viewport: DOMReference,
    );
}

pub fn viewport(
    instance: DOMReference,
    x: DOMReference,
    y: DOMReference,
    width: DOMReference,
    height: DOMReference,
) {
    unsafe { webgl_viewport(instance, x, y, width, height) }
}
extern "C" {
    fn webgl_buffer_data(
        instance: DOMReference,
        buffer_data: DOMReference,
        buffer_data: DOMReference,
        buffer_data: DOMReference,
    );
}

pub fn buffer_data(
    instance: DOMReference,
    target: DOMReference,
    size: DOMReference,
    usage: DOMReference,
) {
    unsafe { webgl_buffer_data(instance, target, size, usage) }
}
extern "C" {
    fn webgl_buffer_data_1(
        instance: DOMReference,
        buffer_data_1: DOMReference,
        buffer_data_1: DOMReference,
        buffer_data_1: DOMReference,
    );
}

pub fn buffer_data_1(
    instance: DOMReference,
    target: DOMReference,
    data: DOMReference,
    usage: DOMReference,
) {
    unsafe { webgl_buffer_data_1(instance, target, data, usage) }
}
extern "C" {
    fn webgl_buffer_data_2(
        instance: DOMReference,
        buffer_data_2: DOMReference,
        buffer_data_2: DOMReference,
        buffer_data_2: DOMReference,
    );
}

pub fn buffer_data_2(
    instance: DOMReference,
    target: DOMReference,
    data: DOMReference,
    usage: DOMReference,
) {
    unsafe { webgl_buffer_data_2(instance, target, data, usage) }
}
extern "C" {
    fn webgl_buffer_sub_data(
        instance: DOMReference,
        buffer_sub_data: DOMReference,
        buffer_sub_data: DOMReference,
        buffer_sub_data: DOMReference,
    );
}

pub fn buffer_sub_data(
    instance: DOMReference,
    target: DOMReference,
    offset: DOMReference,
    data: DOMReference,
) {
    unsafe { webgl_buffer_sub_data(instance, target, offset, data) }
}
extern "C" {
    fn webgl_buffer_sub_data_1(
        instance: DOMReference,
        buffer_sub_data_1: DOMReference,
        buffer_sub_data_1: DOMReference,
        buffer_sub_data_1: DOMReference,
    );
}

pub fn buffer_sub_data_1(
    instance: DOMReference,
    target: DOMReference,
    offset: DOMReference,
    data: DOMReference,
) {
    unsafe { webgl_buffer_sub_data_1(instance, target, offset, data) }
}
extern "C" {
    fn webgl_compressed_tex_image2_d(
        instance: DOMReference,
        compressed_tex_image2_d: DOMReference,
        compressed_tex_image2_d: DOMReference,
        compressed_tex_image2_d: DOMReference,
        compressed_tex_image2_d: DOMReference,
        compressed_tex_image2_d: DOMReference,
        compressed_tex_image2_d: DOMReference,
        compressed_tex_image2_d: DOMReference,
    );
}

pub fn compressed_tex_image2_d(
    instance: DOMReference,
    target: DOMReference,
    level: DOMReference,
    internalformat: DOMReference,
    width: DOMReference,
    height: DOMReference,
    border: DOMReference,
    data: DOMReference,
) {
    unsafe {
        webgl_compressed_tex_image2_d(
            instance,
            target,
            level,
            internalformat,
            width,
            height,
            border,
            data,
        )
    }
}
extern "C" {
    fn webgl_compressed_tex_sub_image2_d(
        instance: DOMReference,
        compressed_tex_sub_image2_d: DOMReference,
        compressed_tex_sub_image2_d: DOMReference,
        compressed_tex_sub_image2_d: DOMReference,
        compressed_tex_sub_image2_d: DOMReference,
        compressed_tex_sub_image2_d: DOMReference,
        compressed_tex_sub_image2_d: DOMReference,
        compressed_tex_sub_image2_d: DOMReference,
        compressed_tex_sub_image2_d: DOMReference,
    );
}

pub fn compressed_tex_sub_image2_d(
    instance: DOMReference,
    target: DOMReference,
    level: DOMReference,
    xoffset: DOMReference,
    yoffset: DOMReference,
    width: DOMReference,
    height: DOMReference,
    format: DOMReference,
    data: DOMReference,
) {
    unsafe {
        webgl_compressed_tex_sub_image2_d(
            instance, target, level, xoffset, yoffset, width, height, format, data,
        )
    }
}
extern "C" {
    fn webgl_read_pixels(
        instance: DOMReference,
        read_pixels: DOMReference,
        read_pixels: DOMReference,
        read_pixels: DOMReference,
        read_pixels: DOMReference,
        read_pixels: DOMReference,
        read_pixels: DOMReference,
        read_pixels: DOMReference,
    );
}

pub fn read_pixels(
    instance: DOMReference,
    x: DOMReference,
    y: DOMReference,
    width: DOMReference,
    height: DOMReference,
    format: DOMReference,
    pixel_type: DOMReference,
    pixels: DOMReference,
) {
    unsafe { webgl_read_pixels(instance, x, y, width, height, format, pixel_type, pixels) }
}
extern "C" {
    fn webgl_tex_image2_d(
        instance: DOMReference,
        tex_image2_d: DOMReference,
        tex_image2_d: DOMReference,
        tex_image2_d: DOMReference,
        tex_image2_d: DOMReference,
        tex_image2_d: DOMReference,
        tex_image2_d: DOMReference,
        tex_image2_d: DOMReference,
        tex_image2_d: DOMReference,
        tex_image2_d: DOMReference,
    );
}

pub fn tex_image2_d(
    instance: DOMReference,
    target: DOMReference,
    level: DOMReference,
    internalformat: DOMReference,
    width: DOMReference,
    height: DOMReference,
    border: DOMReference,
    format: DOMReference,
    image_type: DOMReference,
    pixels: DOMReference,
) {
    unsafe {
        webgl_tex_image2_d(
            instance,
            target,
            level,
            internalformat,
            width,
            height,
            border,
            format,
            image_type,
            pixels,
        )
    }
}
extern "C" {
    fn webgl_tex_image2_d_1(
        instance: DOMReference,
        tex_image2_d_1: DOMReference,
        tex_image2_d_1: DOMReference,
        tex_image2_d_1: DOMReference,
        tex_image2_d_1: DOMReference,
        tex_image2_d_1: DOMReference,
        tex_image2_d_1: DOMReference,
    );
}

pub fn tex_image2_d_1(
    instance: DOMReference,
    target: DOMReference,
    level: DOMReference,
    internalformat: DOMReference,
    format: DOMReference,
    image_type: DOMReference,
    pixels: DOMReference,
) {
    unsafe {
        webgl_tex_image2_d_1(
            instance,
            target,
            level,
            internalformat,
            format,
            image_type,
            pixels,
        )
    }
}
extern "C" {
    fn webgl_tex_image2_d_2(
        instance: DOMReference,
        tex_image2_d_2: DOMReference,
        tex_image2_d_2: DOMReference,
        tex_image2_d_2: DOMReference,
        tex_image2_d_2: DOMReference,
        tex_image2_d_2: DOMReference,
        tex_image2_d_2: DOMReference,
    );
}

pub fn tex_image2_d_2(
    instance: DOMReference,
    target: DOMReference,
    level: DOMReference,
    internalformat: DOMReference,
    format: DOMReference,
    image_type: DOMReference,
    pixels: DOMReference,
) {
    unsafe {
        webgl_tex_image2_d_2(
            instance,
            target,
            level,
            internalformat,
            format,
            image_type,
            pixels,
        )
    }
}
extern "C" {
    fn webgl_tex_image2_d_3(
        instance: DOMReference,
        tex_image2_d_3: DOMReference,
        tex_image2_d_3: DOMReference,
        tex_image2_d_3: DOMReference,
        tex_image2_d_3: DOMReference,
        tex_image2_d_3: DOMReference,
        tex_image2_d_3: DOMReference,
    );
}

pub fn tex_image2_d_3(
    instance: DOMReference,
    target: DOMReference,
    level: DOMReference,
    internalformat: DOMReference,
    format: DOMReference,
    image_type: DOMReference,
    image: DOMReference,
) {
    unsafe {
        webgl_tex_image2_d_3(
            instance,
            target,
            level,
            internalformat,
            format,
            image_type,
            image,
        )
    }
}
extern "C" {
    fn webgl_tex_image2_d_4(
        instance: DOMReference,
        tex_image2_d_4: DOMReference,
        tex_image2_d_4: DOMReference,
        tex_image2_d_4: DOMReference,
        tex_image2_d_4: DOMReference,
        tex_image2_d_4: DOMReference,
        tex_image2_d_4: DOMReference,
    );
}

pub fn tex_image2_d_4(
    instance: DOMReference,
    target: DOMReference,
    level: DOMReference,
    internalformat: DOMReference,
    format: DOMReference,
    image_type: DOMReference,
    canvas: DOMReference,
) {
    unsafe {
        webgl_tex_image2_d_4(
            instance,
            target,
            level,
            internalformat,
            format,
            image_type,
            canvas,
        )
    }
}
extern "C" {
    fn webgl_tex_image2_d_5(
        instance: DOMReference,
        tex_image2_d_5: DOMReference,
        tex_image2_d_5: DOMReference,
        tex_image2_d_5: DOMReference,
        tex_image2_d_5: DOMReference,
        tex_image2_d_5: DOMReference,
        tex_image2_d_5: DOMReference,
    );
}

pub fn tex_image2_d_5(
    instance: DOMReference,
    target: DOMReference,
    level: DOMReference,
    internalformat: DOMReference,
    format: DOMReference,
    image_type: DOMReference,
    video: DOMReference,
) {
    unsafe {
        webgl_tex_image2_d_5(
            instance,
            target,
            level,
            internalformat,
            format,
            image_type,
            video,
        )
    }
}
extern "C" {
    fn webgl_tex_sub_image2_d(
        instance: DOMReference,
        tex_sub_image2_d: DOMReference,
        tex_sub_image2_d: DOMReference,
        tex_sub_image2_d: DOMReference,
        tex_sub_image2_d: DOMReference,
        tex_sub_image2_d: DOMReference,
        tex_sub_image2_d: DOMReference,
        tex_sub_image2_d: DOMReference,
        tex_sub_image2_d: DOMReference,
        tex_sub_image2_d: DOMReference,
    );
}

pub fn tex_sub_image2_d(
    instance: DOMReference,
    target: DOMReference,
    level: DOMReference,
    xoffset: DOMReference,
    yoffset: DOMReference,
    width: DOMReference,
    height: DOMReference,
    format: DOMReference,
    image_type: DOMReference,
    pixels: DOMReference,
) {
    unsafe {
        webgl_tex_sub_image2_d(
            instance, target, level, xoffset, yoffset, width, height, format, image_type, pixels,
        )
    }
}
extern "C" {
    fn webgl_tex_sub_image2_d_1(
        instance: DOMReference,
        tex_sub_image2_d_1: DOMReference,
        tex_sub_image2_d_1: DOMReference,
        tex_sub_image2_d_1: DOMReference,
        tex_sub_image2_d_1: DOMReference,
        tex_sub_image2_d_1: DOMReference,
        tex_sub_image2_d_1: DOMReference,
        tex_sub_image2_d_1: DOMReference,
    );
}

pub fn tex_sub_image2_d_1(
    instance: DOMReference,
    target: DOMReference,
    level: DOMReference,
    xoffset: DOMReference,
    yoffset: DOMReference,
    format: DOMReference,
    image_type: DOMReference,
    pixels: DOMReference,
) {
    unsafe {
        webgl_tex_sub_image2_d_1(
            instance, target, level, xoffset, yoffset, format, image_type, pixels,
        )
    }
}
extern "C" {
    fn webgl_tex_sub_image2_d_2(
        instance: DOMReference,
        tex_sub_image2_d_2: DOMReference,
        tex_sub_image2_d_2: DOMReference,
        tex_sub_image2_d_2: DOMReference,
        tex_sub_image2_d_2: DOMReference,
        tex_sub_image2_d_2: DOMReference,
        tex_sub_image2_d_2: DOMReference,
        tex_sub_image2_d_2: DOMReference,
    );
}

pub fn tex_sub_image2_d_2(
    instance: DOMReference,
    target: DOMReference,
    level: DOMReference,
    xoffset: DOMReference,
    yoffset: DOMReference,
    format: DOMReference,
    image_type: DOMReference,
    pixels: DOMReference,
) {
    unsafe {
        webgl_tex_sub_image2_d_2(
            instance, target, level, xoffset, yoffset, format, image_type, pixels,
        )
    }
}
extern "C" {
    fn webgl_tex_sub_image2_d_3(
        instance: DOMReference,
        tex_sub_image2_d_3: DOMReference,
        tex_sub_image2_d_3: DOMReference,
        tex_sub_image2_d_3: DOMReference,
        tex_sub_image2_d_3: DOMReference,
        tex_sub_image2_d_3: DOMReference,
        tex_sub_image2_d_3: DOMReference,
        tex_sub_image2_d_3: DOMReference,
    );
}

pub fn tex_sub_image2_d_3(
    instance: DOMReference,
    target: DOMReference,
    level: DOMReference,
    xoffset: DOMReference,
    yoffset: DOMReference,
    format: DOMReference,
    image_type: DOMReference,
    image: DOMReference,
) {
    unsafe {
        webgl_tex_sub_image2_d_3(
            instance, target, level, xoffset, yoffset, format, image_type, image,
        )
    }
}
extern "C" {
    fn webgl_tex_sub_image2_d_4(
        instance: DOMReference,
        tex_sub_image2_d_4: DOMReference,
        tex_sub_image2_d_4: DOMReference,
        tex_sub_image2_d_4: DOMReference,
        tex_sub_image2_d_4: DOMReference,
        tex_sub_image2_d_4: DOMReference,
        tex_sub_image2_d_4: DOMReference,
        tex_sub_image2_d_4: DOMReference,
    );
}

pub fn tex_sub_image2_d_4(
    instance: DOMReference,
    target: DOMReference,
    level: DOMReference,
    xoffset: DOMReference,
    yoffset: DOMReference,
    format: DOMReference,
    image_type: DOMReference,
    canvas: DOMReference,
) {
    unsafe {
        webgl_tex_sub_image2_d_4(
            instance, target, level, xoffset, yoffset, format, image_type, canvas,
        )
    }
}
extern "C" {
    fn webgl_tex_sub_image2_d_5(
        instance: DOMReference,
        tex_sub_image2_d_5: DOMReference,
        tex_sub_image2_d_5: DOMReference,
        tex_sub_image2_d_5: DOMReference,
        tex_sub_image2_d_5: DOMReference,
        tex_sub_image2_d_5: DOMReference,
        tex_sub_image2_d_5: DOMReference,
        tex_sub_image2_d_5: DOMReference,
    );
}

pub fn tex_sub_image2_d_5(
    instance: DOMReference,
    target: DOMReference,
    level: DOMReference,
    xoffset: DOMReference,
    yoffset: DOMReference,
    format: DOMReference,
    image_type: DOMReference,
    video: DOMReference,
) {
    unsafe {
        webgl_tex_sub_image2_d_5(
            instance, target, level, xoffset, yoffset, format, image_type, video,
        )
    }
}
extern "C" {
    fn webgl_uniform1fv(instance: DOMReference, uniform1fv: DOMReference, uniform1fv: DOMReference);
}

pub fn uniform1fv(instance: DOMReference, location: DOMReference, data: DOMReference) {
    unsafe { webgl_uniform1fv(instance, location, data) }
}
extern "C" {
    fn webgl_uniform2fv(instance: DOMReference, uniform2fv: DOMReference, uniform2fv: DOMReference);
}

pub fn uniform2fv(instance: DOMReference, location: DOMReference, data: DOMReference) {
    unsafe { webgl_uniform2fv(instance, location, data) }
}
extern "C" {
    fn webgl_uniform3fv(instance: DOMReference, uniform3fv: DOMReference, uniform3fv: DOMReference);
}

pub fn uniform3fv(instance: DOMReference, location: DOMReference, data: DOMReference) {
    unsafe { webgl_uniform3fv(instance, location, data) }
}
extern "C" {
    fn webgl_uniform4fv(instance: DOMReference, uniform4fv: DOMReference, uniform4fv: DOMReference);
}

pub fn uniform4fv(instance: DOMReference, location: DOMReference, data: DOMReference) {
    unsafe { webgl_uniform4fv(instance, location, data) }
}
extern "C" {
    fn webgl_uniform1iv(instance: DOMReference, uniform1iv: DOMReference, uniform1iv: DOMReference);
}

pub fn uniform1iv(instance: DOMReference, location: DOMReference, data: DOMReference) {
    unsafe { webgl_uniform1iv(instance, location, data) }
}
extern "C" {
    fn webgl_uniform2iv(instance: DOMReference, uniform2iv: DOMReference, uniform2iv: DOMReference);
}

pub fn uniform2iv(instance: DOMReference, location: DOMReference, data: DOMReference) {
    unsafe { webgl_uniform2iv(instance, location, data) }
}
extern "C" {
    fn webgl_uniform3iv(instance: DOMReference, uniform3iv: DOMReference, uniform3iv: DOMReference);
}

pub fn uniform3iv(instance: DOMReference, location: DOMReference, data: DOMReference) {
    unsafe { webgl_uniform3iv(instance, location, data) }
}
extern "C" {
    fn webgl_uniform4iv(instance: DOMReference, uniform4iv: DOMReference, uniform4iv: DOMReference);
}

pub fn uniform4iv(instance: DOMReference, location: DOMReference, data: DOMReference) {
    unsafe { webgl_uniform4iv(instance, location, data) }
}
extern "C" {
    fn webgl_uniform_matrix2fv(
        instance: DOMReference,
        uniform_matrix2fv: DOMReference,
        uniform_matrix2fv: DOMReference,
        uniform_matrix2fv: DOMReference,
    );
}

pub fn uniform_matrix2fv(
    instance: DOMReference,
    location: DOMReference,
    transpose: DOMReference,
    data: DOMReference,
) {
    unsafe { webgl_uniform_matrix2fv(instance, location, transpose, data) }
}
extern "C" {
    fn webgl_uniform_matrix3fv(
        instance: DOMReference,
        uniform_matrix3fv: DOMReference,
        uniform_matrix3fv: DOMReference,
        uniform_matrix3fv: DOMReference,
    );
}

pub fn uniform_matrix3fv(
    instance: DOMReference,
    location: DOMReference,
    transpose: DOMReference,
    data: DOMReference,
) {
    unsafe { webgl_uniform_matrix3fv(instance, location, transpose, data) }
}
extern "C" {
    fn webgl_uniform_matrix4fv(
        instance: DOMReference,
        uniform_matrix4fv: DOMReference,
        uniform_matrix4fv: DOMReference,
        uniform_matrix4fv: DOMReference,
    );
}

pub fn uniform_matrix4fv(
    instance: DOMReference,
    location: DOMReference,
    transpose: DOMReference,
    data: DOMReference,
) {
    unsafe { webgl_uniform_matrix4fv(instance, location, transpose, data) }
}
extern "C" {
    fn webgl_commit(instance: DOMReference);
}

pub fn commit(instance: DOMReference) {
    unsafe { webgl_commit(instance) }
}
