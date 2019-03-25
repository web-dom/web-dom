#[allow(unused_imports)]
use crate::*;
pub const DEPTH_BUFFER_BIT: f32 = 0x00000100 as f32;
pub const STENCIL_BUFFER_BIT: f32 = 0x00000400 as f32;
pub const COLOR_BUFFER_BIT: f32 = 0x00004000 as f32;
pub const POINTS: f32 = 0x0000 as f32;
pub const LINES: f32 = 0x0001 as f32;
pub const LINE_LOOP: f32 = 0x0002 as f32;
pub const LINE_STRIP: f32 = 0x0003 as f32;
pub const TRIANGLES: f32 = 0x0004 as f32;
pub const TRIANGLE_STRIP: f32 = 0x0005 as f32;
pub const TRIANGLE_FAN: f32 = 0x0006 as f32;
pub const ZERO: f32 = 0 as f32;
pub const ONE: f32 = 1 as f32;
pub const SRC_COLOR: f32 = 0x0300 as f32;
pub const ONE_MINUS_SRC_COLOR: f32 = 0x0301 as f32;
pub const SRC_ALPHA: f32 = 0x0302 as f32;
pub const ONE_MINUS_SRC_ALPHA: f32 = 0x0303 as f32;
pub const DST_ALPHA: f32 = 0x0304 as f32;
pub const ONE_MINUS_DST_ALPHA: f32 = 0x0305 as f32;
pub const DST_COLOR: f32 = 0x0306 as f32;
pub const ONE_MINUS_DST_COLOR: f32 = 0x0307 as f32;
pub const SRC_ALPHA_SATURATE: f32 = 0x0308 as f32;
pub const FUNC_ADD: f32 = 0x8006 as f32;
pub const BLEND_EQUATION: f32 = 0x8009 as f32;
pub const BLEND_EQUATION_RGB: f32 = 0x8009 as f32;
pub const BLEND_EQUATION_ALPHA: f32 = 0x883D as f32;
pub const FUNC_SUBTRACT: f32 = 0x800A as f32;
pub const FUNC_REVERSE_SUBTRACT: f32 = 0x800B as f32;
pub const BLEND_DST_RGB: f32 = 0x80C8 as f32;
pub const BLEND_SRC_RGB: f32 = 0x80C9 as f32;
pub const BLEND_DST_ALPHA: f32 = 0x80CA as f32;
pub const BLEND_SRC_ALPHA: f32 = 0x80CB as f32;
pub const CONSTANT_COLOR: f32 = 0x8001 as f32;
pub const ONE_MINUS_CONSTANT_COLOR: f32 = 0x8002 as f32;
pub const CONSTANT_ALPHA: f32 = 0x8003 as f32;
pub const ONE_MINUS_CONSTANT_ALPHA: f32 = 0x8004 as f32;
pub const BLEND_COLOR: f32 = 0x8005 as f32;
pub const ARRAY_BUFFER: f32 = 0x8892 as f32;
pub const ELEMENT_ARRAY_BUFFER: f32 = 0x8893 as f32;
pub const ARRAY_BUFFER_BINDING: f32 = 0x8894 as f32;
pub const ELEMENT_ARRAY_BUFFER_BINDING: f32 = 0x8895 as f32;
pub const STREAM_DRAW: f32 = 0x88E0 as f32;
pub const STATIC_DRAW: f32 = 0x88E4 as f32;
pub const DYNAMIC_DRAW: f32 = 0x88E8 as f32;
pub const BUFFER_SIZE: f32 = 0x8764 as f32;
pub const BUFFER_USAGE: f32 = 0x8765 as f32;
pub const CURRENT_VERTEX_ATTRIB: f32 = 0x8626 as f32;
pub const FRONT: f32 = 0x0404 as f32;
pub const BACK: f32 = 0x0405 as f32;
pub const FRONT_AND_BACK: f32 = 0x0408 as f32;
pub const CULL_FACE: f32 = 0x0B44 as f32;
pub const BLEND: f32 = 0x0BE2 as f32;
pub const DITHER: f32 = 0x0BD0 as f32;
pub const STENCIL_TEST: f32 = 0x0B90 as f32;
pub const DEPTH_TEST: f32 = 0x0B71 as f32;
pub const SCISSOR_TEST: f32 = 0x0C11 as f32;
pub const POLYGON_OFFSET_FILL: f32 = 0x8037 as f32;
pub const SAMPLE_ALPHA_TO_COVERAGE: f32 = 0x809E as f32;
pub const SAMPLE_COVERAGE: f32 = 0x80A0 as f32;
pub const NO_ERROR: f32 = 0 as f32;
pub const INVALID_ENUM: f32 = 0x0500 as f32;
pub const INVALID_VALUE: f32 = 0x0501 as f32;
pub const INVALID_OPERATION: f32 = 0x0502 as f32;
pub const OUT_OF_MEMORY: f32 = 0x0505 as f32;
pub const CW: f32 = 0x0900 as f32;
pub const CCW: f32 = 0x0901 as f32;
pub const LINE_WIDTH: f32 = 0x0B21 as f32;
pub const ALIASED_POINT_SIZE_RANGE: f32 = 0x846D as f32;
pub const ALIASED_LINE_WIDTH_RANGE: f32 = 0x846E as f32;
pub const CULL_FACE_MODE: f32 = 0x0B45 as f32;
pub const FRONT_FACE: f32 = 0x0B46 as f32;
pub const DEPTH_RANGE: f32 = 0x0B70 as f32;
pub const DEPTH_WRITEMASK: f32 = 0x0B72 as f32;
pub const DEPTH_CLEAR_VALUE: f32 = 0x0B73 as f32;
pub const DEPTH_FUNC: f32 = 0x0B74 as f32;
pub const STENCIL_CLEAR_VALUE: f32 = 0x0B91 as f32;
pub const STENCIL_FUNC: f32 = 0x0B92 as f32;
pub const STENCIL_FAIL: f32 = 0x0B94 as f32;
pub const STENCIL_PASS_DEPTH_FAIL: f32 = 0x0B95 as f32;
pub const STENCIL_PASS_DEPTH_PASS: f32 = 0x0B96 as f32;
pub const STENCIL_REF: f32 = 0x0B97 as f32;
pub const STENCIL_VALUE_MASK: f32 = 0x0B93 as f32;
pub const STENCIL_WRITEMASK: f32 = 0x0B98 as f32;
pub const STENCIL_BACK_FUNC: f32 = 0x8800 as f32;
pub const STENCIL_BACK_FAIL: f32 = 0x8801 as f32;
pub const STENCIL_BACK_PASS_DEPTH_FAIL: f32 = 0x8802 as f32;
pub const STENCIL_BACK_PASS_DEPTH_PASS: f32 = 0x8803 as f32;
pub const STENCIL_BACK_REF: f32 = 0x8CA3 as f32;
pub const STENCIL_BACK_VALUE_MASK: f32 = 0x8CA4 as f32;
pub const STENCIL_BACK_WRITEMASK: f32 = 0x8CA5 as f32;
pub const VIEWPORT: f32 = 0x0BA2 as f32;
pub const SCISSOR_BOX: f32 = 0x0C10 as f32;
pub const COLOR_CLEAR_VALUE: f32 = 0x0C22 as f32;
pub const COLOR_WRITEMASK: f32 = 0x0C23 as f32;
pub const UNPACK_ALIGNMENT: f32 = 0x0CF5 as f32;
pub const PACK_ALIGNMENT: f32 = 0x0D05 as f32;
pub const MAX_TEXTURE_SIZE: f32 = 0x0D33 as f32;
pub const MAX_VIEWPORT_DIMS: f32 = 0x0D3A as f32;
pub const SUBPIXEL_BITS: f32 = 0x0D50 as f32;
pub const RED_BITS: f32 = 0x0D52 as f32;
pub const GREEN_BITS: f32 = 0x0D53 as f32;
pub const BLUE_BITS: f32 = 0x0D54 as f32;
pub const ALPHA_BITS: f32 = 0x0D55 as f32;
pub const DEPTH_BITS: f32 = 0x0D56 as f32;
pub const STENCIL_BITS: f32 = 0x0D57 as f32;
pub const POLYGON_OFFSET_UNITS: f32 = 0x2A00 as f32;
pub const POLYGON_OFFSET_FACTOR: f32 = 0x8038 as f32;
pub const TEXTURE_BINDING_2D: f32 = 0x8069 as f32;
pub const SAMPLE_BUFFERS: f32 = 0x80A8 as f32;
pub const SAMPLES: f32 = 0x80A9 as f32;
pub const SAMPLE_COVERAGE_VALUE: f32 = 0x80AA as f32;
pub const SAMPLE_COVERAGE_INVERT: f32 = 0x80AB as f32;
pub const COMPRESSED_TEXTURE_FORMATS: f32 = 0x86A3 as f32;
pub const DONT_CARE: f32 = 0x1100 as f32;
pub const FASTEST: f32 = 0x1101 as f32;
pub const NICEST: f32 = 0x1102 as f32;
pub const GENERATE_MIPMAP_HINT: f32 = 0x8192 as f32;
pub const BYTE: f32 = 0x1400 as f32;
pub const UNSIGNED_BYTE: f32 = 0x1401 as f32;
pub const SHORT: f32 = 0x1402 as f32;
pub const UNSIGNED_SHORT: f32 = 0x1403 as f32;
pub const INT: f32 = 0x1404 as f32;
pub const UNSIGNED_INT: f32 = 0x1405 as f32;
pub const FLOAT: f32 = 0x1406 as f32;
pub const DEPTH_COMPONENT: f32 = 0x1902 as f32;
pub const ALPHA: f32 = 0x1906 as f32;
pub const RGB: f32 = 0x1907 as f32;
pub const RGBA: f32 = 0x1908 as f32;
pub const LUMINANCE: f32 = 0x1909 as f32;
pub const LUMINANCE_ALPHA: f32 = 0x190A as f32;
pub const UNSIGNED_SHORT_4_4_4_4: f32 = 0x8033 as f32;
pub const UNSIGNED_SHORT_5_5_5_1: f32 = 0x8034 as f32;
pub const UNSIGNED_SHORT_5_6_5: f32 = 0x8363 as f32;
pub const FRAGMENT_SHADER: f32 = 0x8B30 as f32;
pub const VERTEX_SHADER: f32 = 0x8B31 as f32;
pub const MAX_VERTEX_ATTRIBS: f32 = 0x8869 as f32;
pub const MAX_VERTEX_UNIFORM_VECTORS: f32 = 0x8DFB as f32;
pub const MAX_VARYING_VECTORS: f32 = 0x8DFC as f32;
pub const MAX_COMBINED_TEXTURE_IMAGE_UNITS: f32 = 0x8B4D as f32;
pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS: f32 = 0x8B4C as f32;
pub const MAX_TEXTURE_IMAGE_UNITS: f32 = 0x8872 as f32;
pub const MAX_FRAGMENT_UNIFORM_VECTORS: f32 = 0x8DFD as f32;
pub const SHADER_TYPE: f32 = 0x8B4F as f32;
pub const DELETE_STATUS: f32 = 0x8B80 as f32;
pub const LINK_STATUS: f32 = 0x8B82 as f32;
pub const VALIDATE_STATUS: f32 = 0x8B83 as f32;
pub const ATTACHED_SHADERS: f32 = 0x8B85 as f32;
pub const ACTIVE_UNIFORMS: f32 = 0x8B86 as f32;
pub const ACTIVE_ATTRIBUTES: f32 = 0x8B89 as f32;
pub const SHADING_LANGUAGE_VERSION: f32 = 0x8B8C as f32;
pub const CURRENT_PROGRAM: f32 = 0x8B8D as f32;
pub const NEVER: f32 = 0x0200 as f32;
pub const LESS: f32 = 0x0201 as f32;
pub const EQUAL: f32 = 0x0202 as f32;
pub const LEQUAL: f32 = 0x0203 as f32;
pub const GREATER: f32 = 0x0204 as f32;
pub const NOTEQUAL: f32 = 0x0205 as f32;
pub const GEQUAL: f32 = 0x0206 as f32;
pub const ALWAYS: f32 = 0x0207 as f32;
pub const KEEP: f32 = 0x1E00 as f32;
pub const REPLACE: f32 = 0x1E01 as f32;
pub const INCR: f32 = 0x1E02 as f32;
pub const DECR: f32 = 0x1E03 as f32;
pub const INVERT: f32 = 0x150A as f32;
pub const INCR_WRAP: f32 = 0x8507 as f32;
pub const DECR_WRAP: f32 = 0x8508 as f32;
pub const VENDOR: f32 = 0x1F00 as f32;
pub const RENDERER: f32 = 0x1F01 as f32;
pub const VERSION: f32 = 0x1F02 as f32;
pub const NEAREST: f32 = 0x2600 as f32;
pub const LINEAR: f32 = 0x2601 as f32;
pub const NEAREST_MIPMAP_NEAREST: f32 = 0x2700 as f32;
pub const LINEAR_MIPMAP_NEAREST: f32 = 0x2701 as f32;
pub const NEAREST_MIPMAP_LINEAR: f32 = 0x2702 as f32;
pub const LINEAR_MIPMAP_LINEAR: f32 = 0x2703 as f32;
pub const TEXTURE_MAG_FILTER: f32 = 0x2800 as f32;
pub const TEXTURE_MIN_FILTER: f32 = 0x2801 as f32;
pub const TEXTURE_WRAP_S: f32 = 0x2802 as f32;
pub const TEXTURE_WRAP_T: f32 = 0x2803 as f32;
pub const TEXTURE_2D: f32 = 0x0DE1 as f32;
pub const TEXTURE: f32 = 0x1702 as f32;
pub const TEXTURE_CUBE_MAP: f32 = 0x8513 as f32;
pub const TEXTURE_BINDING_CUBE_MAP: f32 = 0x8514 as f32;
pub const TEXTURE_CUBE_MAP_POSITIVE_X: f32 = 0x8515 as f32;
pub const TEXTURE_CUBE_MAP_NEGATIVE_X: f32 = 0x8516 as f32;
pub const TEXTURE_CUBE_MAP_POSITIVE_Y: f32 = 0x8517 as f32;
pub const TEXTURE_CUBE_MAP_NEGATIVE_Y: f32 = 0x8518 as f32;
pub const TEXTURE_CUBE_MAP_POSITIVE_Z: f32 = 0x8519 as f32;
pub const TEXTURE_CUBE_MAP_NEGATIVE_Z: f32 = 0x851A as f32;
pub const MAX_CUBE_MAP_TEXTURE_SIZE: f32 = 0x851C as f32;
pub const TEXTURE0: f32 = 0x84C0 as f32;
pub const TEXTURE1: f32 = 0x84C1 as f32;
pub const TEXTURE2: f32 = 0x84C2 as f32;
pub const TEXTURE3: f32 = 0x84C3 as f32;
pub const TEXTURE4: f32 = 0x84C4 as f32;
pub const TEXTURE5: f32 = 0x84C5 as f32;
pub const TEXTURE6: f32 = 0x84C6 as f32;
pub const TEXTURE7: f32 = 0x84C7 as f32;
pub const TEXTURE8: f32 = 0x84C8 as f32;
pub const TEXTURE9: f32 = 0x84C9 as f32;
pub const TEXTURE10: f32 = 0x84CA as f32;
pub const TEXTURE11: f32 = 0x84CB as f32;
pub const TEXTURE12: f32 = 0x84CC as f32;
pub const TEXTURE13: f32 = 0x84CD as f32;
pub const TEXTURE14: f32 = 0x84CE as f32;
pub const TEXTURE15: f32 = 0x84CF as f32;
pub const TEXTURE16: f32 = 0x84D0 as f32;
pub const TEXTURE17: f32 = 0x84D1 as f32;
pub const TEXTURE18: f32 = 0x84D2 as f32;
pub const TEXTURE19: f32 = 0x84D3 as f32;
pub const TEXTURE20: f32 = 0x84D4 as f32;
pub const TEXTURE21: f32 = 0x84D5 as f32;
pub const TEXTURE22: f32 = 0x84D6 as f32;
pub const TEXTURE23: f32 = 0x84D7 as f32;
pub const TEXTURE24: f32 = 0x84D8 as f32;
pub const TEXTURE25: f32 = 0x84D9 as f32;
pub const TEXTURE26: f32 = 0x84DA as f32;
pub const TEXTURE27: f32 = 0x84DB as f32;
pub const TEXTURE28: f32 = 0x84DC as f32;
pub const TEXTURE29: f32 = 0x84DD as f32;
pub const TEXTURE30: f32 = 0x84DE as f32;
pub const TEXTURE31: f32 = 0x84DF as f32;
pub const ACTIVE_TEXTURE: f32 = 0x84E0 as f32;
pub const REPEAT: f32 = 0x2901 as f32;
pub const CLAMP_TO_EDGE: f32 = 0x812F as f32;
pub const MIRRORED_REPEAT: f32 = 0x8370 as f32;
pub const FLOAT_VEC2: f32 = 0x8B50 as f32;
pub const FLOAT_VEC3: f32 = 0x8B51 as f32;
pub const FLOAT_VEC4: f32 = 0x8B52 as f32;
pub const INT_VEC2: f32 = 0x8B53 as f32;
pub const INT_VEC3: f32 = 0x8B54 as f32;
pub const INT_VEC4: f32 = 0x8B55 as f32;
pub const BOOL: f32 = 0x8B56 as f32;
pub const BOOL_VEC2: f32 = 0x8B57 as f32;
pub const BOOL_VEC3: f32 = 0x8B58 as f32;
pub const BOOL_VEC4: f32 = 0x8B59 as f32;
pub const FLOAT_MAT2: f32 = 0x8B5A as f32;
pub const FLOAT_MAT3: f32 = 0x8B5B as f32;
pub const FLOAT_MAT4: f32 = 0x8B5C as f32;
pub const SAMPLER_2D: f32 = 0x8B5E as f32;
pub const SAMPLER_CUBE: f32 = 0x8B60 as f32;
pub const VERTEX_ATTRIB_ARRAY_ENABLED: f32 = 0x8622 as f32;
pub const VERTEX_ATTRIB_ARRAY_SIZE: f32 = 0x8623 as f32;
pub const VERTEX_ATTRIB_ARRAY_STRIDE: f32 = 0x8624 as f32;
pub const VERTEX_ATTRIB_ARRAY_TYPE: f32 = 0x8625 as f32;
pub const VERTEX_ATTRIB_ARRAY_NORMALIZED: f32 = 0x886A as f32;
pub const VERTEX_ATTRIB_ARRAY_POINTER: f32 = 0x8645 as f32;
pub const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: f32 = 0x889F as f32;
pub const IMPLEMENTATION_COLOR_READ_TYPE: f32 = 0x8B9A as f32;
pub const IMPLEMENTATION_COLOR_READ_FORMAT: f32 = 0x8B9B as f32;
pub const COMPILE_STATUS: f32 = 0x8B81 as f32;
pub const LOW_FLOAT: f32 = 0x8DF0 as f32;
pub const MEDIUM_FLOAT: f32 = 0x8DF1 as f32;
pub const HIGH_FLOAT: f32 = 0x8DF2 as f32;
pub const LOW_INT: f32 = 0x8DF3 as f32;
pub const MEDIUM_INT: f32 = 0x8DF4 as f32;
pub const HIGH_INT: f32 = 0x8DF5 as f32;
pub const FRAMEBUFFER: f32 = 0x8D40 as f32;
pub const RENDERBUFFER: f32 = 0x8D41 as f32;
pub const RGBA4: f32 = 0x8056 as f32;
pub const RGB5_A1: f32 = 0x8057 as f32;
pub const RGB565: f32 = 0x8D62 as f32;
pub const DEPTH_COMPONENT16: f32 = 0x81A5 as f32;
pub const STENCIL_INDEX8: f32 = 0x8D48 as f32;
pub const DEPTH_STENCIL: f32 = 0x84F9 as f32;
pub const RENDERBUFFER_WIDTH: f32 = 0x8D42 as f32;
pub const RENDERBUFFER_HEIGHT: f32 = 0x8D43 as f32;
pub const RENDERBUFFER_INTERNAL_FORMAT: f32 = 0x8D44 as f32;
pub const RENDERBUFFER_RED_SIZE: f32 = 0x8D50 as f32;
pub const RENDERBUFFER_GREEN_SIZE: f32 = 0x8D51 as f32;
pub const RENDERBUFFER_BLUE_SIZE: f32 = 0x8D52 as f32;
pub const RENDERBUFFER_ALPHA_SIZE: f32 = 0x8D53 as f32;
pub const RENDERBUFFER_DEPTH_SIZE: f32 = 0x8D54 as f32;
pub const RENDERBUFFER_STENCIL_SIZE: f32 = 0x8D55 as f32;
pub const FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: f32 = 0x8CD0 as f32;
pub const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: f32 = 0x8CD1 as f32;
pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: f32 = 0x8CD2 as f32;
pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: f32 = 0x8CD3 as f32;
pub const COLOR_ATTACHMENT0: f32 = 0x8CE0 as f32;
pub const DEPTH_ATTACHMENT: f32 = 0x8D00 as f32;
pub const STENCIL_ATTACHMENT: f32 = 0x8D20 as f32;
pub const DEPTH_STENCIL_ATTACHMENT: f32 = 0x821A as f32;
pub const NONE: f32 = 0 as f32;
pub const FRAMEBUFFER_COMPLETE: f32 = 0x8CD5 as f32;
pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT: f32 = 0x8CD6 as f32;
pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: f32 = 0x8CD7 as f32;
pub const FRAMEBUFFER_INCOMPLETE_DIMENSIONS: f32 = 0x8CD9 as f32;
pub const FRAMEBUFFER_UNSUPPORTED: f32 = 0x8CDD as f32;
pub const FRAMEBUFFER_BINDING: f32 = 0x8CA6 as f32;
pub const RENDERBUFFER_BINDING: f32 = 0x8CA7 as f32;
pub const MAX_RENDERBUFFER_SIZE: f32 = 0x84E8 as f32;
pub const INVALID_FRAMEBUFFER_OPERATION: f32 = 0x0506 as f32;
pub const UNPACK_FLIP_Y_WEBGL: f32 = 0x9240 as f32;
pub const UNPACK_PREMULTIPLY_ALPHA_WEBGL: f32 = 0x9241 as f32;
pub const CONTEXT_LOST_WEBGL: f32 = 0x9242 as f32;
pub const UNPACK_COLORSPACE_CONVERSION_WEBGL: f32 = 0x9243 as f32;
pub const BROWSER_DEFAULT_WEBGL: f32 = 0x9244 as f32;
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
    fn webgl_get_drawing_buffer_width(instance: DOMReference) -> f32;
    fn webgl_set_drawing_buffer_width(instance: DOMReference, value: f32);
}

