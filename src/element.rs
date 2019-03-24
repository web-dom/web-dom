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
    fn element_get_class_list(instance: DOMReference) -> DOMReference;
    fn element_set_class_list(instance: DOMReference, value: DOMReference);
}

pub fn get_class_list(instance: DOMReference) -> DOMReference {
    unsafe { element_get_class_list(instance) }
}

pub fn set_class_list(instance: DOMReference, value: DOMReference) {
    unsafe {
        element_set_class_list(instance, value);
    }
}
extern "C" {
    fn element_get_attributes(instance: DOMReference) -> DOMReference;
    fn element_set_attributes(instance: DOMReference, value: DOMReference);
}

pub fn get_attributes(instance: DOMReference) -> DOMReference {
    unsafe { element_get_attributes(instance) }
}

pub fn set_attributes(instance: DOMReference, value: DOMReference) {
    unsafe {
        element_set_attributes(instance, value);
    }
}
extern "C" {
    fn element_get_attribute_names(instance: DOMReference) -> DOMReference;
}

pub fn get_attribute_names(instance: DOMReference) -> DOMReference {
    unsafe { element_get_attribute_names(instance) }
}
extern "C" {
    fn element_get_attribute(instance: DOMReference, get_attribute: CString) -> CString;
}

pub fn get_attribute(instance: DOMReference, name: &str) -> String {
    unsafe { to_string(element_get_attribute(instance, to_cstring(name))) }
}
extern "C" {
    fn element_get_attribute_n_s(
        instance: DOMReference,
        get_attribute_n_s: CString,
        get_attribute_n_s: CString,
    ) -> CString;
}

pub fn get_attribute_n_s(instance: DOMReference, namespace: &str, local_name: &str) -> String {
    unsafe {
        to_string(element_get_attribute_n_s(
            instance,
            to_cstring(namespace),
            to_cstring(local_name),
        ))
    }
}
extern "C" {
    fn element_toggle_attribute(
        instance: DOMReference,
        toggle_attribute: CString,
        toggle_attribute: i32,
    ) -> i32;
}

pub fn toggle_attribute(instance: DOMReference, name: &str, force: bool) -> bool {
    unsafe { 0 != element_toggle_attribute(instance, to_cstring(name), if force { 1 } else { 0 }) }
}
extern "C" {
    fn element_set_attribute(
        instance: DOMReference,
        set_attribute: CString,
        set_attribute: CString,
    );
}

pub fn set_attribute(instance: DOMReference, name: &str, value: &str) {
    unsafe { element_set_attribute(instance, to_cstring(name), to_cstring(value)) }
}
extern "C" {
    fn element_set_attribute_n_s(
        instance: DOMReference,
        set_attribute_n_s: CString,
        set_attribute_n_s: CString,
        set_attribute_n_s: CString,
    );
}

pub fn set_attribute_n_s(instance: DOMReference, namespace: &str, name: &str, value: &str) {
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
    fn element_remove_attribute(instance: DOMReference, remove_attribute: CString);
}

pub fn remove_attribute(instance: DOMReference, name: &str) {
    unsafe { element_remove_attribute(instance, to_cstring(name)) }
}
extern "C" {
    fn element_remove_attribute_n_s(
        instance: DOMReference,
        remove_attribute_n_s: CString,
        remove_attribute_n_s: CString,
    );
}

pub fn remove_attribute_n_s(instance: DOMReference, namespace: &str, local_name: &str) {
    unsafe { element_remove_attribute_n_s(instance, to_cstring(namespace), to_cstring(local_name)) }
}
extern "C" {
    fn element_has_attribute(instance: DOMReference, has_attribute: CString) -> i32;
}

pub fn has_attribute(instance: DOMReference, name: &str) -> bool {
    unsafe { 0 != element_has_attribute(instance, to_cstring(name)) }
}
extern "C" {
    fn element_has_attribute_n_s(
        instance: DOMReference,
        has_attribute_n_s: CString,
        has_attribute_n_s: CString,
    ) -> i32;
}

