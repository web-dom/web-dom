#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn element_get_namespaceURI(instance: i32) -> CString;
    fn element_set_namespaceURI(instance: i32, value: CString);
}

pub fn get_namespace_uri(instance: i32) -> String {
    unsafe { cstr_to_string(element_get_namespaceURI(instance)) }
}

pub fn set_namespace_uri(instance: i32, value: &str) {
    unsafe {
        element_set_namespaceURI(instance, cstr(value));
    }
}
extern "C" {
    fn element_get_prefix(instance: i32) -> CString;
    fn element_set_prefix(instance: i32, value: CString);
}

pub fn get_prefix(instance: i32) -> String {
    unsafe { cstr_to_string(element_get_prefix(instance)) }
}

pub fn set_prefix(instance: i32, value: &str) {
    unsafe {
        element_set_prefix(instance, cstr(value));
    }
}
extern "C" {
    fn element_get_localName(instance: i32) -> CString;
    fn element_set_localName(instance: i32, value: CString);
}

pub fn get_local_name(instance: i32) -> String {
    unsafe { cstr_to_string(element_get_localName(instance)) }
}

pub fn set_local_name(instance: i32, value: &str) {
    unsafe {
        element_set_localName(instance, cstr(value));
    }
}
extern "C" {
    fn element_get_tagName(instance: i32) -> CString;
    fn element_set_tagName(instance: i32, value: CString);
}

pub fn get_tag_name(instance: i32) -> String {
    unsafe { cstr_to_string(element_get_tagName(instance)) }
}

pub fn set_tag_name(instance: i32, value: &str) {
    unsafe {
        element_set_tagName(instance, cstr(value));
    }
}
extern "C" {
    fn element_get_id(instance: i32) -> CString;
    fn element_set_id(instance: i32, value: CString);
}

pub fn get_id(instance: i32) -> String {
    unsafe { cstr_to_string(element_get_id(instance)) }
}

pub fn set_id(instance: i32, value: &str) {
    unsafe {
        element_set_id(instance, cstr(value));
    }
}
extern "C" {
    fn element_get_className(instance: i32) -> CString;
    fn element_set_className(instance: i32, value: CString);
}

pub fn get_class_name(instance: i32) -> String {
    unsafe { cstr_to_string(element_get_className(instance)) }
}

pub fn set_class_name(instance: i32, value: &str) {
    unsafe {
        element_set_className(instance, cstr(value));
    }
}
extern "C" {
    fn element_get_classList(instance: i32) -> i32;
    fn element_set_classList(instance: i32, value: i32);
}

pub fn get_class_list(instance: i32) -> i32 {
    unsafe { element_get_classList(instance) }
}

pub fn set_class_list(instance: i32, value: i32) {
    unsafe {
        element_set_classList(instance, value);
    }
}
extern "C" {
    fn element_get_attributes(instance: i32) -> i32;
    fn element_set_attributes(instance: i32, value: i32);
}

pub fn get_attributes(instance: i32) -> i32 {
    unsafe { element_get_attributes(instance) }
}

pub fn set_attributes(instance: i32, value: i32) {
    unsafe {
        element_set_attributes(instance, value);
    }
}
extern "C" {
    fn element_get_attribute_names(instance: i32) -> i32;
}

pub fn get_attribute_names(instance: i32) -> i32 {
    unsafe { element_get_attribute_names(instance) }
}
extern "C" {
    fn element_get_attribute(instance: i32, name: CString) -> CString;
}

pub fn get_attribute(instance: i32, name: &str) -> String {
    unsafe { cstr_to_string(element_get_attribute(instance, cstr(name))) }
}
extern "C" {
    fn element_get_attribute_n_s(instance: i32, namespace: CString, local_name: CString)
        -> CString;
}

pub fn get_attribute_n_s(instance: i32, namespace: &str, local_name: &str) -> String {
    unsafe {
        cstr_to_string(element_get_attribute_n_s(
            instance,
            cstr(namespace),
            cstr(local_name),
        ))
    }
}
extern "C" {
    fn element_toggle_attribute(instance: i32, name: CString, force: i32) -> i32;
}