pub fn get_drawing_buffer_width(instance: DOMReference) -> f32 {
    unsafe { webgl_get_drawing_buffer_width(instance) }
}

pub fn set_drawing_buffer_width(instance: DOMReference, value: f32) {
    unsafe {
        webgl_set_drawing_buffer_width(instance, value);
    }
}
extern "C" {
    fn webgl_get_drawing_buffer_height(instance: DOMReference) -> f32;
    fn webgl_set_drawing_buffer_height(instance: DOMReference, value: f32);
}

pub fn get_drawing_buffer_height(instance: DOMReference) -> f32 {
    unsafe { webgl_get_drawing_buffer_height(instance) }
}

pub fn set_drawing_buffer_height(instance: DOMReference, value: f32) {
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
    fn webgl_active_texture(instance: DOMReference, active_texture: f32);
}

pub fn active_texture(instance: DOMReference, texture: f32) {
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
        bind_attrib_location: f32,
        bind_attrib_location: CString,
    );
}

pub fn bind_attrib_location(instance: DOMReference, program: DOMReference, index: f32, name: &str) {
    unsafe { webgl_bind_attrib_location(instance, program, index, to_cstring(name)) }
}
extern "C" {
    fn webgl_bind_buffer(instance: DOMReference, bind_buffer: f32, bind_buffer: DOMReference);
}