pub fn has_attribute_n_s(instance: DOMReference, namespace: &str, local_name: &str) -> bool {
    unsafe {
        0 != element_has_attribute_n_s(instance, to_cstring(namespace), to_cstring(local_name))
    }
}
extern "C" {
    fn element_has_attributes(instance: DOMReference) -> i32;
}

pub fn has_attributes(instance: DOMReference) -> bool {
    unsafe { 0 != element_has_attributes(instance) }
}
extern "C" {
    fn element_closest(instance: DOMReference, closest: CString) -> DOMReference;
}

pub fn closest(instance: DOMReference, selector: &str) -> DOMReference {
    unsafe { element_closest(instance, to_cstring(selector)) }
}
extern "C" {
    fn element_matches(instance: DOMReference, matches: CString) -> i32;
}

pub fn matches(instance: DOMReference, selector: &str) -> bool {
    unsafe { 0 != element_matches(instance, to_cstring(selector)) }
}
extern "C" {
    fn element_webkit_matches_selector(
        instance: DOMReference,
        webkit_matches_selector: CString,
    ) -> i32;
}

pub fn webkit_matches_selector(instance: DOMReference, selector: &str) -> bool {
    unsafe { 0 != element_webkit_matches_selector(instance, to_cstring(selector)) }
}
extern "C" {
    fn element_get_elements_with_grid(instance: DOMReference) -> DOMReference;
}

pub fn get_elements_with_grid(instance: DOMReference) -> DOMReference {
    unsafe { element_get_elements_with_grid(instance) }
}
extern "C" {
    fn element_insert_adjacent_element(
        instance: DOMReference,
        insert_adjacent_element: CString,
        insert_adjacent_element: DOMReference,
    ) -> DOMReference;
}

pub fn insert_adjacent_element(
    instance: DOMReference,
    location: &str,
    element: DOMReference,
) -> DOMReference {
    unsafe { element_insert_adjacent_element(instance, to_cstring(location), element) }
}
extern "C" {
    fn element_insert_adjacent_text(
        instance: DOMReference,
        insert_adjacent_text: CString,
        insert_adjacent_text: CString,
    );
}

pub fn insert_adjacent_text(instance: DOMReference, location: &str, data: &str) {
    unsafe { element_insert_adjacent_text(instance, to_cstring(location), to_cstring(data)) }
}
extern "C" {
    fn element_get_font_size_inflation(instance: DOMReference) -> f32;
    fn element_set_font_size_inflation(instance: DOMReference, value: f32);
}

pub fn get_font_size_inflation(instance: DOMReference) -> f32 {
    unsafe { element_get_font_size_inflation(instance) }
}

pub fn set_font_size_inflation(instance: DOMReference, value: f32) {
    unsafe {
        element_set_font_size_inflation(instance, value);
    }
}
extern "C" {
    fn element_set_pointer_capture(instance: DOMReference, set_pointer_capture: f32);
}

pub fn set_pointer_capture(instance: DOMReference, pointer_id: f32) {
    unsafe { element_set_pointer_capture(instance, pointer_id) }
}
extern "C" {
    fn element_release_pointer_capture(instance: DOMReference, release_pointer_capture: f32);
}

pub fn release_pointer_capture(instance: DOMReference, pointer_id: f32) {
    unsafe { element_release_pointer_capture(instance, pointer_id) }
}
extern "C" {
    fn element_has_pointer_capture(instance: DOMReference, has_pointer_capture: f32) -> i32;
}

pub fn has_pointer_capture(instance: DOMReference, pointer_id: f32) -> bool {
    unsafe { 0 != element_has_pointer_capture(instance, pointer_id) }
}
extern "C" {
    fn element_set_capture(instance: DOMReference, set_capture: i32);
}

