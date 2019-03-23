#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn node_get_node_type(instance: DOMReference) -> i32;
    fn node_set_node_type(instance: DOMReference, value: i32);
}

pub fn get_node_type(instance: DOMReference) -> i32 {
    unsafe { node_get_node_type(instance) }
}

pub fn set_node_type(instance: DOMReference, value: i32) {
    unsafe {
        node_set_node_type(instance, value);
    }
}
extern "C" {
    fn node_get_node_name(instance: DOMReference) -> CString;
    fn node_set_node_name(instance: DOMReference, value: CString);
}

pub fn get_node_name(instance: DOMReference) -> String {
    unsafe { to_string(node_get_node_name(instance)) }
}

pub fn set_node_name(instance: DOMReference, value: &str) {
    unsafe {
        node_set_node_name(instance, to_cstring(value));
    }
}
extern "C" {
    fn node_get_base_uri(instance: DOMReference) -> CString;
    fn node_set_base_uri(instance: DOMReference, value: CString);
}

pub fn get_base_uri(instance: DOMReference) -> String {
    unsafe { to_string(node_get_base_uri(instance)) }
}

pub fn set_base_uri(instance: DOMReference, value: &str) {
    unsafe {
        node_set_base_uri(instance, to_cstring(value));
    }
}
extern "C" {
    fn node_get_is_connected(instance: DOMReference) -> i32;
    fn node_set_is_connected(instance: DOMReference, value: i32);
}

pub fn get_is_connected(instance: DOMReference) -> i32 {
    unsafe { node_get_is_connected(instance) }
}

pub fn set_is_connected(instance: DOMReference, value: i32) {
    unsafe {
        node_set_is_connected(instance, value);
    }
}
extern "C" {
    fn node_get_owner_document(instance: DOMReference) -> i32;
    fn node_set_owner_document(instance: DOMReference, value: i32);
}

pub fn get_owner_document(instance: DOMReference) -> i32 {
    unsafe { node_get_owner_document(instance) }
}

pub fn set_owner_document(instance: DOMReference, value: i32) {
    unsafe {
        node_set_owner_document(instance, value);
    }
}
extern "C" {
    fn node_get_root_node(instance: i32, options: i32) -> i32;
}

pub fn get_root_node(instance: i32, options: i32) -> i32 {
    unsafe { node_get_root_node(instance, options) }
}
extern "C" {
    fn node_get_parent_node(instance: DOMReference) -> i32;
    fn node_set_parent_node(instance: DOMReference, value: i32);
}

pub fn get_parent_node(instance: DOMReference) -> i32 {
    unsafe { node_get_parent_node(instance) }
}

pub fn set_parent_node(instance: DOMReference, value: i32) {
    unsafe {
        node_set_parent_node(instance, value);
    }
}
extern "C" {
    fn node_get_parent_element(instance: DOMReference) -> i32;
    fn node_set_parent_element(instance: DOMReference, value: i32);
}

pub fn get_parent_element(instance: DOMReference) -> i32 {
    unsafe { node_get_parent_element(instance) }
}

pub fn set_parent_element(instance: DOMReference, value: i32) {
    unsafe {
        node_set_parent_element(instance, value);
    }
}
extern "C" {
    fn node_has_child_nodes(instance: i32) -> i32;
}

pub fn has_child_nodes(instance: i32) -> i32 {
    unsafe { node_has_child_nodes(instance) }
}
extern "C" {
    fn node_get_child_nodes(instance: DOMReference) -> i32;
    fn node_set_child_nodes(instance: DOMReference, value: i32);
}

pub fn get_child_nodes(instance: DOMReference) -> i32 {
    unsafe { node_get_child_nodes(instance) }
}

pub fn set_child_nodes(instance: DOMReference, value: i32) {
    unsafe {
        node_set_child_nodes(instance, value);
    }
}
extern "C" {
    fn node_get_first_child(instance: DOMReference) -> i32;
    fn node_set_first_child(instance: DOMReference, value: i32);
}

pub fn get_first_child(instance: DOMReference) -> i32 {
    unsafe { node_get_first_child(instance) }
}

pub fn set_first_child(instance: DOMReference, value: i32) {
    unsafe {
        node_set_first_child(instance, value);
    }
}
extern "C" {
    fn node_get_last_child(instance: DOMReference) -> i32;
    fn node_set_last_child(instance: DOMReference, value: i32);
}

pub fn get_last_child(instance: DOMReference) -> i32 {
    unsafe { node_get_last_child(instance) }
}

pub fn set_last_child(instance: DOMReference, value: i32) {
    unsafe {
        node_set_last_child(instance, value);
    }
}
extern "C" {
    fn node_get_previous_sibling(instance: DOMReference) -> i32;
    fn node_set_previous_sibling(instance: DOMReference, value: i32);
}