pub fn bind_buffer(instance: DOMReference, target: f32, buffer: DOMReference) {
    unsafe { webgl_bind_buffer(instance, target, buffer) }
}
extern "C" {
    fn webgl_bind_framebuffer(
        instance: DOMReference,
        bind_framebuffer: f32,
        bind_framebuffer: DOMReference,
    );
}

pub fn bind_framebuffer(instance: DOMReference, target: f32, framebuffer: DOMReference) {
    unsafe { webgl_bind_framebuffer(instance, target, framebuffer) }
}
extern "C" {
    fn webgl_bind_renderbuffer(
        instance: DOMReference,
        bind_renderbuffer: f32,
        bind_renderbuffer: DOMReference,
    );
}

pub fn bind_renderbuffer(instance: DOMReference, target: f32, renderbuffer: DOMReference) {
    unsafe { webgl_bind_renderbuffer(instance, target, renderbuffer) }
}
extern "C" {
    fn webgl_bind_texture(instance: DOMReference, bind_texture: f32, bind_texture: DOMReference);
}

pub fn bind_texture(instance: DOMReference, target: f32, texture: DOMReference) {
    unsafe { webgl_bind_texture(instance, target, texture) }
}
extern "C" {
    fn webgl_blend_color(
        instance: DOMReference,
        blend_color: f32,
        blend_color: f32,
        blend_color: f32,
        blend_color: f32,
    );
}

