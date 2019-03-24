#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn webgl_compressed_texture_astc_get_supported_profiles(instance: DOMReference)
        -> DOMReference;
}

pub fn get_supported_profiles(instance: DOMReference) -> DOMReference {
    unsafe { webgl_compressed_texture_astc_get_supported_profiles(instance) }
}
