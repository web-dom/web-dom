#[allow(unused_imports)]
use crate::*;
pub const COMPRESSED_RGBA_ASTC_4X4_KHR: f32 = 0x93B0 as f32;
pub const COMPRESSED_RGBA_ASTC_5X4_KHR: f32 = 0x93B1 as f32;
pub const COMPRESSED_RGBA_ASTC_5X5_KHR: f32 = 0x93B2 as f32;
pub const COMPRESSED_RGBA_ASTC_6X5_KHR: f32 = 0x93B3 as f32;
pub const COMPRESSED_RGBA_ASTC_6X6_KHR: f32 = 0x93B4 as f32;
pub const COMPRESSED_RGBA_ASTC_8X5_KHR: f32 = 0x93B5 as f32;
pub const COMPRESSED_RGBA_ASTC_8X6_KHR: f32 = 0x93B6 as f32;
pub const COMPRESSED_RGBA_ASTC_8X8_KHR: f32 = 0x93B7 as f32;
pub const COMPRESSED_RGBA_ASTC_10X5_KHR: f32 = 0x93B8 as f32;
pub const COMPRESSED_RGBA_ASTC_10X6_KHR: f32 = 0x93B9 as f32;
pub const COMPRESSED_RGBA_ASTC_10X8_KHR: f32 = 0x93BA as f32;
pub const COMPRESSED_RGBA_ASTC_10X10_KHR: f32 = 0x93BB as f32;
pub const COMPRESSED_RGBA_ASTC_12X10_KHR: f32 = 0x93BC as f32;
pub const COMPRESSED_RGBA_ASTC_12X12_KHR: f32 = 0x93BD as f32;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_4X4_KHR: f32 = 0x93D0 as f32;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_5X4_KHR: f32 = 0x93D1 as f32;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_5X5_KHR: f32 = 0x93D2 as f32;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_6X5_KHR: f32 = 0x93D3 as f32;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_6X6_KHR: f32 = 0x93D4 as f32;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_8X5_KHR: f32 = 0x93D5 as f32;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_8X6_KHR: f32 = 0x93D6 as f32;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_8X8_KHR: f32 = 0x93D7 as f32;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_10X5_KHR: f32 = 0x93D8 as f32;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_10X6_KHR: f32 = 0x93D9 as f32;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_10X8_KHR: f32 = 0x93DA as f32;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_10X10_KHR: f32 = 0x93DB as f32;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_12X10_KHR: f32 = 0x93DC as f32;
pub const COMPRESSED_SRGB8_ALPHA8_ASTC_12X12_KHR: f32 = 0x93DD as f32;
extern "C" {
    fn webglcompressedtextureastc_get_supported_profiles(instance: DOMReference) -> DOMReference;
}

pub fn get_supported_profiles(instance: DOMReference) -> DOMReference {
    unsafe { webglcompressedtextureastc_get_supported_profiles(instance) }
}