pub fn blend_color(instance: DOMReference, red: f32, green: f32, blue: f32, alpha: f32) {
    unsafe { webgl_blend_color(instance, red, green, blue, alpha) }
}
extern "C" {
    fn webgl_blend_equation(instance: DOMReference, blend_equation: f32);
}

pub fn blend_equation(instance: DOMReference, mode: f32) {
    unsafe { webgl_blend_equation(instance, mode) }
}
extern "C" {
    fn webgl_blend_equation_separate(
        instance: DOMReference,
        blend_equation_separate: f32,
        blend_equation_separate: f32,
    );
}

pub fn blend_equation_separate(instance: DOMReference, mode_r_g_b: f32, mode_alpha: f32) {
    unsafe { webgl_blend_equation_separate(instance, mode_r_g_b, mode_alpha) }
}
extern "C" {
    fn webgl_blend_func(instance: DOMReference, blend_func: f32, blend_func: f32);
}

pub fn blend_func(instance: DOMReference, sfactor: f32, dfactor: f32) {
    unsafe { webgl_blend_func(instance, sfactor, dfactor) }
}
extern "C" {
    fn webgl_blend_func_separate(
        instance: DOMReference,
        blend_func_separate: f32,
        blend_func_separate: f32,
        blend_func_separate: f32,
        blend_func_separate: f32,
    );
}

pub fn blend_func_separate(
    instance: DOMReference,
    src_r_g_b: f32,
    dst_r_g_b: f32,
    src_alpha: f32,
    dst_alpha: f32,
) {
    unsafe { webgl_blend_func_separate(instance, src_r_g_b, dst_r_g_b, src_alpha, dst_alpha) }
}
extern "C" {
    fn webgl_check_framebuffer_status(instance: DOMReference, check_framebuffer_status: f32)
        -> f32;
}

pub fn check_framebuffer_status(instance: DOMReference, target: f32) -> f32 {
    unsafe { webgl_check_framebuffer_status(instance, target) }
}
extern "C" {
    fn webgl_clear(instance: DOMReference, clear: f32);
}

pub fn clear(instance: DOMReference, mask: f32) {
    unsafe { webgl_clear(instance, mask) }
}
extern "C" {
    fn webgl_clear_color(
        instance: DOMReference,
        clear_color: f32,
        clear_color: f32,
        clear_color: f32,
        clear_color: f32,
    );
}

pub fn clear_color(instance: DOMReference, red: f32, green: f32, blue: f32, alpha: f32) {
    unsafe { webgl_clear_color(instance, red, green, blue, alpha) }
}
extern "C" {
    fn webgl_clear_depth(instance: DOMReference, clear_depth: f32);
}

pub fn clear_depth(instance: DOMReference, depth: f32) {
    unsafe { webgl_clear_depth(instance, depth) }
}
extern "C" {
    fn webgl_clear_stencil(instance: DOMReference, clear_stencil: f32);
}

pub fn clear_stencil(instance: DOMReference, s: f32) {
    unsafe { webgl_clear_stencil(instance, s) }
}
extern "C" {
    fn webgl_color_mask(
        instance: DOMReference,
        color_mask: i32,
        color_mask: i32,
        color_mask: i32,
        color_mask: i32,
    );
}

pub fn color_mask(instance: DOMReference, red: bool, green: bool, blue: bool, alpha: bool) {
    unsafe {
        webgl_color_mask(
            instance,
            if red { 1 } else { 0 },
            if green { 1 } else { 0 },
            if blue { 1 } else { 0 },
            if alpha { 1 } else { 0 },
        )
    }
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
        copy_tex_image2_d: f32,
        copy_tex_image2_d: f32,
        copy_tex_image2_d: f32,
        copy_tex_image2_d: f32,
        copy_tex_image2_d: f32,
        copy_tex_image2_d: f32,
        copy_tex_image2_d: f32,
        copy_tex_image2_d: f32,
    );
}

pub fn copy_tex_image2_d(
    instance: DOMReference,
    target: f32,
    level: f32,
    internalformat: f32,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    border: f32,
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
        copy_tex_sub_image2_d: f32,
        copy_tex_sub_image2_d: f32,
        copy_tex_sub_image2_d: f32,
        copy_tex_sub_image2_d: f32,
        copy_tex_sub_image2_d: f32,
        copy_tex_sub_image2_d: f32,
        copy_tex_sub_image2_d: f32,
        copy_tex_sub_image2_d: f32,
    );
}

pub fn copy_tex_sub_image2_d(
    instance: DOMReference,
    target: f32,
    level: f32,
    xoffset: f32,
    yoffset: f32,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
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
    fn webgl_create_shader(instance: DOMReference, create_shader: f32) -> DOMReference;
}

pub fn create_shader(instance: DOMReference, shader_type: f32) -> DOMReference {
    unsafe { webgl_create_shader(instance, shader_type) }
}
extern "C" {
    fn webgl_create_texture(instance: DOMReference) -> DOMReference;
}

pub fn create_texture(instance: DOMReference) -> DOMReference {
    unsafe { webgl_create_texture(instance) }
}
extern "C" {
    fn webgl_cull_face(instance: DOMReference, cull_face: f32);
}

pub fn cull_face(instance: DOMReference, mode: f32) {
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
    fn webgl_depth_func(instance: DOMReference, depth_func: f32);
}

pub fn depth_func(instance: DOMReference, func: f32) {
    unsafe { webgl_depth_func(instance, func) }
}
extern "C" {
    fn webgl_depth_mask(instance: DOMReference, depth_mask: i32);
}

pub fn depth_mask(instance: DOMReference, flag: bool) {
    unsafe { webgl_depth_mask(instance, if flag { 1 } else { 0 }) }
}
extern "C" {
    fn webgl_depth_range(instance: DOMReference, depth_range: f32, depth_range: f32);
}

pub fn depth_range(instance: DOMReference, z_near: f32, z_far: f32) {
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
    fn webgl_disable(instance: DOMReference, disable: f32);
}

pub fn disable(instance: DOMReference, cap: f32) {
    unsafe { webgl_disable(instance, cap) }
}
extern "C" {
    fn webgl_disable_vertex_attrib_array(instance: DOMReference, disable_vertex_attrib_array: f32);
}

pub fn disable_vertex_attrib_array(instance: DOMReference, index: f32) {
    unsafe { webgl_disable_vertex_attrib_array(instance, index) }
}
extern "C" {
    fn webgl_draw_arrays(
        instance: DOMReference,
        draw_arrays: f32,
        draw_arrays: f32,
        draw_arrays: f32,
    );
}

