#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn webglshaderprecisionformat_get_range_min(instance: DOMReference) -> DOMReference;
    fn webglshaderprecisionformat_set_range_min(instance: DOMReference, value: DOMReference);
}

pub fn get_range_min(instance: DOMReference) -> DOMReference {
    unsafe { webglshaderprecisionformat_get_range_min(instance) }
}

pub fn set_range_min(instance: DOMReference, value: DOMReference) {
    unsafe {
        webglshaderprecisionformat_set_range_min(instance, value);
    }
}
extern "C" {
    fn webglshaderprecisionformat_get_range_max(instance: DOMReference) -> DOMReference;
    fn webglshaderprecisionformat_set_range_max(instance: DOMReference, value: DOMReference);
}

pub fn get_range_max(instance: DOMReference) -> DOMReference {
    unsafe { webglshaderprecisionformat_get_range_max(instance) }
}

pub fn set_range_max(instance: DOMReference, value: DOMReference) {
    unsafe {
        webglshaderprecisionformat_set_range_max(instance, value);
    }
}
extern "C" {
    fn webglshaderprecisionformat_get_precision(instance: DOMReference) -> DOMReference;
    fn webglshaderprecisionformat_set_precision(instance: DOMReference, value: DOMReference);
}

pub fn get_precision(instance: DOMReference) -> DOMReference {
    unsafe { webglshaderprecisionformat_get_precision(instance) }
}

pub fn set_precision(instance: DOMReference, value: DOMReference) {
    unsafe {
        webglshaderprecisionformat_set_precision(instance, value);
    }
}