pub fn set_capture(instance: DOMReference, retarget_to_element: bool) {
    unsafe { element_set_capture(instance, if retarget_to_element { 1 } else { 0 }) }
}
extern "C" {
    fn element_release_capture(instance: DOMReference);
}

pub fn release_capture(instance: DOMReference) {
    unsafe { element_release_capture(instance) }
}
extern "C" {
    fn element_set_capture_always(instance: DOMReference, set_capture_always: i32);
}

pub fn set_capture_always(instance: DOMReference, retarget_to_element: bool) {
    unsafe { element_set_capture_always(instance, if retarget_to_element { 1 } else { 0 }) }
}
extern "C" {
    fn element_get_attribute_node(
        instance: DOMReference,
        get_attribute_node: CString,
    ) -> DOMReference;
}

pub fn get_attribute_node(instance: DOMReference, name: &str) -> DOMReference {
    unsafe { element_get_attribute_node(instance, to_cstring(name)) }
}
extern "C" {
    fn element_set_attribute_node(
        instance: DOMReference,
        set_attribute_node: DOMReference,
    ) -> DOMReference;
}

pub fn set_attribute_node(instance: DOMReference, new_attr: DOMReference) -> DOMReference {
    unsafe { element_set_attribute_node(instance, new_attr) }
}
extern "C" {
    fn element_remove_attribute_node(
        instance: DOMReference,
        remove_attribute_node: DOMReference,
    ) -> DOMReference;
}

pub fn remove_attribute_node(instance: DOMReference, old_attr: DOMReference) -> DOMReference {
    unsafe { element_remove_attribute_node(instance, old_attr) }
}
extern "C" {
    fn element_get_attribute_node_n_s(
        instance: DOMReference,
        get_attribute_node_n_s: CString,
        get_attribute_node_n_s: CString,
    ) -> DOMReference;
}

pub fn get_attribute_node_n_s(
    instance: DOMReference,
    namespace_uri: &str,
    local_name: &str,
) -> DOMReference {
    unsafe {
        element_get_attribute_node_n_s(instance, to_cstring(namespace_uri), to_cstring(local_name))
    }
}
extern "C" {
    fn element_set_attribute_node_n_s(
        instance: DOMReference,
        set_attribute_node_n_s: DOMReference,
    ) -> DOMReference;
}

pub fn set_attribute_node_n_s(instance: DOMReference, new_attr: DOMReference) -> DOMReference {
    unsafe { element_set_attribute_node_n_s(instance, new_attr) }
}
extern "C" {
    fn element_scroll_by_no_flush(
        instance: DOMReference,
        scroll_by_no_flush: f32,
        scroll_by_no_flush: f32,
    ) -> i32;
}

pub fn scroll_by_no_flush(instance: DOMReference, dx: f32, dy: f32) -> bool {
    unsafe { 0 != element_scroll_by_no_flush(instance, dx, dy) }
}
extern "C" {
    fn element_get_as_flex_container(instance: DOMReference) -> DOMReference;
}

pub fn get_as_flex_container(instance: DOMReference) -> DOMReference {
    unsafe { element_get_as_flex_container(instance) }
}
extern "C" {
    fn element_get_grid_fragments(instance: DOMReference) -> DOMReference;
}

pub fn get_grid_fragments(instance: DOMReference) -> DOMReference {
    unsafe { element_get_grid_fragments(instance) }
}
extern "C" {
    fn element_get_transform_to_ancestor(
        instance: DOMReference,
        get_transform_to_ancestor: DOMReference,
    ) -> DOMReference;
}

pub fn get_transform_to_ancestor(instance: DOMReference, ancestor: DOMReference) -> DOMReference {
    unsafe { element_get_transform_to_ancestor(instance, ancestor) }
}
extern "C" {
    fn element_get_transform_to_parent(instance: DOMReference) -> DOMReference;
}

pub fn get_transform_to_parent(instance: DOMReference) -> DOMReference {
    unsafe { element_get_transform_to_parent(instance) }
}
extern "C" {
    fn element_get_transform_to_viewport(instance: DOMReference) -> DOMReference;
}