pub fn draw_arrays(instance: DOMReference, mode: f32, first: f32, count: f32) {
    unsafe { webgl_draw_arrays(instance, mode, first, count) }
}
extern "C" {
    fn webgl_draw_elements(
        instance: DOMReference,
        draw_elements: f32,
        draw_elements: f32,
        draw_elements: f32,
        draw_elements: f32,
    );
}

pub fn draw_elements(
    instance: DOMReference,
    mode: f32,
    count: f32,
    element_type: f32,
    offset: f32,
) {
    unsafe { webgl_draw_elements(instance, mode, count, element_type, offset) }
}
extern "C" {
    fn webgl_enable(instance: DOMReference, enable: f32);
}

pub fn enable(instance: DOMReference, cap: f32) {
    unsafe { webgl_enable(instance, cap) }
}
extern "C" {
    fn webgl_enable_vertex_attrib_array(instance: DOMReference, enable_vertex_attrib_array: f32);
}

pub fn enable_vertex_attrib_array(instance: DOMReference, index: f32) {
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
        framebuffer_renderbuffer: f32,
        framebuffer_renderbuffer: f32,
        framebuffer_renderbuffer: f32,
        framebuffer_renderbuffer: DOMReference,
    );
}

pub fn framebuffer_renderbuffer(
    instance: DOMReference,
    target: f32,
    attachment: f32,
    renderbuffertarget: f32,
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
        framebuffer_texture2_d: f32,
        framebuffer_texture2_d: f32,
        framebuffer_texture2_d: f32,
        framebuffer_texture2_d: DOMReference,
        framebuffer_texture2_d: f32,
    );
}

pub fn framebuffer_texture2_d(
    instance: DOMReference,
    target: f32,
    attachment: f32,
    textarget: f32,
    texture: DOMReference,
    level: f32,
) {
    unsafe { webgl_framebuffer_texture2_d(instance, target, attachment, textarget, texture, level) }
}
extern "C" {
    fn webgl_front_face(instance: DOMReference, front_face: f32);
}

pub fn front_face(instance: DOMReference, mode: f32) {
    unsafe { webgl_front_face(instance, mode) }
}
extern "C" {
    fn webgl_generate_mipmap(instance: DOMReference, generate_mipmap: f32);
}

pub fn generate_mipmap(instance: DOMReference, target: f32) {
    unsafe { webgl_generate_mipmap(instance, target) }
}
extern "C" {
    fn webgl_get_active_attrib(
        instance: DOMReference,
        get_active_attrib: DOMReference,
        get_active_attrib: f32,
    ) -> DOMReference;
}

pub fn get_active_attrib(
    instance: DOMReference,
    program: DOMReference,
    index: f32,
) -> DOMReference {
    unsafe { webgl_get_active_attrib(instance, program, index) }
}
extern "C" {
    fn webgl_get_active_uniform(
        instance: DOMReference,
        get_active_uniform: DOMReference,
        get_active_uniform: f32,
    ) -> DOMReference;
}

pub fn get_active_uniform(
    instance: DOMReference,
    program: DOMReference,
    index: f32,
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
    ) -> f32;
}

pub fn get_attrib_location(instance: DOMReference, program: DOMReference, name: &str) -> f32 {
    unsafe { webgl_get_attrib_location(instance, program, to_cstring(name)) }
}
extern "C" {
    fn webgl_get_buffer_parameter(
        instance: DOMReference,
        get_buffer_parameter: f32,
        get_buffer_parameter: f32,
    ) -> DOMReference;
}

pub fn get_buffer_parameter(instance: DOMReference, target: f32, pname: f32) -> DOMReference {
    unsafe { webgl_get_buffer_parameter(instance, target, pname) }
}
extern "C" {
    fn webgl_get_parameter(instance: DOMReference, get_parameter: f32) -> DOMReference;
}

pub fn get_parameter(instance: DOMReference, pname: f32) -> DOMReference {
    unsafe { webgl_get_parameter(instance, pname) }
}
extern "C" {
    fn webgl_get_error(instance: DOMReference) -> f32;
}

pub fn get_error(instance: DOMReference) -> f32 {
    unsafe { webgl_get_error(instance) }
}
extern "C" {
    fn webgl_get_framebuffer_attachment_parameter(
        instance: DOMReference,
        get_framebuffer_attachment_parameter: f32,
        get_framebuffer_attachment_parameter: f32,
        get_framebuffer_attachment_parameter: f32,
    ) -> DOMReference;
}

pub fn get_framebuffer_attachment_parameter(
    instance: DOMReference,
    target: f32,
    attachment: f32,
    pname: f32,
) -> DOMReference {
    unsafe { webgl_get_framebuffer_attachment_parameter(instance, target, attachment, pname) }
}
extern "C" {
    fn webgl_get_program_parameter(
        instance: DOMReference,
        get_program_parameter: DOMReference,
        get_program_parameter: f32,
    ) -> DOMReference;
}

pub fn get_program_parameter(
    instance: DOMReference,
    program: DOMReference,
    pname: f32,
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
        get_renderbuffer_parameter: f32,
        get_renderbuffer_parameter: f32,
    ) -> DOMReference;
}

pub fn get_renderbuffer_parameter(instance: DOMReference, target: f32, pname: f32) -> DOMReference {
    unsafe { webgl_get_renderbuffer_parameter(instance, target, pname) }
}
extern "C" {
    fn webgl_get_shader_parameter(
        instance: DOMReference,
        get_shader_parameter: DOMReference,
        get_shader_parameter: f32,
    ) -> DOMReference;
}

pub fn get_shader_parameter(
    instance: DOMReference,
    shader: DOMReference,
    pname: f32,
) -> DOMReference {
    unsafe { webgl_get_shader_parameter(instance, shader, pname) }
}
extern "C" {
    fn webgl_get_shader_precision_format(
        instance: DOMReference,
        get_shader_precision_format: f32,
        get_shader_precision_format: f32,
    ) -> DOMReference;
}

pub fn get_shader_precision_format(
    instance: DOMReference,
    shadertype: f32,
    precisiontype: f32,
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
        get_tex_parameter: f32,
        get_tex_parameter: f32,
    ) -> DOMReference;
}

pub fn get_tex_parameter(instance: DOMReference, target: f32, pname: f32) -> DOMReference {
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
        get_vertex_attrib: f32,
        get_vertex_attrib: f32,
    ) -> DOMReference;
}

pub fn get_vertex_attrib(instance: DOMReference, index: f32, pname: f32) -> DOMReference {
    unsafe { webgl_get_vertex_attrib(instance, index, pname) }
}
extern "C" {
    fn webgl_get_vertex_attrib_offset(
        instance: DOMReference,
        get_vertex_attrib_offset: f32,
        get_vertex_attrib_offset: f32,
    ) -> f32;
}

pub fn get_vertex_attrib_offset(instance: DOMReference, index: f32, pname: f32) -> f32 {
    unsafe { webgl_get_vertex_attrib_offset(instance, index, pname) }
}
extern "C" {
    fn webgl_hint(instance: DOMReference, hint: f32, hint: f32);
}

pub fn hint(instance: DOMReference, target: f32, mode: f32) {
    unsafe { webgl_hint(instance, target, mode) }
}
extern "C" {
    fn webgl_is_buffer(instance: DOMReference, is_buffer: DOMReference) -> i32;
}

pub fn is_buffer(instance: DOMReference, buffer: DOMReference) -> bool {
    unsafe { 0 != webgl_is_buffer(instance, buffer) }
}
extern "C" {
    fn webgl_is_enabled(instance: DOMReference, is_enabled: f32) -> i32;
}

pub fn is_enabled(instance: DOMReference, cap: f32) -> bool {
    unsafe { 0 != webgl_is_enabled(instance, cap) }
}
extern "C" {
    fn webgl_is_framebuffer(instance: DOMReference, is_framebuffer: DOMReference) -> i32;
}

pub fn is_framebuffer(instance: DOMReference, framebuffer: DOMReference) -> bool {
    unsafe { 0 != webgl_is_framebuffer(instance, framebuffer) }
}
extern "C" {
    fn webgl_is_program(instance: DOMReference, is_program: DOMReference) -> i32;
}