pub fn toggle_attribute(instance: i32, name: &str, force: i32) -> i32 {
    unsafe { element_toggle_attribute(instance, cstr(name), force) }
}
extern "C" {
    fn element_set_attribute(instance: i32, name: CString, value: CString);
}

pub fn set_attribute(instance: i32, name: &str, value: &str) {
    unsafe { element_set_attribute(instance, cstr(name), cstr(value)) }
}
extern "C" {
    fn element_set_attribute_n_s(instance: i32, namespace: CString, name: CString, value: CString);
}

pub fn set_attribute_n_s(instance: i32, namespace: &str, name: &str, value: &str) {
    unsafe { element_set_attribute_n_s(instance, cstr(namespace), cstr(name), cstr(value)) }
}
extern "C" {
    fn element_remove_attribute(instance: i32, name: CString);
}

pub fn remove_attribute(instance: i32, name: &str) {
    unsafe { element_remove_attribute(instance, cstr(name)) }
}
extern "C" {
    fn element_remove_attribute_n_s(instance: i32, namespace: CString, local_name: CString);
}

pub fn remove_attribute_n_s(instance: i32, namespace: &str, local_name: &str) {
    unsafe { element_remove_attribute_n_s(instance, cstr(namespace), cstr(local_name)) }
}
extern "C" {
    fn element_has_attribute(instance: i32, name: CString) -> i32;
}

pub fn has_attribute(instance: i32, name: &str) -> i32 {
    unsafe { element_has_attribute(instance, cstr(name)) }
}
extern "C" {
    fn element_has_attribute_n_s(instance: i32, namespace: CString, local_name: CString) -> i32;
}

pub fn has_attribute_n_s(instance: i32, namespace: &str, local_name: &str) -> i32 {
    unsafe { element_has_attribute_n_s(instance, cstr(namespace), cstr(local_name)) }
}
extern "C" {
    fn element_has_attributes(instance: i32) -> i32;
}

pub fn has_attributes(instance: i32) -> i32 {
    unsafe { element_has_attributes(instance) }
}
extern "C" {
    fn element_closest(instance: i32, selector: CString) -> i32;
}

pub fn closest(instance: i32, selector: &str) -> i32 {
    unsafe { element_closest(instance, cstr(selector)) }
}
extern "C" {
    fn element_matches(instance: i32, selector: CString) -> i32;
}

pub fn matches(instance: i32, selector: &str) -> i32 {
    unsafe { element_matches(instance, cstr(selector)) }
}
extern "C" {
    fn element_webkit_matches_selector(instance: i32, selector: CString) -> i32;
}

pub fn webkit_matches_selector(instance: i32, selector: &str) -> i32 {
    unsafe { element_webkit_matches_selector(instance, cstr(selector)) }
}
extern "C" {
    fn element_get_elements_with_grid(instance: i32) -> i32;
}

pub fn get_elements_with_grid(instance: i32) -> i32 {
    unsafe { element_get_elements_with_grid(instance) }
}
extern "C" {
    fn element_insert_adjacent_element(instance: i32, location: CString, element: i32) -> i32;
}

pub fn insert_adjacent_element(instance: i32, location: &str, element: i32) -> i32 {
    unsafe { element_insert_adjacent_element(instance, cstr(location), element) }
}
extern "C" {
    fn element_insert_adjacent_text(instance: i32, location: CString, data: CString);
}

pub fn insert_adjacent_text(instance: i32, location: &str, data: &str) {
    unsafe { element_insert_adjacent_text(instance, cstr(location), cstr(data)) }
}
extern "C" {
    fn element_get_fontSizeInflation(instance: i32) -> i32;
    fn element_set_fontSizeInflation(instance: i32, value: i32);
}

pub fn get_font_size_inflation(instance: i32) -> i32 {
    unsafe { element_get_fontSizeInflation(instance) }
}

pub fn set_font_size_inflation(instance: i32, value: i32) {
    unsafe {
        element_set_fontSizeInflation(instance, value);
    }
}
extern "C" {
    fn element_set_pointer_capture(instance: i32, pointer_id: i32);
}

