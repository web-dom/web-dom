#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn webgl_lose_context_lose_context(instance: DOMReference);
}

pub fn lose_context(instance: DOMReference) {
    unsafe { webgl_lose_context_lose_context(instance) }
}
extern "C" {
    fn webgl_lose_context_restore_context(instance: DOMReference);
}

pub fn restore_context(instance: DOMReference) {
    unsafe { webgl_lose_context_restore_context(instance) }
}