pub fn is_program(instance: DOMReference, program: DOMReference) -> bool {
    unsafe { 0 != webgl_is_program(instance, program) }
}
extern "C" {
    fn webgl_is_renderbuffer(instance: DOMReference, is_renderbuffer: DOMReference) -> i32;
}

pub fn is_renderbuffer(instance: DOMReference, renderbuffer: DOMReference) -> bool {
    unsafe { 0 != webgl_is_renderbuffer(instance, renderbuffer) }
}
extern "C" {
    fn webgl_is_shader(instance: DOMReference, is_shader: DOMReference) -> i32;
}

pub fn is_shader(instance: DOMReference, shader: DOMReference) -> bool {
    unsafe { 0 != webgl_is_shader(instance, shader) }
}
extern "C" {
    fn webgl_is_texture(instance: DOMReference, is_texture: DOMReference) -> i32;
}

pub fn is_texture(instance: DOMReference, texture: DOMReference) -> bool {
    unsafe { 0 != webgl_is_texture(instance, texture) }
}
extern "C" {
    fn webgl_line_width(instance: DOMReference, line_width: f32);
}

pub fn line_width(instance: DOMReference, width: f32) {
    unsafe { webgl_line_width(instance, width) }
}
extern "C" {
    fn webgl_link_program(instance: DOMReference, link_program: DOMReference);
}

pub fn link_program(instance: DOMReference, program: DOMReference) {
    unsafe { webgl_link_program(instance, program) }
}
extern "C" {
    fn webgl_pixel_storei(instance: DOMReference, pixel_storei: f32, pixel_storei: f32);
}

pub fn pixel_storei(instance: DOMReference, pname: f32, param: f32) {
    unsafe { webgl_pixel_storei(instance, pname, param) }
}
extern "C" {
    fn webgl_polygon_offset(instance: DOMReference, polygon_offset: f32, polygon_offset: f32);
}

pub fn polygon_offset(instance: DOMReference, factor: f32, units: f32) {
    unsafe { webgl_polygon_offset(instance, factor, units) }
}
extern "C" {
    fn webgl_renderbuffer_storage(
        instance: DOMReference,
        renderbuffer_storage: f32,
        renderbuffer_storage: f32,
        renderbuffer_storage: f32,
        renderbuffer_storage: f32,
    );
}

pub fn renderbuffer_storage(
    instance: DOMReference,
    target: f32,
    internalformat: f32,
    width: f32,
    height: f32,
) {
    unsafe { webgl_renderbuffer_storage(instance, target, internalformat, width, height) }
}
extern "C" {
    fn webgl_sample_coverage(instance: DOMReference, sample_coverage: f32, sample_coverage: i32);
}

pub fn sample_coverage(instance: DOMReference, value: f32, invert: bool) {
    unsafe { webgl_sample_coverage(instance, value, if invert { 1 } else { 0 }) }
}
extern "C" {
    fn webgl_scissor(
        instance: DOMReference,
        scissor: f32,
        scissor: f32,
        scissor: f32,
        scissor: f32,
    );
}

pub fn scissor(instance: DOMReference, x: f32, y: f32, width: f32, height: f32) {
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
        stencil_func: f32,
        stencil_func: f32,
        stencil_func: f32,
    );
}

pub fn stencil_func(instance: DOMReference, func: f32, setencel_ref: f32, mask: f32) {
    unsafe { webgl_stencil_func(instance, func, setencel_ref, mask) }
}
extern "C" {
    fn webgl_stencil_func_separate(
        instance: DOMReference,
        stencil_func_separate: f32,
        stencil_func_separate: f32,
        stencil_func_separate: f32,
        stencil_func_separate: f32,
    );
}

pub fn stencil_func_separate(
    instance: DOMReference,
    face: f32,
    func: f32,
    setencel_ref: f32,
    mask: f32,
) {
    unsafe { webgl_stencil_func_separate(instance, face, func, setencel_ref, mask) }
}
extern "C" {
    fn webgl_stencil_mask(instance: DOMReference, stencil_mask: f32);
}

pub fn stencil_mask(instance: DOMReference, mask: f32) {
    unsafe { webgl_stencil_mask(instance, mask) }
}
extern "C" {
    fn webgl_stencil_mask_separate(
        instance: DOMReference,
        stencil_mask_separate: f32,
        stencil_mask_separate: f32,
    );
}

pub fn stencil_mask_separate(instance: DOMReference, face: f32, mask: f32) {
    unsafe { webgl_stencil_mask_separate(instance, face, mask) }
}
extern "C" {
    fn webgl_stencil_op(instance: DOMReference, stencil_op: f32, stencil_op: f32, stencil_op: f32);
}

pub fn stencil_op(instance: DOMReference, fail: f32, zfail: f32, zpass: f32) {
    unsafe { webgl_stencil_op(instance, fail, zfail, zpass) }
}
extern "C" {
    fn webgl_stencil_op_separate(
        instance: DOMReference,
        stencil_op_separate: f32,
        stencil_op_separate: f32,
        stencil_op_separate: f32,
        stencil_op_separate: f32,
    );
}

pub fn stencil_op_separate(instance: DOMReference, face: f32, fail: f32, zfail: f32, zpass: f32) {
    unsafe { webgl_stencil_op_separate(instance, face, fail, zfail, zpass) }
}
extern "C" {
    fn webgl_tex_parameterf(
        instance: DOMReference,
        tex_parameterf: f32,
        tex_parameterf: f32,
        tex_parameterf: f32,
    );
}

pub fn tex_parameterf(instance: DOMReference, target: f32, pname: f32, param: f32) {
    unsafe { webgl_tex_parameterf(instance, target, pname, param) }
}
extern "C" {
    fn webgl_tex_parameteri(
        instance: DOMReference,
        tex_parameteri: f32,
        tex_parameteri: f32,
        tex_parameteri: f32,
    );
}

pub fn tex_parameteri(instance: DOMReference, target: f32, pname: f32, param: f32) {
    unsafe { webgl_tex_parameteri(instance, target, pname, param) }
}
extern "C" {
    fn webgl_uniform1f(instance: DOMReference, uniform1f: DOMReference, uniform1f: f32);
}

pub fn uniform1f(instance: DOMReference, location: DOMReference, x: f32) {
    unsafe { webgl_uniform1f(instance, location, x) }
}
extern "C" {
    fn webgl_uniform2f(
        instance: DOMReference,
        uniform2f: DOMReference,
        uniform2f: f32,
        uniform2f: f32,
    );
}

pub fn uniform2f(instance: DOMReference, location: DOMReference, x: f32, y: f32) {
    unsafe { webgl_uniform2f(instance, location, x, y) }
}
extern "C" {
    fn webgl_uniform3f(
        instance: DOMReference,
        uniform3f: DOMReference,
        uniform3f: f32,
        uniform3f: f32,
        uniform3f: f32,
    );
}

pub fn uniform3f(instance: DOMReference, location: DOMReference, x: f32, y: f32, z: f32) {
    unsafe { webgl_uniform3f(instance, location, x, y, z) }
}
extern "C" {
    fn webgl_uniform4f(
        instance: DOMReference,
        uniform4f: DOMReference,
        uniform4f: f32,
        uniform4f: f32,
        uniform4f: f32,
        uniform4f: f32,
    );
}

pub fn uniform4f(instance: DOMReference, location: DOMReference, x: f32, y: f32, z: f32, w: f32) {
    unsafe { webgl_uniform4f(instance, location, x, y, z, w) }
}
extern "C" {
    fn webgl_uniform1i(instance: DOMReference, uniform1i: DOMReference, uniform1i: f32);
}

pub fn uniform1i(instance: DOMReference, location: DOMReference, x: f32) {
    unsafe { webgl_uniform1i(instance, location, x) }
}
extern "C" {
    fn webgl_uniform2i(
        instance: DOMReference,
        uniform2i: DOMReference,
        uniform2i: f32,
        uniform2i: f32,
    );
}

