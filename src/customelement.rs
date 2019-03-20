use crate::*;
extern "C" {
    fn customelement_attach_shadow(element: Element) -> Element;
    fn customelement_define(tag: CString);
    fn customelement_define_with_attributes(tag: CString, attributes: CString);
}

pub fn attach_shadow(element: Element) -> Element {
    unsafe { customelement_attach_shadow(element) }
}

pub fn define(tag: &str) {
    unsafe {
        customelement_define(cstr(tag));
    }
}

pub fn define_with_attributes(tag: &str, attributes: &str) {
    unsafe {
        customelement_define_with_attributes(cstr(tag), cstr(attributes));
    }
}