pub fn get_transform_to_viewport(instance: DOMReference) -> DOMReference {
    unsafe { element_get_transform_to_viewport(instance) }
}
extern "C" {
    fn element_get_client_rects(instance: DOMReference) -> DOMReference;
}

pub fn get_client_rects(instance: DOMReference) -> DOMReference {
    unsafe { element_get_client_rects(instance) }
}
extern "C" {
    fn element_get_bounding_client_rect(instance: DOMReference) -> DOMReference;
}

pub fn get_bounding_client_rect(instance: DOMReference) -> DOMReference {
    unsafe { element_get_bounding_client_rect(instance) }
}
extern "C" {
    fn element_scroll_into_view(instance: DOMReference, scroll_into_view: DOMReference);
}

pub fn scroll_into_view(instance: DOMReference, arg: DOMReference) {
    unsafe { element_scroll_into_view(instance, arg) }
}
extern "C" {
    fn element_get_scroll_top(instance: DOMReference) -> f32;
    fn element_set_scroll_top(instance: DOMReference, value: f32);
}

pub fn get_scroll_top(instance: DOMReference) -> f32 {
    unsafe { element_get_scroll_top(instance) }
}

pub fn set_scroll_top(instance: DOMReference, value: f32) {
    unsafe {
        element_set_scroll_top(instance, value);
    }
}
extern "C" {
    fn element_get_scroll_left(instance: DOMReference) -> f32;
    fn element_set_scroll_left(instance: DOMReference, value: f32);
}

pub fn get_scroll_left(instance: DOMReference) -> f32 {
    unsafe { element_get_scroll_left(instance) }
}

pub fn set_scroll_left(instance: DOMReference, value: f32) {
    unsafe {
        element_set_scroll_left(instance, value);
    }
}
extern "C" {
    fn element_get_scroll_width(instance: DOMReference) -> f32;
    fn element_set_scroll_width(instance: DOMReference, value: f32);
}

pub fn get_scroll_width(instance: DOMReference) -> f32 {
    unsafe { element_get_scroll_width(instance) }
}

pub fn set_scroll_width(instance: DOMReference, value: f32) {
    unsafe {
        element_set_scroll_width(instance, value);
    }
}
extern "C" {
    fn element_get_scroll_height(instance: DOMReference) -> f32;
    fn element_set_scroll_height(instance: DOMReference, value: f32);
}

pub fn get_scroll_height(instance: DOMReference) -> f32 {
    unsafe { element_get_scroll_height(instance) }
}

pub fn set_scroll_height(instance: DOMReference, value: f32) {
    unsafe {
        element_set_scroll_height(instance, value);
    }
}
extern "C" {
    fn element_scroll(instance: DOMReference, scroll: f32, scroll: f32);
}

pub fn scroll(instance: DOMReference, x: f32, y: f32) {
    unsafe { element_scroll(instance, x, y) }
}
extern "C" {
    fn element_scroll_to(instance: DOMReference, scroll_to: f32, scroll_to: f32);
}

pub fn scroll_to(instance: DOMReference, x: f32, y: f32) {
    unsafe { element_scroll_to(instance, x, y) }
}
extern "C" {
    fn element_scroll_by(instance: DOMReference, scroll_by: f32, scroll_by: f32);
}

pub fn scroll_by(instance: DOMReference, x: f32, y: f32) {
    unsafe { element_scroll_by(instance, x, y) }
}
extern "C" {
    fn element_get_client_top(instance: DOMReference) -> f32;
    fn element_set_client_top(instance: DOMReference, value: f32);
}

pub fn get_client_top(instance: DOMReference) -> f32 {
    unsafe { element_get_client_top(instance) }
}

pub fn set_client_top(instance: DOMReference, value: f32) {
    unsafe {
        element_set_client_top(instance, value);
    }
}
extern "C" {
    fn element_get_client_left(instance: DOMReference) -> f32;
    fn element_set_client_left(instance: DOMReference, value: f32);
}