pub fn set_pointer_capture(instance: i32, pointer_id: i32) {
    unsafe { element_set_pointer_capture(instance, pointer_id) }
}
extern "C" {
    fn element_release_pointer_capture(instance: i32, pointer_id: i32);
}

pub fn release_pointer_capture(instance: i32, pointer_id: i32) {
    unsafe { element_release_pointer_capture(instance, pointer_id) }
}
extern "C" {
    fn element_has_pointer_capture(instance: i32, pointer_id: i32) -> i32;
}

pub fn has_pointer_capture(instance: i32, pointer_id: i32) -> i32 {
    unsafe { element_has_pointer_capture(instance, pointer_id) }
}
extern "C" {
    fn element_set_capture(instance: i32, retarget_to_element: i32);
}

pub fn set_capture(instance: i32, retarget_to_element: i32) {
    unsafe { element_set_capture(instance, retarget_to_element) }
}
extern "C" {
    fn element_release_capture(instance: i32);
}

pub fn release_capture(instance: i32) {
    unsafe { element_release_capture(instance) }
}
extern "C" {
    fn element_set_capture_always(instance: i32, retarget_to_element: i32);
}

pub fn set_capture_always(instance: i32, retarget_to_element: i32) {
    unsafe { element_set_capture_always(instance, retarget_to_element) }
}
extern "C" {
    fn element_get_attribute_node(instance: i32, name: CString) -> i32;
}

pub fn get_attribute_node(instance: i32, name: &str) -> i32 {
    unsafe { element_get_attribute_node(instance, cstr(name)) }
}
extern "C" {
    fn element_set_attribute_node(instance: i32, new_attr: i32) -> i32;
}

pub fn set_attribute_node(instance: i32, new_attr: i32) -> i32 {
    unsafe { element_set_attribute_node(instance, new_attr) }
}
extern "C" {
    fn element_remove_attribute_node(instance: i32, old_attr: i32) -> i32;
}

pub fn remove_attribute_node(instance: i32, old_attr: i32) -> i32 {
    unsafe { element_remove_attribute_node(instance, old_attr) }
}
extern "C" {
    fn element_get_attribute_node_n_s(
        instance: i32,
        namespace_uri: CString,
        local_name: CString,
    ) -> i32;
}

pub fn get_attribute_node_n_s(instance: i32, namespace_uri: &str, local_name: &str) -> i32 {
    unsafe { element_get_attribute_node_n_s(instance, cstr(namespace_uri), cstr(local_name)) }
}
extern "C" {
    fn element_set_attribute_node_n_s(instance: i32, new_attr: i32) -> i32;
}

pub fn set_attribute_node_n_s(instance: i32, new_attr: i32) -> i32 {
    unsafe { element_set_attribute_node_n_s(instance, new_attr) }
}
extern "C" {
    fn element_scroll_by_no_flush(instance: i32, dx: i32, dy: i32) -> i32;
}

pub fn scroll_by_no_flush(instance: i32, dx: i32, dy: i32) -> i32 {
    unsafe { element_scroll_by_no_flush(instance, dx, dy) }
}
extern "C" {
    fn element_get_as_flex_container(instance: i32) -> i32;
}

pub fn get_as_flex_container(instance: i32) -> i32 {
    unsafe { element_get_as_flex_container(instance) }
}
extern "C" {
    fn element_get_grid_fragments(instance: i32) -> i32;
}

pub fn get_grid_fragments(instance: i32) -> i32 {
    unsafe { element_get_grid_fragments(instance) }
}
extern "C" {
    fn element_get_transform_to_ancestor(instance: i32, ancestor: i32) -> i32;
}

pub fn get_transform_to_ancestor(instance: i32, ancestor: i32) -> i32 {
    unsafe { element_get_transform_to_ancestor(instance, ancestor) }
}
extern "C" {
    fn element_get_transform_to_parent(instance: i32) -> i32;
}

pub fn get_transform_to_parent(instance: i32) -> i32 {
    unsafe { element_get_transform_to_parent(instance) }
}
extern "C" {
    fn element_get_transform_to_viewport(instance: i32) -> i32;
}

pub fn get_transform_to_viewport(instance: i32) -> i32 {
    unsafe { element_get_transform_to_viewport(instance) }
}
extern "C" {
    fn element_get_client_rects(instance: i32) -> i32;
}

