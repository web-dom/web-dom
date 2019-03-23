#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn element_get_namespace_uri(instance: DOMReference) -> CString;
    fn element_set_namespace_uri(instance: DOMReference, value: CString);
}

pub fn get_namespace_uri(instance: DOMReference) -> String {
    unsafe { to_string(element_get_namespace_uri(instance)) }
}

pub fn set_namespace_uri(instance: DOMReference, value: &str) {
    unsafe {
        element_set_namespace_uri(instance, to_cstring(value));
    }
}
extern "C" {
    fn element_get_prefix(instance: DOMReference) -> CString;
    fn element_set_prefix(instance: DOMReference, value: CString);
}

pub fn get_prefix(instance: DOMReference) -> String {
    unsafe { to_string(element_get_prefix(instance)) }
}

pub fn set_prefix(instance: DOMReference, value: &str) {
    unsafe {
        element_set_prefix(instance, to_cstring(value));
    }
}
extern "C" {
    fn element_get_local_name(instance: DOMReference) -> CString;
    fn element_set_local_name(instance: DOMReference, value: CString);
}

pub fn get_local_name(instance: DOMReference) -> String {
    unsafe { to_string(element_get_local_name(instance)) }
}

pub fn set_local_name(instance: DOMReference, value: &str) {
    unsafe {
        element_set_local_name(instance, to_cstring(value));
    }
}
extern "C" {
    fn element_get_tag_name(instance: DOMReference) -> CString;
    fn element_set_tag_name(instance: DOMReference, value: CString);
}

pub fn get_tag_name(instance: DOMReference) -> String {
    unsafe { to_string(element_get_tag_name(instance)) }
}

pub fn set_tag_name(instance: DOMReference, value: &str) {
    unsafe {
        element_set_tag_name(instance, to_cstring(value));
    }
}
extern "C" {
    fn element_get_id(instance: DOMReference) -> CString;
    fn element_set_id(instance: DOMReference, value: CString);
}

pub fn get_id(instance: DOMReference) -> String {
    unsafe { to_string(element_get_id(instance)) }
}

pub fn set_id(instance: DOMReference, value: &str) {
    unsafe {
        element_set_id(instance, to_cstring(value));
    }
}
extern "C" {
    fn element_get_class_name(instance: DOMReference) -> CString;
    fn element_set_class_name(instance: DOMReference, value: CString);
}

pub fn get_class_name(instance: DOMReference) -> String {
    unsafe { to_string(element_get_class_name(instance)) }
}

pub fn set_class_name(instance: DOMReference, value: &str) {
    unsafe {
        element_set_class_name(instance, to_cstring(value));
    }
}
extern "C" {
    fn element_get_class_list(instance: DOMReference) -> i32;
    fn element_set_class_list(instance: DOMReference, value: i32);
}

pub fn get_class_list(instance: DOMReference) -> i32 {
    unsafe { element_get_class_list(instance) }
}

pub fn set_class_list(instance: DOMReference, value: i32) {
    unsafe {
        element_set_class_list(instance, value);
    }
}
extern "C" {
    fn element_get_attributes(instance: DOMReference) -> i32;
    fn element_set_attributes(instance: DOMReference, value: i32);
}

pub fn get_attributes(instance: DOMReference) -> i32 {
    unsafe { element_get_attributes(instance) }
}

pub fn set_attributes(instance: DOMReference, value: i32) {
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
    unsafe { to_string(element_get_attribute(instance, to_cstring(name))) }
}
extern "C" {
    fn element_get_attribute_n_s(instance: i32, namespace: CString, local_name: CString)
        -> CString;
}

pub fn get_attribute_n_s(instance: i32, namespace: &str, local_name: &str) -> String {
    unsafe {
        to_string(element_get_attribute_n_s(
            instance,
            to_cstring(namespace),
            to_cstring(local_name),
        ))
    }
}
extern "C" {
    fn element_toggle_attribute(instance: i32, name: CString, force: i32) -> i32;
}

pub fn toggle_attribute(instance: i32, name: &str, force: i32) -> i32 {
    unsafe { element_toggle_attribute(instance, to_cstring(name), force) }
}
extern "C" {
    fn element_set_attribute(instance: i32, name: CString, value: CString);
}

pub fn set_attribute(instance: i32, name: &str, value: &str) {
    unsafe { element_set_attribute(instance, to_cstring(name), to_cstring(value)) }
}
extern "C" {
    fn element_set_attribute_n_s(instance: i32, namespace: CString, name: CString, value: CString);
}

