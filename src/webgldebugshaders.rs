#[allow(unused_imports)]
use crate::*;
#[allow(unused_imports)]
use alloc::string::String;
extern "C" {
    fn webgldebugshaders_get_translated_shader_source(
        instance: DOMReference,
        get_translated_shader_source: DOMReference,
    ) -> CString;
}

pub fn get_translated_shader_source(instance: DOMReference, shader: DOMReference) -> String {
    unsafe {
        to_string(webgldebugshaders_get_translated_shader_source(
            instance, shader,
        ))
    }
}