pub fn get_client_rects(instance: i32) -> i32 {
    unsafe { element_get_client_rects(instance) }
}
extern "C" {
    fn element_get_bounding_client_rect(instance: i32) -> i32;
}

pub fn get_bounding_client_rect(instance: i32) -> i32 {
    unsafe { element_get_bounding_client_rect(instance) }
}
extern "C" {
    fn element_scroll_into_view(instance: i32, arg: i32);
}

pub fn scroll_into_view(instance: i32, arg: i32) {
    unsafe { element_scroll_into_view(instance, arg) }
}
extern "C" {
    fn element_get_scrollTop(instance: i32) -> i32;
    fn element_set_scrollTop(instance: i32, value: i32);
}

pub fn get_scroll_top(instance: i32) -> i32 {
    unsafe { element_get_scrollTop(instance) }
}

pub fn set_scroll_top(instance: i32, value: i32) {
    unsafe {
        element_set_scrollTop(instance, value);
    }
}
extern "C" {
    fn element_get_scrollLeft(instance: i32) -> i32;
    fn element_set_scrollLeft(instance: i32, value: i32);
}

pub fn get_scroll_left(instance: i32) -> i32 {
    unsafe { element_get_scrollLeft(instance) }
}

pub fn set_scroll_left(instance: i32, value: i32) {
    unsafe {
        element_set_scrollLeft(instance, value);
    }
}
extern "C" {
    fn element_get_scrollWidth(instance: i32) -> i32;
    fn element_set_scrollWidth(instance: i32, value: i32);
}

pub fn get_scroll_width(instance: i32) -> i32 {
    unsafe { element_get_scrollWidth(instance) }
}

pub fn set_scroll_width(instance: i32, value: i32) {
    unsafe {
        element_set_scrollWidth(instance, value);
    }
}
extern "C" {
    fn element_get_scrollHeight(instance: i32) -> i32;
    fn element_set_scrollHeight(instance: i32, value: i32);
}

pub fn get_scroll_height(instance: i32) -> i32 {
    unsafe { element_get_scrollHeight(instance) }
}

pub fn set_scroll_height(instance: i32, value: i32) {
    unsafe {
        element_set_scrollHeight(instance, value);
    }
}
extern "C" {
    fn element_scroll(instance: i32, x: i32, y: i32);
}

pub fn scroll(instance: i32, x: i32, y: i32) {
    unsafe { element_scroll(instance, x, y) }
}
extern "C" {
    fn element_scroll_to(instance: i32, x: i32, y: i32);
}

pub fn scroll_to(instance: i32, x: i32, y: i32) {
    unsafe { element_scroll_to(instance, x, y) }
}
extern "C" {
    fn element_scroll_by(instance: i32, x: i32, y: i32);
}

pub fn scroll_by(instance: i32, x: i32, y: i32) {
    unsafe { element_scroll_by(instance, x, y) }
}
extern "C" {
    fn element_get_clientTop(instance: i32) -> i32;
    fn element_set_clientTop(instance: i32, value: i32);
}

pub fn get_client_top(instance: i32) -> i32 {
    unsafe { element_get_clientTop(instance) }
}

pub fn set_client_top(instance: i32, value: i32) {
    unsafe {
        element_set_clientTop(instance, value);
    }
}
extern "C" {
    fn element_get_clientLeft(instance: i32) -> i32;
    fn element_set_clientLeft(instance: i32, value: i32);
}

pub fn get_client_left(instance: i32) -> i32 {
    unsafe { element_get_clientLeft(instance) }
}

pub fn set_client_left(instance: i32, value: i32) {
    unsafe {
        element_set_clientLeft(instance, value);
    }
}
extern "C" {
    fn element_get_clientWidth(instance: i32) -> i32;
    fn element_set_clientWidth(instance: i32, value: i32);
}

pub fn get_client_width(instance: i32) -> i32 {
    unsafe { element_get_clientWidth(instance) }
}