pub fn set_attribute_n_s(instance: i32, namespace: &str, name: &str, value: &str) {
    unsafe {
        element_set_attribute_n_s(
            instance,
            to_cstring(namespace),
            to_cstring(name),
            to_cstring(value),
        )
    }
}
extern "C" {
    fn element_remove_attribute(instance: i32, name: CString);
}

pub fn remove_attribute(instance: i32, name: &str) {
    unsafe { element_remove_attribute(instance, to_cstring(name)) }
}
extern "C" {
    fn element_remove_attribute_n_s(instance: i32, namespace: CString, local_name: CString);
}

pub fn remove_attribute_n_s(instance: i32, namespace: &str, local_name: &str) {
    unsafe { element_remove_attribute_n_s(instance, to_cstring(namespace), to_cstring(local_name)) }
}
extern "C" {
    fn element_has_attribute(instance: i32, name: CString) -> i32;
}

pub fn has_attribute(instance: i32, name: &str) -> i32 {
    unsafe { element_has_attribute(instance, to_cstring(name)) }
}
extern "C" {
    fn element_has_attribute_n_s(instance: i32, namespace: CString, local_name: CString) -> i32;
}

pub fn has_attribute_n_s(instance: i32, namespace: &str, local_name: &str) -> i32 {
    unsafe { element_has_attribute_n_s(instance, to_cstring(namespace), to_cstring(local_name)) }
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
    unsafe { element_closest(instance, to_cstring(selector)) }
}
extern "C" {
    fn element_matches(instance: i32, selector: CString) -> i32;
}

pub fn matches(instance: i32, selector: &str) -> i32 {
    unsafe { element_matches(instance, to_cstring(selector)) }
}
extern "C" {
    fn element_webkit_matches_selector(instance: i32, selector: CString) -> i32;
}

pub fn webkit_matches_selector(instance: i32, selector: &str) -> i32 {
    unsafe { element_webkit_matches_selector(instance, to_cstring(selector)) }
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
    unsafe { element_insert_adjacent_element(instance, to_cstring(location), element) }
}
extern "C" {
    fn element_insert_adjacent_text(instance: i32, location: CString, data: CString);
}

pub fn insert_adjacent_text(instance: i32, location: &str, data: &str) {
    unsafe { element_insert_adjacent_text(instance, to_cstring(location), to_cstring(data)) }
}
extern "C" {
    fn element_get_font_size_inflation(instance: DOMReference) -> i32;
    fn element_set_font_size_inflation(instance: DOMReference, value: i32);
}

pub fn get_font_size_inflation(instance: DOMReference) -> i32 {
    unsafe { element_get_font_size_inflation(instance) }
}