pub fn get_previous_sibling(instance: DOMReference) -> i32 {
    unsafe { node_get_previous_sibling(instance) }
}

pub fn set_previous_sibling(instance: DOMReference, value: i32) {
    unsafe {
        node_set_previous_sibling(instance, value);
    }
}
extern "C" {
    fn node_get_next_sibling(instance: DOMReference) -> i32;
    fn node_set_next_sibling(instance: DOMReference, value: i32);
}

pub fn get_next_sibling(instance: DOMReference) -> i32 {
    unsafe { node_get_next_sibling(instance) }
}

pub fn set_next_sibling(instance: DOMReference, value: i32) {
    unsafe {
        node_set_next_sibling(instance, value);
    }
}
extern "C" {
    fn node_get_node_value(instance: DOMReference) -> CString;
    fn node_set_node_value(instance: DOMReference, value: CString);
}

pub fn get_node_value(instance: DOMReference) -> String {
    unsafe { to_string(node_get_node_value(instance)) }
}

pub fn set_node_value(instance: DOMReference, value: &str) {
    unsafe {
        node_set_node_value(instance, to_cstring(value));
    }
}
extern "C" {
    fn node_get_text_content(instance: DOMReference) -> CString;
    fn node_set_text_content(instance: DOMReference, value: CString);
}

pub fn get_text_content(instance: DOMReference) -> String {
    unsafe { to_string(node_get_text_content(instance)) }
}

pub fn set_text_content(instance: DOMReference, value: &str) {
    unsafe {
        node_set_text_content(instance, to_cstring(value));
    }
}
extern "C" {
    fn node_insert_before(instance: i32, node: i32, child: i32) -> i32;
}

pub fn insert_before(instance: i32, node: i32, child: i32) -> i32 {
    unsafe { node_insert_before(instance, node, child) }
}
extern "C" {
    fn node_append_child(instance: i32, node: i32) -> i32;
}

pub fn append_child(instance: i32, node: i32) -> i32 {
    unsafe { node_append_child(instance, node) }
}
extern "C" {
    fn node_replace_child(instance: i32, node: i32, child: i32) -> i32;
}

pub fn replace_child(instance: i32, node: i32, child: i32) -> i32 {
    unsafe { node_replace_child(instance, node, child) }
}
extern "C" {
    fn node_remove_child(instance: i32, child: i32) -> i32;
}

pub fn remove_child(instance: i32, child: i32) -> i32 {
    unsafe { node_remove_child(instance, child) }
}
extern "C" {
    fn node_normalize(instance: i32);
}

pub fn normalize(instance: i32) {
    unsafe { node_normalize(instance) }
}
extern "C" {
    fn node_clone_node(instance: i32, deep: i32) -> i32;
}

pub fn clone_node(instance: i32, deep: i32) -> i32 {
    unsafe { node_clone_node(instance, deep) }
}
extern "C" {
    fn node_is_same_node(instance: i32, node: i32) -> i32;
}

pub fn is_same_node(instance: i32, node: i32) -> i32 {
    unsafe { node_is_same_node(instance, node) }
}
extern "C" {
    fn node_is_equal_node(instance: i32, node: i32) -> i32;
}

pub fn is_equal_node(instance: i32, node: i32) -> i32 {
    unsafe { node_is_equal_node(instance, node) }
}
extern "C" {
    fn node_compare_document_position(instance: i32, other: i32) -> i32;
}

pub fn compare_document_position(instance: i32, other: i32) -> i32 {
    unsafe { node_compare_document_position(instance, other) }
}
extern "C" {
    fn node_contains(instance: i32, other: i32) -> i32;
}

pub fn contains(instance: i32, other: i32) -> i32 {
    unsafe { node_contains(instance, other) }
}
extern "C" {
    fn node_lookup_prefix(instance: i32, namespace: CString) -> CString;
}

pub fn lookup_prefix(instance: i32, namespace: &str) -> String {
    unsafe { to_string(node_lookup_prefix(instance, to_cstring(namespace))) }
}
extern "C" {
    fn node_lookup_namespace_uri(instance: i32, prefix: CString) -> CString;
}

pub fn lookup_namespace_uri(instance: i32, prefix: &str) -> String {
    unsafe { to_string(node_lookup_namespace_uri(instance, to_cstring(prefix))) }
}
extern "C" {
    fn node_is_default_namespace(instance: i32, namespace: CString) -> i32;
}

pub fn is_default_namespace(instance: i32, namespace: &str) -> i32 {
    unsafe { node_is_default_namespace(instance, to_cstring(namespace)) }
}