pub fn set_client_width(instance: i32, value: i32) {
    unsafe {
        element_set_clientWidth(instance, value);
    }
}
extern "C" {
    fn element_get_clientHeight(instance: i32) -> i32;
    fn element_set_clientHeight(instance: i32, value: i32);
}

pub fn get_client_height(instance: i32) -> i32 {
    unsafe { element_get_clientHeight(instance) }
}

pub fn set_client_height(instance: i32, value: i32) {
    unsafe {
        element_set_clientHeight(instance, value);
    }
}
extern "C" {
    fn element_get_innerHTML(instance: i32) -> CString;
    fn element_set_innerHTML(instance: i32, value: CString);
}

pub fn get_inner_html(instance: i32) -> String {
    unsafe { cstr_to_string(element_get_innerHTML(instance)) }
}

pub fn set_inner_html(instance: i32, value: &str) {
    unsafe {
        element_set_innerHTML(instance, cstr(value));
    }
}
extern "C" {
    fn element_get_outerHTML(instance: i32) -> CString;
    fn element_set_outerHTML(instance: i32, value: CString);
}

pub fn get_outer_html(instance: i32) -> String {
    unsafe { cstr_to_string(element_get_outerHTML(instance)) }
}

pub fn set_outer_html(instance: i32, value: &str) {
    unsafe {
        element_set_outerHTML(instance, cstr(value));
    }
}
extern "C" {
    fn element_insert_adjacent_html(instance: i32, position: CString, text: CString);
}

pub fn insert_adjacent_html(instance: i32, position: &str, text: &str) {
    unsafe { element_insert_adjacent_html(instance, cstr(position), cstr(text)) }
}
extern "C" {
    fn element_query_selector(instance: i32, selectors: CString) -> i32;
}

pub fn query_selector(instance: i32, selectors: &str) -> i32 {
    unsafe { element_query_selector(instance, cstr(selectors)) }
}
extern "C" {
    fn element_query_selector_all(instance: i32, selectors: CString) -> i32;
}

pub fn query_selector_all(instance: i32, selectors: &str) -> i32 {
    unsafe { element_query_selector_all(instance, cstr(selectors)) }
}
extern "C" {
    fn element_get_shadowRoot(instance: i32) -> i32;
    fn element_set_shadowRoot(instance: i32, value: i32);
}

pub fn get_shadow_root(instance: i32) -> i32 {
    unsafe { element_get_shadowRoot(instance) }
}

pub fn set_shadow_root(instance: i32, value: i32) {
    unsafe {
        element_set_shadowRoot(instance, value);
    }
}
extern "C" {
    fn element_get_openOrClosedShadowRoot(instance: i32) -> i32;
    fn element_set_openOrClosedShadowRoot(instance: i32, value: i32);
}

pub fn get_open_or_closed_shadow_root(instance: i32) -> i32 {
    unsafe { element_get_openOrClosedShadowRoot(instance) }
}

pub fn set_open_or_closed_shadow_root(instance: i32, value: i32) {
    unsafe {
        element_set_openOrClosedShadowRoot(instance, value);
    }
}
extern "C" {
    fn element_get_assignedSlot(instance: i32) -> i32;
    fn element_set_assignedSlot(instance: i32, value: i32);
}

pub fn get_assigned_slot(instance: i32) -> i32 {
    unsafe { element_get_assignedSlot(instance) }
}

pub fn set_assigned_slot(instance: i32, value: i32) {
    unsafe {
        element_set_assignedSlot(instance, value);
    }
}
extern "C" {
    fn element_get_slot(instance: i32) -> CString;
    fn element_set_slot(instance: i32, value: CString);
}

pub fn get_slot(instance: i32) -> String {
    unsafe { cstr_to_string(element_get_slot(instance)) }
}

pub fn set_slot(instance: i32, value: &str) {
    unsafe {
        element_set_slot(instance, cstr(value));
    }
}
extern "C" {
    fn element_request_fullscreen(instance: i32);
}

pub fn request_fullscreen(instance: i32) {
    unsafe { element_request_fullscreen(instance) }
}
extern "C" {
    fn element_request_pointer_lock(instance: i32);
}

pub fn request_pointer_lock(instance: i32) {
    unsafe { element_request_pointer_lock(instance) }
}