pub fn set_font_size_inflation(instance: DOMReference, value: i32) {
    unsafe {
        element_set_font_size_inflation(instance, value);
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
    unsafe { element_get_attribute_node(instance, to_cstring(name)) }
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
    unsafe {
        element_get_attribute_node_n_s(instance, to_cstring(namespace_uri), to_cstring(local_name))
    }
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
    fn element_get_scroll_top(instance: DOMReference) -> i32;
    fn element_set_scroll_top(instance: DOMReference, value: i32);
}

pub fn get_scroll_top(instance: DOMReference) -> i32 {
    unsafe { element_get_scroll_top(instance) }
}

pub fn set_scroll_top(instance: DOMReference, value: i32) {
    unsafe {
        element_set_scroll_top(instance, value);
    }
}
extern "C" {
    fn element_get_scroll_left(instance: DOMReference) -> i32;
    fn element_set_scroll_left(instance: DOMReference, value: i32);
}

pub fn get_scroll_left(instance: DOMReference) -> i32 {
    unsafe { element_get_scroll_left(instance) }
}

pub fn set_scroll_left(instance: DOMReference, value: i32) {
    unsafe {
        element_set_scroll_left(instance, value);
    }
}
extern "C" {
    fn element_get_scroll_width(instance: DOMReference) -> i32;
    fn element_set_scroll_width(instance: DOMReference, value: i32);
}

pub fn get_scroll_width(instance: DOMReference) -> i32 {
    unsafe { element_get_scroll_width(instance) }
}

pub fn set_scroll_width(instance: DOMReference, value: i32) {
    unsafe {
        element_set_scroll_width(instance, value);
    }
}
extern "C" {
    fn element_get_scroll_height(instance: DOMReference) -> i32;
    fn element_set_scroll_height(instance: DOMReference, value: i32);
}

pub fn get_scroll_height(instance: DOMReference) -> i32 {
    unsafe { element_get_scroll_height(instance) }
}

pub fn set_scroll_height(instance: DOMReference, value: i32) {
    unsafe {
        element_set_scroll_height(instance, value);
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
    fn element_get_client_top(instance: DOMReference) -> i32;
    fn element_set_client_top(instance: DOMReference, value: i32);
}

pub fn get_client_top(instance: DOMReference) -> i32 {
    unsafe { element_get_client_top(instance) }
}

pub fn set_client_top(instance: DOMReference, value: i32) {
    unsafe {
        element_set_client_top(instance, value);
    }
}
extern "C" {
    fn element_get_client_left(instance: DOMReference) -> i32;
    fn element_set_client_left(instance: DOMReference, value: i32);
}

pub fn get_client_left(instance: DOMReference) -> i32 {
    unsafe { element_get_client_left(instance) }
}

pub fn set_client_left(instance: DOMReference, value: i32) {
    unsafe {
        element_set_client_left(instance, value);
    }
}
extern "C" {
    fn element_get_client_width(instance: DOMReference) -> i32;
    fn element_set_client_width(instance: DOMReference, value: i32);
}

pub fn get_client_width(instance: DOMReference) -> i32 {
    unsafe { element_get_client_width(instance) }
}

pub fn set_client_width(instance: DOMReference, value: i32) {
    unsafe {
        element_set_client_width(instance, value);
    }
}
extern "C" {
    fn element_get_client_height(instance: DOMReference) -> i32;
    fn element_set_client_height(instance: DOMReference, value: i32);
}

pub fn get_client_height(instance: DOMReference) -> i32 {
    unsafe { element_get_client_height(instance) }
}

pub fn set_client_height(instance: DOMReference, value: i32) {
    unsafe {
        element_set_client_height(instance, value);
    }
}
extern "C" {
    fn element_get_inner_html(instance: DOMReference) -> CString;
    fn element_set_inner_html(instance: DOMReference, value: CString);
}

pub fn get_inner_html(instance: DOMReference) -> String {
    unsafe { to_string(element_get_inner_html(instance)) }
}

pub fn set_inner_html(instance: DOMReference, value: &str) {
    unsafe {
        element_set_inner_html(instance, to_cstring(value));
    }
}
extern "C" {
    fn element_get_outer_html(instance: DOMReference) -> CString;
    fn element_set_outer_html(instance: DOMReference, value: CString);
}

pub fn get_outer_html(instance: DOMReference) -> String {
    unsafe { to_string(element_get_outer_html(instance)) }
}

pub fn set_outer_html(instance: DOMReference, value: &str) {
    unsafe {
        element_set_outer_html(instance, to_cstring(value));
    }
}
extern "C" {
    fn element_insert_adjacent_html(instance: i32, position: CString, text: CString);
}

pub fn insert_adjacent_html(instance: i32, position: &str, text: &str) {
    unsafe { element_insert_adjacent_html(instance, to_cstring(position), to_cstring(text)) }
}
extern "C" {
    fn element_query_selector(instance: i32, selectors: CString) -> i32;
}

pub fn query_selector(instance: i32, selectors: &str) -> i32 {
    unsafe { element_query_selector(instance, to_cstring(selectors)) }
}
extern "C" {
    fn element_query_selector_all(instance: i32, selectors: CString) -> i32;
}

pub fn query_selector_all(instance: i32, selectors: &str) -> i32 {
    unsafe { element_query_selector_all(instance, to_cstring(selectors)) }
}
extern "C" {
    fn element_get_shadow_root(instance: DOMReference) -> i32;
    fn element_set_shadow_root(instance: DOMReference, value: i32);
}

pub fn get_shadow_root(instance: DOMReference) -> i32 {
    unsafe { element_get_shadow_root(instance) }
}

pub fn set_shadow_root(instance: DOMReference, value: i32) {
    unsafe {
        element_set_shadow_root(instance, value);
    }
}
extern "C" {
    fn element_get_open_or_closed_shadow_root(instance: DOMReference) -> i32;
    fn element_set_open_or_closed_shadow_root(instance: DOMReference, value: i32);
}

pub fn get_open_or_closed_shadow_root(instance: DOMReference) -> i32 {
    unsafe { element_get_open_or_closed_shadow_root(instance) }
}

pub fn set_open_or_closed_shadow_root(instance: DOMReference, value: i32) {
    unsafe {
        element_set_open_or_closed_shadow_root(instance, value);
    }
}
extern "C" {
    fn element_get_assigned_slot(instance: DOMReference) -> i32;
    fn element_set_assigned_slot(instance: DOMReference, value: i32);
}

pub fn get_assigned_slot(instance: DOMReference) -> i32 {
    unsafe { element_get_assigned_slot(instance) }
}

pub fn set_assigned_slot(instance: DOMReference, value: i32) {
    unsafe {
        element_set_assigned_slot(instance, value);
    }
}
extern "C" {
    fn element_get_slot(instance: DOMReference) -> CString;
    fn element_set_slot(instance: DOMReference, value: CString);
}

pub fn get_slot(instance: DOMReference) -> String {
    unsafe { to_string(element_get_slot(instance)) }
}

pub fn set_slot(instance: DOMReference, value: &str) {
    unsafe {
        element_set_slot(instance, to_cstring(value));
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
extern "C" {
    fn element_get_children(instance: DOMReference) -> i32;
    fn element_set_children(instance: DOMReference, value: i32);
}

pub fn get_children(instance: DOMReference) -> i32 {
    unsafe { element_get_children(instance) }
}

pub fn set_children(instance: DOMReference, value: i32) {
    unsafe {
        element_set_children(instance, value);
    }
}
extern "C" {
    fn element_get_first_element_child(instance: DOMReference) -> i32;
    fn element_set_first_element_child(instance: DOMReference, value: i32);
}

pub fn get_first_element_child(instance: DOMReference) -> i32 {
    unsafe { element_get_first_element_child(instance) }
}

pub fn set_first_element_child(instance: DOMReference, value: i32) {
    unsafe {
        element_set_first_element_child(instance, value);
    }
}
extern "C" {
    fn element_get_last_element_child(instance: DOMReference) -> i32;
    fn element_set_last_element_child(instance: DOMReference, value: i32);
}

pub fn get_last_element_child(instance: DOMReference) -> i32 {
    unsafe { element_get_last_element_child(instance) }
}

pub fn set_last_element_child(instance: DOMReference, value: i32) {
    unsafe {
        element_set_last_element_child(instance, value);
    }
}
extern "C" {
    fn element_get_child_element_count(instance: DOMReference) -> i32;
    fn element_set_child_element_count(instance: DOMReference, value: i32);
}

pub fn get_child_element_count(instance: DOMReference) -> i32 {
    unsafe { element_get_child_element_count(instance) }
}

pub fn set_child_element_count(instance: DOMReference, value: i32) {
    unsafe {
        element_set_child_element_count(instance, value);
    }
}
extern "C" {
    fn element_prepend(instance: i32, nodes: i32);
}

pub fn prepend(instance: i32, nodes: i32) {
    unsafe { element_prepend(instance, nodes) }
}
extern "C" {
    fn element_append(instance: i32, nodes: i32);
}

pub fn append(instance: i32, nodes: i32) {
    unsafe { element_append(instance, nodes) }
}
extern "C" {
    fn element_before(instance: i32, nodes: i32);
}

pub fn before(instance: i32, nodes: i32) {
    unsafe { element_before(instance, nodes) }
}
extern "C" {
    fn element_after(instance: i32, nodes: i32);
}

pub fn after(instance: i32, nodes: i32) {
    unsafe { element_after(instance, nodes) }
}
extern "C" {
    fn element_replace_with(instance: i32, nodes: i32);
}

pub fn replace_with(instance: i32, nodes: i32) {
    unsafe { element_replace_with(instance, nodes) }
}
extern "C" {
    fn element_remove(instance: i32);
}

pub fn remove(instance: i32) {
    unsafe { element_remove(instance) }
}
extern "C" {
    fn element_get_previous_element_sibling(instance: DOMReference) -> i32;
    fn element_set_previous_element_sibling(instance: DOMReference, value: i32);
}

pub fn get_previous_element_sibling(instance: DOMReference) -> i32 {
    unsafe { element_get_previous_element_sibling(instance) }
}

pub fn set_previous_element_sibling(instance: DOMReference, value: i32) {
    unsafe {
        element_set_previous_element_sibling(instance, value);
    }
}
extern "C" {
    fn element_get_next_element_sibling(instance: DOMReference) -> i32;
    fn element_set_next_element_sibling(instance: DOMReference, value: i32);
}

pub fn get_next_element_sibling(instance: DOMReference) -> i32 {
    unsafe { element_get_next_element_sibling(instance) }
}

pub fn set_next_element_sibling(instance: DOMReference, value: i32) {
    unsafe {
        element_set_next_element_sibling(instance, value);
    }
}