pub fn uniform2i(instance: DOMReference, location: DOMReference, x: f32, y: f32) {
    unsafe { webgl_uniform2i(instance, location, x, y) }
}
extern "C" {
    fn webgl_uniform3i(
        instance: DOMReference,
        uniform3i: DOMReference,
        uniform3i: f32,
        uniform3i: f32,
        uniform3i: f32,
    );
}

pub fn uniform3i(instance: DOMReference, location: DOMReference, x: f32, y: f32, z: f32) {
    unsafe { webgl_uniform3i(instance, location, x, y, z) }
}
extern "C" {
    fn webgl_uniform4i(
        instance: DOMReference,
        uniform4i: DOMReference,
        uniform4i: f32,
        uniform4i: f32,
        uniform4i: f32,
        uniform4i: f32,
    );
}

pub fn uniform4i(instance: DOMReference, location: DOMReference, x: f32, y: f32, z: f32, w: f32) {
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
    fn webgl_vertex_attrib1f(instance: DOMReference, vertex_attrib1f: f32, vertex_attrib1f: f32);
}

pub fn vertex_attrib1f(instance: DOMReference, indx: f32, x: f32) {
    unsafe { webgl_vertex_attrib1f(instance, indx, x) }
}
extern "C" {
    fn webgl_vertex_attrib1fv(
        instance: DOMReference,
        vertex_attrib1fv: f32,
        vertex_attrib1fv: DOMReference,
    );
}

pub fn vertex_attrib1fv(instance: DOMReference, indx: f32, values: DOMReference) {
    unsafe { webgl_vertex_attrib1fv(instance, indx, values) }
}
extern "C" {
    fn webgl_vertex_attrib2f(
        instance: DOMReference,
        vertex_attrib2f: f32,
        vertex_attrib2f: f32,
        vertex_attrib2f: f32,
    );
}

pub fn vertex_attrib2f(instance: DOMReference, indx: f32, x: f32, y: f32) {
    unsafe { webgl_vertex_attrib2f(instance, indx, x, y) }
}
extern "C" {
    fn webgl_vertex_attrib2fv(
        instance: DOMReference,
        vertex_attrib2fv: f32,
        vertex_attrib2fv: DOMReference,
    );
}

pub fn vertex_attrib2fv(instance: DOMReference, indx: f32, values: DOMReference) {
    unsafe { webgl_vertex_attrib2fv(instance, indx, values) }
}
extern "C" {
    fn webgl_vertex_attrib3f(
        instance: DOMReference,
        vertex_attrib3f: f32,
        vertex_attrib3f: f32,
        vertex_attrib3f: f32,
        vertex_attrib3f: f32,
    );
}

pub fn vertex_attrib3f(instance: DOMReference, indx: f32, x: f32, y: f32, z: f32) {
    unsafe { webgl_vertex_attrib3f(instance, indx, x, y, z) }
}
extern "C" {
    fn webgl_vertex_attrib3fv(
        instance: DOMReference,
        vertex_attrib3fv: f32,
        vertex_attrib3fv: DOMReference,
    );
}

pub fn vertex_attrib3fv(instance: DOMReference, indx: f32, values: DOMReference) {
    unsafe { webgl_vertex_attrib3fv(instance, indx, values) }
}
extern "C" {
    fn webgl_vertex_attrib4f(
        instance: DOMReference,
        vertex_attrib4f: f32,
        vertex_attrib4f: f32,
        vertex_attrib4f: f32,
        vertex_attrib4f: f32,
        vertex_attrib4f: f32,
    );
}

pub fn vertex_attrib4f(instance: DOMReference, indx: f32, x: f32, y: f32, z: f32, w: f32) {
    unsafe { webgl_vertex_attrib4f(instance, indx, x, y, z, w) }
}
extern "C" {
    fn webgl_vertex_attrib4fv(
        instance: DOMReference,
        vertex_attrib4fv: f32,
        vertex_attrib4fv: DOMReference,
    );
}

pub fn vertex_attrib4fv(instance: DOMReference, indx: f32, values: DOMReference) {
    unsafe { webgl_vertex_attrib4fv(instance, indx, values) }
}
extern "C" {
    fn webgl_vertex_attrib_pointer(
        instance: DOMReference,
        vertex_attrib_pointer: f32,
        vertex_attrib_pointer: f32,
        vertex_attrib_pointer: f32,
        vertex_attrib_pointer: i32,
        vertex_attrib_pointer: f32,
        vertex_attrib_pointer: f32,
    );
}

pub fn vertex_attrib_pointer(
    instance: DOMReference,
    indx: f32,
    size: f32,
    pointer_type: f32,
    normalized: bool,
    stride: f32,
    offset: f32,
) {
    unsafe {
        webgl_vertex_attrib_pointer(
            instance,
            indx,
            size,
            pointer_type,
            if normalized { 1 } else { 0 },
            stride,
            offset,
        )
    }
}
extern "C" {
    fn webgl_viewport(
        instance: DOMReference,
        viewport: f32,
        viewport: f32,
        viewport: f32,
        viewport: f32,
    );
}

pub fn viewport(instance: DOMReference, x: f32, y: f32, width: f32, height: f32) {
    unsafe { webgl_viewport(instance, x, y, width, height) }
}
extern "C" {
    fn webgl_buffer_data(
        instance: DOMReference,
        buffer_data: f32,
        buffer_data: DOMReference,
        buffer_data: f32,
    );
}

pub fn buffer_data(instance: DOMReference, target: f32, data: DOMReference, usage: f32) {
    unsafe { webgl_buffer_data(instance, target, data, usage) }
}
extern "C" {
    fn webgl_buffer_data_1(
        instance: DOMReference,
        buffer_data_1: f32,
        buffer_data_1: DOMReference,
        buffer_data_1: f32,
    );
}

pub fn buffer_data_1(instance: DOMReference, target: f32, size: DOMReference, usage: f32) {
    unsafe { webgl_buffer_data_1(instance, target, size, usage) }
}
extern "C" {
    fn webgl_buffer_data_2(
        instance: DOMReference,
        buffer_data_2: f32,
        buffer_data_2: DOMReference,
        buffer_data_2: f32,
    );
}

pub fn buffer_data_2(instance: DOMReference, target: f32, data: DOMReference, usage: f32) {
    unsafe { webgl_buffer_data_2(instance, target, data, usage) }
}
extern "C" {
    fn webgl_buffer_sub_data(
        instance: DOMReference,
        buffer_sub_data: f32,
        buffer_sub_data: f32,
        buffer_sub_data: DOMReference,
    );
}

pub fn buffer_sub_data(instance: DOMReference, target: f32, offset: f32, data: DOMReference) {
    unsafe { webgl_buffer_sub_data(instance, target, offset, data) }
}
extern "C" {
    fn webgl_buffer_sub_data_1(
        instance: DOMReference,
        buffer_sub_data_1: f32,
        buffer_sub_data_1: f32,
        buffer_sub_data_1: DOMReference,
    );
}

pub fn buffer_sub_data_1(instance: DOMReference, target: f32, offset: f32, data: DOMReference) {
    unsafe { webgl_buffer_sub_data_1(instance, target, offset, data) }
}
extern "C" {
    fn webgl_compressed_tex_image2_d(
        instance: DOMReference,
        compressed_tex_image2_d: f32,
        compressed_tex_image2_d: f32,
        compressed_tex_image2_d: f32,
        compressed_tex_image2_d: f32,
        compressed_tex_image2_d: f32,
        compressed_tex_image2_d: f32,
        compressed_tex_image2_d: DOMReference,
    );
}

pub fn compressed_tex_image2_d(
    instance: DOMReference,
    target: f32,
    level: f32,
    internalformat: f32,
    width: f32,
    height: f32,
    border: f32,
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
        compressed_tex_sub_image2_d: f32,
        compressed_tex_sub_image2_d: f32,
        compressed_tex_sub_image2_d: f32,
        compressed_tex_sub_image2_d: f32,
        compressed_tex_sub_image2_d: f32,
        compressed_tex_sub_image2_d: f32,
        compressed_tex_sub_image2_d: f32,
        compressed_tex_sub_image2_d: DOMReference,
    );
}

