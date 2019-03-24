#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn webgllosecontext_lose_context(instance: DOMReference);
}

pub fn lose_context(instance: DOMReference) {
    unsafe { webgllosecontext_lose_context(instance) }
}
extern "C" {
    fn webgllosecontext_restore_context(instance: DOMReference);
}

pub fn restore_context(instance: DOMReference) {
    unsafe { webgllosecontext_restore_context(instance) }
}
