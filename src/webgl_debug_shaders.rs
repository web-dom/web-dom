#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn webgl_debug_shaders_get_translated_shader_source(
        instance: DOMReference,
        get_translated_shader_source: DOMReference,
    ) -> CString;
}

pub fn get_translated_shader_source(instance: DOMReference, shader: DOMReference) -> String {
    unsafe {
        to_string(webgl_debug_shaders_get_translated_shader_source(
            instance, shader,
        ))
    }
}