pub fn get_client_left(instance: DOMReference) -> f32 {
    unsafe { element_get_client_left(instance) }
}

pub fn set_client_left(instance: DOMReference, value: f32) {
    unsafe {
        element_set_client_left(instance, value);
    }
}
extern "C" {
    fn element_get_client_width(instance: DOMReference) -> f32;
    fn element_set_client_width(instance: DOMReference, value: f32);
}

pub fn get_client_width(instance: DOMReference) -> f32 {
    unsafe { element_get_client_width(instance) }
}

pub fn set_client_width(instance: DOMReference, value: f32) {
    unsafe {
        element_set_client_width(instance, value);
    }
}
extern "C" {
    fn element_get_client_height(instance: DOMReference) -> f32;
    fn element_set_client_height(instance: DOMReference, value: f32);
}

pub fn get_client_height(instance: DOMReference) -> f32 {
    unsafe { element_get_client_height(instance) }
}

pub fn set_client_height(instance: DOMReference, value: f32) {
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
    fn element_insert_adjacent_html(
        instance: DOMReference,
        insert_adjacent_html: CString,
        insert_adjacent_html: CString,
    );
}

pub fn insert_adjacent_html(instance: DOMReference, position: &str, text: &str) {
    unsafe { element_insert_adjacent_html(instance, to_cstring(position), to_cstring(text)) }
}
extern "C" {
    fn element_query_selector(instance: DOMReference, query_selector: CString) -> DOMReference;
}

pub fn query_selector(instance: DOMReference, selectors: &str) -> DOMReference {
    unsafe { element_query_selector(instance, to_cstring(selectors)) }
}
extern "C" {
    fn element_query_selector_all(
        instance: DOMReference,
        query_selector_all: CString,
    ) -> DOMReference;
}

pub fn query_selector_all(instance: DOMReference, selectors: &str) -> DOMReference {
    unsafe { element_query_selector_all(instance, to_cstring(selectors)) }
}
extern "C" {
    fn element_get_shadow_root(instance: DOMReference) -> DOMReference;
    fn element_set_shadow_root(instance: DOMReference, value: DOMReference);
}

pub fn get_shadow_root(instance: DOMReference) -> DOMReference {
    unsafe { element_get_shadow_root(instance) }
}

pub fn set_shadow_root(instance: DOMReference, value: DOMReference) {
    unsafe {
        element_set_shadow_root(instance, value);
    }
}
extern "C" {
    fn element_get_open_or_closed_shadow_root(instance: DOMReference) -> DOMReference;
    fn element_set_open_or_closed_shadow_root(instance: DOMReference, value: DOMReference);
}

pub fn get_open_or_closed_shadow_root(instance: DOMReference) -> DOMReference {
    unsafe { element_get_open_or_closed_shadow_root(instance) }
}

pub fn set_open_or_closed_shadow_root(instance: DOMReference, value: DOMReference) {
    unsafe {
        element_set_open_or_closed_shadow_root(instance, value);
    }
}
extern "C" {
    fn element_get_assigned_slot(instance: DOMReference) -> DOMReference;
    fn element_set_assigned_slot(instance: DOMReference, value: DOMReference);
}

pub fn get_assigned_slot(instance: DOMReference) -> DOMReference {
    unsafe { element_get_assigned_slot(instance) }
}

pub fn set_assigned_slot(instance: DOMReference, value: DOMReference) {
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
    fn element_request_fullscreen(instance: DOMReference);
}

pub fn request_fullscreen(instance: DOMReference) {
    unsafe { element_request_fullscreen(instance) }
}
extern "C" {
    fn element_request_pointer_lock(instance: DOMReference);
}

pub fn request_pointer_lock(instance: DOMReference) {
    unsafe { element_request_pointer_lock(instance) }
}
extern "C" {
    fn element_get_children(instance: DOMReference) -> DOMReference;
    fn element_set_children(instance: DOMReference, value: DOMReference);
}

