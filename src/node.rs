#[allow(unused_imports)]
use crate::*;
pub const ELEMENT_NODE: f32 = 1 as f32;
pub const ATTRIBUTE_NODE: f32 = 2 as f32;
pub const TEXT_NODE: f32 = 3 as f32;
pub const CDATA_SECTION_NODE: f32 = 4 as f32;
pub const ENTITY_REFERENCE_NODE: f32 = 5 as f32;
pub const ENTITY_NODE: f32 = 6 as f32;
pub const PROCESSING_INSTRUCTION_NODE: f32 = 7 as f32;
pub const COMMENT_NODE: f32 = 8 as f32;
pub const DOCUMENT_NODE: f32 = 9 as f32;
pub const DOCUMENT_TYPE_NODE: f32 = 10 as f32;
pub const DOCUMENT_FRAGMENT_NODE: f32 = 11 as f32;
pub const NOTATION_NODE: f32 = 12 as f32;
extern "C" {
    fn node_get_node_type(instance: DOMReference) -> f32;
    fn node_set_node_type(instance: DOMReference, value: f32);
}

pub fn get_node_type(instance: DOMReference) -> f32 {
    unsafe { node_get_node_type(instance) }
}

pub fn set_node_type(instance: DOMReference, value: f32) {
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

pub fn get_is_connected(instance: DOMReference) -> bool {
    unsafe { 0 != node_get_is_connected(instance) }
}

pub fn set_is_connected(instance: DOMReference, value: bool) {
    unsafe {
        node_set_is_connected(instance, if value == true { 1 } else { 0 });
    }
}
extern "C" {
    fn node_get_owner_document(instance: DOMReference) -> DOMReference;
    fn node_set_owner_document(instance: DOMReference, value: DOMReference);
}

pub fn get_owner_document(instance: DOMReference) -> DOMReference {
    unsafe { node_get_owner_document(instance) }
}

pub fn set_owner_document(instance: DOMReference, value: DOMReference) {
    unsafe {
        node_set_owner_document(instance, value);
    }
}
extern "C" {
    fn node_get_root_node(instance: DOMReference, get_root_node: DOMReference) -> DOMReference;
}

pub fn get_root_node(instance: DOMReference, options: DOMReference) -> DOMReference {
    unsafe { node_get_root_node(instance, options) }
}
extern "C" {
    fn node_get_parent_node(instance: DOMReference) -> DOMReference;
    fn node_set_parent_node(instance: DOMReference, value: DOMReference);
}

pub fn get_parent_node(instance: DOMReference) -> DOMReference {
    unsafe { node_get_parent_node(instance) }
}

pub fn set_parent_node(instance: DOMReference, value: DOMReference) {
    unsafe {
        node_set_parent_node(instance, value);
    }
}
extern "C" {
    fn node_get_parent_element(instance: DOMReference) -> DOMReference;
    fn node_set_parent_element(instance: DOMReference, value: DOMReference);
}

pub fn get_parent_element(instance: DOMReference) -> DOMReference {
    unsafe { node_get_parent_element(instance) }
}

pub fn set_parent_element(instance: DOMReference, value: DOMReference) {
    unsafe {
        node_set_parent_element(instance, value);
    }
}
extern "C" {
    fn node_has_child_nodes(instance: DOMReference) -> i32;
}

pub fn has_child_nodes(instance: DOMReference) -> bool {
    unsafe { 0 != node_has_child_nodes(instance) }
}
extern "C" {
    fn node_get_child_nodes(instance: DOMReference) -> DOMReference;
    fn node_set_child_nodes(instance: DOMReference, value: DOMReference);
}

pub fn get_child_nodes(instance: DOMReference) -> DOMReference {
    unsafe { node_get_child_nodes(instance) }
}

pub fn set_child_nodes(instance: DOMReference, value: DOMReference) {
    unsafe {
        node_set_child_nodes(instance, value);
    }
}
extern "C" {
    fn node_get_first_child(instance: DOMReference) -> DOMReference;
    fn node_set_first_child(instance: DOMReference, value: DOMReference);
}

pub fn get_first_child(instance: DOMReference) -> DOMReference {
    unsafe { node_get_first_child(instance) }
}

pub fn set_first_child(instance: DOMReference, value: DOMReference) {
    unsafe {
        node_set_first_child(instance, value);
    }
}
extern "C" {
    fn node_get_last_child(instance: DOMReference) -> DOMReference;
    fn node_set_last_child(instance: DOMReference, value: DOMReference);
}

pub fn get_last_child(instance: DOMReference) -> DOMReference {
    unsafe { node_get_last_child(instance) }
}

pub fn set_last_child(instance: DOMReference, value: DOMReference) {
    unsafe {
        node_set_last_child(instance, value);
    }
}
extern "C" {
    fn node_get_previous_sibling(instance: DOMReference) -> DOMReference;
    fn node_set_previous_sibling(instance: DOMReference, value: DOMReference);
}

pub fn get_previous_sibling(instance: DOMReference) -> DOMReference {
    unsafe { node_get_previous_sibling(instance) }
}

pub fn set_previous_sibling(instance: DOMReference, value: DOMReference) {
    unsafe {
        node_set_previous_sibling(instance, value);
    }
}
extern "C" {
    fn node_get_next_sibling(instance: DOMReference) -> DOMReference;
    fn node_set_next_sibling(instance: DOMReference, value: DOMReference);
}

pub fn get_next_sibling(instance: DOMReference) -> DOMReference {
    unsafe { node_get_next_sibling(instance) }
}

pub fn set_next_sibling(instance: DOMReference, value: DOMReference) {
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
    fn node_insert_before(
        instance: DOMReference,
        insert_before: DOMReference,
        insert_before: DOMReference,
    ) -> DOMReference;
}

pub fn insert_before(
    instance: DOMReference,
    node: DOMReference,
    child: DOMReference,
) -> DOMReference {
    unsafe { node_insert_before(instance, node, child) }
}
extern "C" {
    fn node_append_child(instance: DOMReference, append_child: DOMReference) -> DOMReference;
}

pub fn append_child(instance: DOMReference, node: DOMReference) -> DOMReference {
    unsafe { node_append_child(instance, node) }
}
extern "C" {
    fn node_replace_child(
        instance: DOMReference,
        replace_child: DOMReference,
        replace_child: DOMReference,
    ) -> DOMReference;
}

pub fn replace_child(
    instance: DOMReference,
    node: DOMReference,
    child: DOMReference,
) -> DOMReference {
    unsafe { node_replace_child(instance, node, child) }
}
extern "C" {
    fn node_remove_child(instance: DOMReference, remove_child: DOMReference) -> DOMReference;
}

pub fn remove_child(instance: DOMReference, child: DOMReference) -> DOMReference {
    unsafe { node_remove_child(instance, child) }
}
extern "C" {
    fn node_normalize(instance: DOMReference);
}

pub fn normalize(instance: DOMReference) {
    unsafe { node_normalize(instance) }
}
extern "C" {
    fn node_clone_node(instance: DOMReference, clone_node: i32) -> DOMReference;
}

pub fn clone_node(instance: DOMReference, deep: bool) -> DOMReference {
    unsafe { node_clone_node(instance, if deep { 1 } else { 0 }) }
}
extern "C" {
    fn node_is_same_node(instance: DOMReference, is_same_node: DOMReference) -> i32;
}

pub fn is_same_node(instance: DOMReference, node: DOMReference) -> bool {
    unsafe { 0 != node_is_same_node(instance, node) }
}
extern "C" {
    fn node_is_equal_node(instance: DOMReference, is_equal_node: DOMReference) -> i32;
}

pub fn is_equal_node(instance: DOMReference, node: DOMReference) -> bool {
    unsafe { 0 != node_is_equal_node(instance, node) }
}
pub const DOCUMENT_POSITION_DISCONNECTED: f32 = 0x01 as f32;
pub const DOCUMENT_POSITION_PRECEDING: f32 = 0x02 as f32;
pub const DOCUMENT_POSITION_FOLLOWING: f32 = 0x04 as f32;
pub const DOCUMENT_POSITION_CONTAINS: f32 = 0x08 as f32;
pub const DOCUMENT_POSITION_CONTAINED_BY: f32 = 0x10 as f32;
pub const DOCUMENT_POSITION_IMPLEMENTATION_SPECIFIC: f32 = 0x20 as f32;
extern "C" {
    fn node_compare_document_position(
        instance: DOMReference,
        compare_document_position: DOMReference,
    ) -> f32;
}

pub fn compare_document_position(instance: DOMReference, other: DOMReference) -> f32 {
    unsafe { node_compare_document_position(instance, other) }
}
extern "C" {
    fn node_contains(instance: DOMReference, contains: DOMReference) -> i32;
}

pub fn contains(instance: DOMReference, other: DOMReference) -> bool {
    unsafe { 0 != node_contains(instance, other) }
}
extern "C" {
    fn node_lookup_prefix(instance: DOMReference, lookup_prefix: CString) -> CString;
}

pub fn lookup_prefix(instance: DOMReference, namespace: &str) -> String {
    unsafe { to_string(node_lookup_prefix(instance, to_cstring(namespace))) }
}
extern "C" {
    fn node_lookup_namespace_uri(instance: DOMReference, lookup_namespace_uri: CString) -> CString;
}

pub fn lookup_namespace_uri(instance: DOMReference, prefix: &str) -> String {
    unsafe { to_string(node_lookup_namespace_uri(instance, to_cstring(prefix))) }
}
extern "C" {
    fn node_is_default_namespace(instance: DOMReference, is_default_namespace: CString) -> i32;
}

pub fn is_default_namespace(instance: DOMReference, namespace: &str) -> bool {
    unsafe { 0 != node_is_default_namespace(instance, to_cstring(namespace)) }
}