pub fn compressed_tex_sub_image2_d(
    instance: DOMReference,
    target: f32,
    level: f32,
    xoffset: f32,
    yoffset: f32,
    width: f32,
    height: f32,
    format: f32,
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
        read_pixels: f32,
        read_pixels: f32,
        read_pixels: f32,
        read_pixels: f32,
        read_pixels: f32,
        read_pixels: f32,
        read_pixels: DOMReference,
    );
}

pub fn read_pixels(
    instance: DOMReference,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    format: f32,
    pixel_type: f32,
    pixels: DOMReference,
) {
    unsafe { webgl_read_pixels(instance, x, y, width, height, format, pixel_type, pixels) }
}
extern "C" {
    fn webgl_tex_image2_d(
        instance: DOMReference,
        tex_image2_d: f32,
        tex_image2_d: f32,
        tex_image2_d: f32,
        tex_image2_d: f32,
        tex_image2_d: f32,
        tex_image2_d: f32,
        tex_image2_d: f32,
        tex_image2_d: f32,
        tex_image2_d: DOMReference,
    );
}

pub fn tex_image2_d(
    instance: DOMReference,
    target: f32,
    level: f32,
    internalformat: f32,
    width: f32,
    height: f32,
    border: f32,
    format: f32,
    image_type: f32,
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
        tex_image2_d_1: f32,
        tex_image2_d_1: f32,
        tex_image2_d_1: f32,
        tex_image2_d_1: f32,
        tex_image2_d_1: f32,
        tex_image2_d_1: DOMReference,
    );
}

pub fn tex_image2_d_1(
    instance: DOMReference,
    target: f32,
    level: f32,
    internalformat: f32,
    format: f32,
    image_type: f32,
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
        tex_image2_d_2: f32,
        tex_image2_d_2: f32,
        tex_image2_d_2: f32,
        tex_image2_d_2: f32,
        tex_image2_d_2: f32,
        tex_image2_d_2: DOMReference,
    );
}

pub fn tex_image2_d_2(
    instance: DOMReference,
    target: f32,
    level: f32,
    internalformat: f32,
    format: f32,
    image_type: f32,
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
        tex_image2_d_3: f32,
        tex_image2_d_3: f32,
        tex_image2_d_3: f32,
        tex_image2_d_3: f32,
        tex_image2_d_3: f32,
        tex_image2_d_3: DOMReference,
    );
}

pub fn tex_image2_d_3(
    instance: DOMReference,
    target: f32,
    level: f32,
    internalformat: f32,
    format: f32,
    image_type: f32,
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
        tex_image2_d_4: f32,
        tex_image2_d_4: f32,
        tex_image2_d_4: f32,
        tex_image2_d_4: f32,
        tex_image2_d_4: f32,
        tex_image2_d_4: DOMReference,
    );
}

pub fn tex_image2_d_4(
    instance: DOMReference,
    target: f32,
    level: f32,
    internalformat: f32,
    format: f32,
    image_type: f32,
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
        tex_image2_d_5: f32,
        tex_image2_d_5: f32,
        tex_image2_d_5: f32,
        tex_image2_d_5: f32,
        tex_image2_d_5: f32,
        tex_image2_d_5: DOMReference,
    );
}

pub fn tex_image2_d_5(
    instance: DOMReference,
    target: f32,
    level: f32,
    internalformat: f32,
    format: f32,
    image_type: f32,
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
        tex_sub_image2_d: f32,
        tex_sub_image2_d: f32,
        tex_sub_image2_d: f32,
        tex_sub_image2_d: f32,
        tex_sub_image2_d: f32,
        tex_sub_image2_d: f32,
        tex_sub_image2_d: f32,
        tex_sub_image2_d: f32,
        tex_sub_image2_d: DOMReference,
    );
}

pub fn tex_sub_image2_d(
    instance: DOMReference,
    target: f32,
    level: f32,
    xoffset: f32,
    yoffset: f32,
    width: f32,
    height: f32,
    format: f32,
    image_type: f32,
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
        tex_sub_image2_d_1: f32,
        tex_sub_image2_d_1: f32,
        tex_sub_image2_d_1: f32,
        tex_sub_image2_d_1: f32,
        tex_sub_image2_d_1: f32,
        tex_sub_image2_d_1: f32,
        tex_sub_image2_d_1: DOMReference,
    );
}

pub fn tex_sub_image2_d_1(
    instance: DOMReference,
    target: f32,
    level: f32,
    xoffset: f32,
    yoffset: f32,
    format: f32,
    image_type: f32,
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
        tex_sub_image2_d_2: f32,
        tex_sub_image2_d_2: f32,
        tex_sub_image2_d_2: f32,
        tex_sub_image2_d_2: f32,
        tex_sub_image2_d_2: f32,
        tex_sub_image2_d_2: f32,
        tex_sub_image2_d_2: DOMReference,
    );
}

pub fn tex_sub_image2_d_2(
    instance: DOMReference,
    target: f32,
    level: f32,
    xoffset: f32,
    yoffset: f32,
    format: f32,
    image_type: f32,
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
        tex_sub_image2_d_3: f32,
        tex_sub_image2_d_3: f32,
        tex_sub_image2_d_3: f32,
        tex_sub_image2_d_3: f32,
        tex_sub_image2_d_3: f32,
        tex_sub_image2_d_3: f32,
        tex_sub_image2_d_3: DOMReference,
    );
}

pub fn tex_sub_image2_d_3(
    instance: DOMReference,
    target: f32,
    level: f32,
    xoffset: f32,
    yoffset: f32,
    format: f32,
    image_type: f32,
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
        tex_sub_image2_d_4: f32,
        tex_sub_image2_d_4: f32,
        tex_sub_image2_d_4: f32,
        tex_sub_image2_d_4: f32,
        tex_sub_image2_d_4: f32,
        tex_sub_image2_d_4: f32,
        tex_sub_image2_d_4: DOMReference,
    );
}

pub fn tex_sub_image2_d_4(
    instance: DOMReference,
    target: f32,
    level: f32,
    xoffset: f32,
    yoffset: f32,
    format: f32,
    image_type: f32,
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
        tex_sub_image2_d_5: f32,
        tex_sub_image2_d_5: f32,
        tex_sub_image2_d_5: f32,
        tex_sub_image2_d_5: f32,
        tex_sub_image2_d_5: f32,
        tex_sub_image2_d_5: f32,
        tex_sub_image2_d_5: DOMReference,
    );
}

pub fn tex_sub_image2_d_5(
    instance: DOMReference,
    target: f32,
    level: f32,
    xoffset: f32,
    yoffset: f32,
    format: f32,
    image_type: f32,
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
        uniform_matrix2fv: i32,
        uniform_matrix2fv: DOMReference,
    );
}

pub fn uniform_matrix2fv(
    instance: DOMReference,
    location: DOMReference,
    transpose: bool,
    data: DOMReference,
) {
    unsafe { webgl_uniform_matrix2fv(instance, location, if transpose { 1 } else { 0 }, data) }
}
extern "C" {
    fn webgl_uniform_matrix3fv(
        instance: DOMReference,
        uniform_matrix3fv: DOMReference,
        uniform_matrix3fv: i32,
        uniform_matrix3fv: DOMReference,
    );
}

pub fn uniform_matrix3fv(
    instance: DOMReference,
    location: DOMReference,
    transpose: bool,
    data: DOMReference,
) {
    unsafe { webgl_uniform_matrix3fv(instance, location, if transpose { 1 } else { 0 }, data) }
}
extern "C" {
    fn webgl_uniform_matrix4fv(
        instance: DOMReference,
        uniform_matrix4fv: DOMReference,
        uniform_matrix4fv: i32,
        uniform_matrix4fv: DOMReference,
    );
}

pub fn uniform_matrix4fv(
    instance: DOMReference,
    location: DOMReference,
    transpose: bool,
    data: DOMReference,
) {
    unsafe { webgl_uniform_matrix4fv(instance, location, if transpose { 1 } else { 0 }, data) }
}
extern "C" {
    fn webgl_commit(instance: DOMReference);
}

pub fn commit(instance: DOMReference) {
    unsafe { webgl_commit(instance) }
}