pub fn get_children(instance: DOMReference) -> DOMReference {
    unsafe { element_get_children(instance) }
}

pub fn set_children(instance: DOMReference, value: DOMReference) {
    unsafe {
        element_set_children(instance, value);
    }
}
extern "C" {
    fn element_get_first_element_child(instance: DOMReference) -> DOMReference;
    fn element_set_first_element_child(instance: DOMReference, value: DOMReference);
}

pub fn get_first_element_child(instance: DOMReference) -> DOMReference {
    unsafe { element_get_first_element_child(instance) }
}

pub fn set_first_element_child(instance: DOMReference, value: DOMReference) {
    unsafe {
        element_set_first_element_child(instance, value);
    }
}
extern "C" {
    fn element_get_last_element_child(instance: DOMReference) -> DOMReference;
    fn element_set_last_element_child(instance: DOMReference, value: DOMReference);
}

pub fn get_last_element_child(instance: DOMReference) -> DOMReference {
    unsafe { element_get_last_element_child(instance) }
}

pub fn set_last_element_child(instance: DOMReference, value: DOMReference) {
    unsafe {
        element_set_last_element_child(instance, value);
    }
}
extern "C" {
    fn element_get_child_element_count(instance: DOMReference) -> f32;
    fn element_set_child_element_count(instance: DOMReference, value: f32);
}

pub fn get_child_element_count(instance: DOMReference) -> f32 {
    unsafe { element_get_child_element_count(instance) }
}

pub fn set_child_element_count(instance: DOMReference, value: f32) {
    unsafe {
        element_set_child_element_count(instance, value);
    }
}
extern "C" {
    fn element_prepend(instance: DOMReference, prepend: DOMReference);
}

pub fn prepend(instance: DOMReference, nodes: DOMReference) {
    unsafe { element_prepend(instance, nodes) }
}
extern "C" {
    fn element_append(instance: DOMReference, append: DOMReference);
}

pub fn append(instance: DOMReference, nodes: DOMReference) {
    unsafe { element_append(instance, nodes) }
}
extern "C" {
    fn element_before(instance: DOMReference, before: DOMReference);
}

pub fn before(instance: DOMReference, nodes: DOMReference) {
    unsafe { element_before(instance, nodes) }
}
extern "C" {
    fn element_after(instance: DOMReference, after: DOMReference);
}

pub fn after(instance: DOMReference, nodes: DOMReference) {
    unsafe { element_after(instance, nodes) }
}
extern "C" {
    fn element_replace_with(instance: DOMReference, replace_with: DOMReference);
}

pub fn replace_with(instance: DOMReference, nodes: DOMReference) {
    unsafe { element_replace_with(instance, nodes) }
}
extern "C" {
    fn element_remove(instance: DOMReference);
}

pub fn remove(instance: DOMReference) {
    unsafe { element_remove(instance) }
}
extern "C" {
    fn element_get_previous_element_sibling(instance: DOMReference) -> DOMReference;
    fn element_set_previous_element_sibling(instance: DOMReference, value: DOMReference);
}

pub fn get_previous_element_sibling(instance: DOMReference) -> DOMReference {
    unsafe { element_get_previous_element_sibling(instance) }
}

pub fn set_previous_element_sibling(instance: DOMReference, value: DOMReference) {
    unsafe {
        element_set_previous_element_sibling(instance, value);
    }
}
extern "C" {
    fn element_get_next_element_sibling(instance: DOMReference) -> DOMReference;
    fn element_set_next_element_sibling(instance: DOMReference, value: DOMReference);
}

pub fn get_next_element_sibling(instance: DOMReference) -> DOMReference {
    unsafe { element_get_next_element_sibling(instance) }
}

pub fn set_next_element_sibling(instance: DOMReference, value: DOMReference) {
    unsafe {
        element_set_next_element_sibling(instance, value);
    }
}
