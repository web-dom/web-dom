#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn document_get_implementation(instance: i32) -> i32;
    fn document_set_implementation(instance: i32, value: i32);
}

pub fn get_implementation(instance: i32) -> i32 {
    unsafe { document_get_implementation(instance) }
}

pub fn set_implementation(instance: i32, value: i32) {
    unsafe {
        document_set_implementation(instance, value);
    }
}
extern "C" {
    fn document_get_url(instance: i32) -> CString;
    fn document_set_url(instance: i32, value: i32);
}

pub fn get_url(instance: i32) -> String {
    unsafe { cstr_to_string(document_get_url(instance)) }
}

pub fn set_url(instance: i32, value: i32) {
    unsafe {
        document_set_url(instance, value);
    }
}
extern "C" {
    fn document_get_document_uri(instance: i32) -> CString;
    fn document_set_document_uri(instance: i32, value: i32);
}

pub fn get_document_uri(instance: i32) -> String {
    unsafe { cstr_to_string(document_get_document_uri(instance)) }
}

pub fn set_document_uri(instance: i32, value: i32) {
    unsafe {
        document_set_document_uri(instance, value);
    }
}
extern "C" {
    fn document_get_compat_mode(instance: i32) -> CString;
    fn document_set_compat_mode(instance: i32, value: i32);
}

pub fn get_compat_mode(instance: i32) -> String {
    unsafe { cstr_to_string(document_get_compat_mode(instance)) }
}

pub fn set_compat_mode(instance: i32, value: i32) {
    unsafe {
        document_set_compat_mode(instance, value);
    }
}
extern "C" {
    fn document_get_character_set(instance: i32) -> CString;
    fn document_set_character_set(instance: i32, value: i32);
}

pub fn get_character_set(instance: i32) -> String {
    unsafe { cstr_to_string(document_get_character_set(instance)) }
}

pub fn set_character_set(instance: i32, value: i32) {
    unsafe {
        document_set_character_set(instance, value);
    }
}
extern "C" {
    fn document_get_charset(instance: i32) -> CString;
    fn document_set_charset(instance: i32, value: i32);
}

pub fn get_charset(instance: i32) -> String {
    unsafe { cstr_to_string(document_get_charset(instance)) }
}

pub fn set_charset(instance: i32, value: i32) {
    unsafe {
        document_set_charset(instance, value);
    }
}
extern "C" {
    fn document_get_input_encoding(instance: i32) -> CString;
    fn document_set_input_encoding(instance: i32, value: i32);
}

pub fn get_input_encoding(instance: i32) -> String {
    unsafe { cstr_to_string(document_get_input_encoding(instance)) }
}

pub fn set_input_encoding(instance: i32, value: i32) {
    unsafe {
        document_set_input_encoding(instance, value);
    }
}
extern "C" {
    fn document_get_content_type(instance: i32) -> CString;
    fn document_set_content_type(instance: i32, value: i32);
}

pub fn get_content_type(instance: i32) -> String {
    unsafe { cstr_to_string(document_get_content_type(instance)) }
}

pub fn set_content_type(instance: i32, value: i32) {
    unsafe {
        document_set_content_type(instance, value);
    }
}
extern "C" {
    fn document_get_doctype(instance: i32) -> i32;
    fn document_set_doctype(instance: i32, value: i32);
}

pub fn get_doctype(instance: i32) -> i32 {
    unsafe { document_get_doctype(instance) }
}

pub fn set_doctype(instance: i32, value: i32) {
    unsafe {
        document_set_doctype(instance, value);
    }
}
extern "C" {
    fn document_get_document_element(instance: i32) -> i32;
    fn document_set_document_element(instance: i32, value: i32);
}

pub fn get_document_element(instance: i32) -> i32 {
    unsafe { document_get_document_element(instance) }
}

pub fn set_document_element(instance: i32, value: i32) {
    unsafe {
        document_set_document_element(instance, value);
    }
}
extern "C" {
    fn document_get_elements_by_tag_name(instance: i32, local_name: CString) -> i32;
}

pub fn get_elements_by_tag_name(instance: i32, local_name: &str) -> i32 {
    unsafe { document_get_elements_by_tag_name(instance, cstr(local_name)) }
}
extern "C" {
    fn document_get_elements_by_tag_name_n_s(
        instance: i32,
        namespace: CString,
        local_name: CString,
    ) -> i32;
}

pub fn get_elements_by_tag_name_n_s(instance: i32, namespace: &str, local_name: &str) -> i32 {
    unsafe { document_get_elements_by_tag_name_n_s(instance, cstr(namespace), cstr(local_name)) }
}
extern "C" {
    fn document_get_elements_by_class_name(instance: i32, class_names: CString) -> i32;
}

pub fn get_elements_by_class_name(instance: i32, class_names: &str) -> i32 {
    unsafe { document_get_elements_by_class_name(instance, cstr(class_names)) }
}
extern "C" {
    fn document_get_element_by_id(instance: i32, element_id: CString) -> i32;
}

pub fn get_element_by_id(instance: i32, element_id: &str) -> i32 {
    unsafe { document_get_element_by_id(instance, cstr(element_id)) }
}
extern "C" {
    fn document_create_element(instance: i32, local_name: CString, options: i32) -> i32;
}

pub fn create_element(instance: i32, local_name: &str, options: i32) -> i32 {
    unsafe { document_create_element(instance, cstr(local_name), options) }
}
extern "C" {
    fn document_create_element_n_s(
        instance: i32,
        namespace: CString,
        qualified_name: CString,
        options: i32,
    ) -> i32;
}

pub fn create_element_n_s(
    instance: i32,
    namespace: &str,
    qualified_name: &str,
    options: i32,
) -> i32 {
    unsafe { document_create_element_n_s(instance, cstr(namespace), cstr(qualified_name), options) }
}
extern "C" {
    fn document_create_document_fragment(instance: i32) -> i32;
}

pub fn create_document_fragment(instance: i32) -> i32 {
    unsafe { document_create_document_fragment(instance) }
}
extern "C" {
    fn document_create_text_node(instance: i32, data: CString) -> i32;
}

pub fn create_text_node(instance: i32, data: &str) -> i32 {
    unsafe { document_create_text_node(instance, cstr(data)) }
}
extern "C" {
    fn document_create_comment(instance: i32, data: CString) -> i32;
}

pub fn create_comment(instance: i32, data: &str) -> i32 {
    unsafe { document_create_comment(instance, cstr(data)) }
}
extern "C" {
    fn document_create_processing_instruction(instance: i32, target: CString, data: CString)
        -> i32;
}

pub fn create_processing_instruction(instance: i32, target: &str, data: &str) -> i32 {
    unsafe { document_create_processing_instruction(instance, cstr(target), cstr(data)) }
}
extern "C" {
    fn document_import_node(instance: i32, node: i32, deep: i32) -> i32;
}

pub fn import_node(instance: i32, node: i32, deep: i32) -> i32 {
    unsafe { document_import_node(instance, node, deep) }
}
extern "C" {
    fn document_adopt_node(instance: i32, node: i32) -> i32;
}

pub fn adopt_node(instance: i32, node: i32) -> i32 {
    unsafe { document_adopt_node(instance, node) }
}
extern "C" {
    fn document_create_event(instance: i32, name: CString) -> i32;
}

pub fn create_event(instance: i32, name: &str) -> i32 {
    unsafe { document_create_event(instance, cstr(name)) }
}
extern "C" {
    fn document_create_range(instance: i32) -> i32;
}

pub fn create_range(instance: i32) -> i32 {
    unsafe { document_create_range(instance) }
}
extern "C" {
    fn document_create_node_iterator(
        instance: i32,
        root: i32,
        what_to_show: i32,
        filter: i32,
    ) -> i32;
}

pub fn create_node_iterator(instance: i32, root: i32, what_to_show: i32, filter: i32) -> i32 {
    unsafe { document_create_node_iterator(instance, root, what_to_show, filter) }
}
extern "C" {
    fn document_create_tree_walker(instance: i32, root: i32, what_to_show: i32, filter: i32)
        -> i32;
}

pub fn create_tree_walker(instance: i32, root: i32, what_to_show: i32, filter: i32) -> i32 {
    unsafe { document_create_tree_walker(instance, root, what_to_show, filter) }
}
extern "C" {
    fn document_create_c_d_a_t_a_section(instance: i32, data: CString) -> i32;
}

pub fn create_c_d_a_t_a_section(instance: i32, data: &str) -> i32 {
    unsafe { document_create_c_d_a_t_a_section(instance, cstr(data)) }
}
extern "C" {
    fn document_create_attribute(instance: i32, name: CString) -> i32;
}

pub fn create_attribute(instance: i32, name: &str) -> i32 {
    unsafe { document_create_attribute(instance, cstr(name)) }
}
extern "C" {
    fn document_create_attribute_n_s(instance: i32, namespace: CString, name: CString) -> i32;
}

pub fn create_attribute_n_s(instance: i32, namespace: &str, name: &str) -> i32 {
    unsafe { document_create_attribute_n_s(instance, cstr(namespace), cstr(name)) }
}
extern "C" {
    fn document_get_location(instance: i32) -> i32;
    fn document_set_location(instance: i32, value: i32);
}

pub fn get_location(instance: i32) -> i32 {
    unsafe { document_get_location(instance) }
}

pub fn set_location(instance: i32, value: i32) {
    unsafe {
        document_set_location(instance, value);
    }
}
extern "C" {
    fn document_get_referrer(instance: i32) -> CString;
    fn document_set_referrer(instance: i32, value: i32);
}

pub fn get_referrer(instance: i32) -> String {
    unsafe { cstr_to_string(document_get_referrer(instance)) }
}

pub fn set_referrer(instance: i32, value: i32) {
    unsafe {
        document_set_referrer(instance, value);
    }
}
extern "C" {
    fn document_get_last_modified(instance: i32) -> CString;
    fn document_set_last_modified(instance: i32, value: i32);
}

pub fn get_last_modified(instance: i32) -> String {
    unsafe { cstr_to_string(document_get_last_modified(instance)) }
}

pub fn set_last_modified(instance: i32, value: i32) {
    unsafe {
        document_set_last_modified(instance, value);
    }
}
extern "C" {
    fn document_get_ready_state(instance: i32) -> CString;
    fn document_set_ready_state(instance: i32, value: i32);
}

pub fn get_ready_state(instance: i32) -> String {
    unsafe { cstr_to_string(document_get_ready_state(instance)) }
}

pub fn set_ready_state(instance: i32, value: i32) {
    unsafe {
        document_set_ready_state(instance, value);
    }
}
extern "C" {
    fn document_get_title(instance: i32) -> CString;
    fn document_set_title(instance: i32, value: i32);
}

pub fn get_title(instance: i32) -> String {
    unsafe { cstr_to_string(document_get_title(instance)) }
}

pub fn set_title(instance: i32, value: i32) {
    unsafe {
        document_set_title(instance, value);
    }
}
extern "C" {
    fn document_get_dir(instance: i32) -> CString;
    fn document_set_dir(instance: i32, value: i32);
}

pub fn get_dir(instance: i32) -> String {
    unsafe { cstr_to_string(document_get_dir(instance)) }
}

pub fn set_dir(instance: i32, value: i32) {
    unsafe {
        document_set_dir(instance, value);
    }
}
extern "C" {
    fn document_get_body(instance: i32) -> i32;
    fn document_set_body(instance: i32, value: i32);
}

pub fn get_body(instance: i32) -> i32 {
    unsafe { document_get_body(instance) }
}

pub fn set_body(instance: i32, value: i32) {
    unsafe {
        document_set_body(instance, value);
    }
}
extern "C" {
    fn document_get_head(instance: i32) -> i32;
    fn document_set_head(instance: i32, value: i32);
}

pub fn get_head(instance: i32) -> i32 {
    unsafe { document_get_head(instance) }
}

pub fn set_head(instance: i32, value: i32) {
    unsafe {
        document_set_head(instance, value);
    }
}
extern "C" {
    fn document_get_images(instance: i32) -> i32;
    fn document_set_images(instance: i32, value: i32);
}

pub fn get_images(instance: i32) -> i32 {
    unsafe { document_get_images(instance) }
}

pub fn set_images(instance: i32, value: i32) {
    unsafe {
        document_set_images(instance, value);
    }
}
extern "C" {
    fn document_get_embeds(instance: i32) -> i32;
    fn document_set_embeds(instance: i32, value: i32);
}

pub fn get_embeds(instance: i32) -> i32 {
    unsafe { document_get_embeds(instance) }
}

pub fn set_embeds(instance: i32, value: i32) {
    unsafe {
        document_set_embeds(instance, value);
    }
}
extern "C" {
    fn document_get_plugins(instance: i32) -> i32;
    fn document_set_plugins(instance: i32, value: i32);
}

pub fn get_plugins(instance: i32) -> i32 {
    unsafe { document_get_plugins(instance) }
}

pub fn set_plugins(instance: i32, value: i32) {
    unsafe {
        document_set_plugins(instance, value);
    }
}
extern "C" {
    fn document_get_links(instance: i32) -> i32;
    fn document_set_links(instance: i32, value: i32);
}

pub fn get_links(instance: i32) -> i32 {
    unsafe { document_get_links(instance) }
}

pub fn set_links(instance: i32, value: i32) {
    unsafe {
        document_set_links(instance, value);
    }
}
extern "C" {
    fn document_get_forms(instance: i32) -> i32;
    fn document_set_forms(instance: i32, value: i32);
}

pub fn get_forms(instance: i32) -> i32 {
    unsafe { document_get_forms(instance) }
}

pub fn set_forms(instance: i32, value: i32) {
    unsafe {
        document_set_forms(instance, value);
    }
}
extern "C" {
    fn document_get_scripts(instance: i32) -> i32;
    fn document_set_scripts(instance: i32, value: i32);
}

pub fn get_scripts(instance: i32) -> i32 {
    unsafe { document_get_scripts(instance) }
}

pub fn set_scripts(instance: i32, value: i32) {
    unsafe {
        document_set_scripts(instance, value);
    }
}
extern "C" {
    fn document_get_elements_by_name(instance: i32, element_name: CString) -> i32;
}

pub fn get_elements_by_name(instance: i32, element_name: &str) -> i32 {
    unsafe { document_get_elements_by_name(instance, cstr(element_name)) }
}
extern "C" {
    fn document_get_default_view(instance: i32) -> i32;
    fn document_set_default_view(instance: i32, value: i32);
}

pub fn get_default_view(instance: i32) -> i32 {
    unsafe { document_get_default_view(instance) }
}

pub fn set_default_view(instance: i32, value: i32) {
    unsafe {
        document_set_default_view(instance, value);
    }
}
extern "C" {
    fn document_has_focus(instance: i32) -> i32;
}

pub fn has_focus(instance: i32) -> i32 {
    unsafe { document_has_focus(instance) }
}
extern "C" {
    fn document_get_onreadystatechange(instance: i32) -> i32;
    fn document_set_onreadystatechange(instance: i32, value: i32);
}

pub fn get_onreadystatechange(instance: i32) -> i32 {
    unsafe { document_get_onreadystatechange(instance) }
}

pub fn set_onreadystatechange(instance: i32, value: i32) {
    unsafe {
        document_set_onreadystatechange(instance, value);
    }
}
extern "C" {
    fn document_get_onbeforescriptexecute(instance: i32) -> i32;
    fn document_set_onbeforescriptexecute(instance: i32, value: i32);
}

pub fn get_onbeforescriptexecute(instance: i32) -> i32 {
    unsafe { document_get_onbeforescriptexecute(instance) }
}

pub fn set_onbeforescriptexecute(instance: i32, value: i32) {
    unsafe {
        document_set_onbeforescriptexecute(instance, value);
    }
}
extern "C" {
    fn document_get_onafterscriptexecute(instance: i32) -> i32;
    fn document_set_onafterscriptexecute(instance: i32, value: i32);
}

pub fn get_onafterscriptexecute(instance: i32) -> i32 {
    unsafe { document_get_onafterscriptexecute(instance) }
}

pub fn set_onafterscriptexecute(instance: i32, value: i32) {
    unsafe {
        document_set_onafterscriptexecute(instance, value);
    }
}
extern "C" {
    fn document_get_onselectionchange(instance: i32) -> i32;
    fn document_set_onselectionchange(instance: i32, value: i32);
}

pub fn get_onselectionchange(instance: i32) -> i32 {
    unsafe { document_get_onselectionchange(instance) }
}

pub fn set_onselectionchange(instance: i32, value: i32) {
    unsafe {
        document_set_onselectionchange(instance, value);
    }
}
extern "C" {
    fn document_get_current_script(instance: i32) -> i32;
    fn document_set_current_script(instance: i32, value: i32);
}

pub fn get_current_script(instance: i32) -> i32 {
    unsafe { document_get_current_script(instance) }
}

pub fn set_current_script(instance: i32, value: i32) {
    unsafe {
        document_set_current_script(instance, value);
    }
}
extern "C" {
    fn document_release_capture(instance: i32);
}

pub fn release_capture(instance: i32) {
    unsafe { document_release_capture(instance) }
}
extern "C" {
    fn document_get_document_uri_object(instance: i32) -> i32;
    fn document_set_document_uri_object(instance: i32, value: i32);
}

pub fn get_document_uri_object(instance: i32) -> i32 {
    unsafe { document_get_document_uri_object(instance) }
}

pub fn set_document_uri_object(instance: i32, value: i32) {
    unsafe {
        document_set_document_uri_object(instance, value);
    }
}
extern "C" {
    fn document_get_referrer_policy(instance: i32) -> i32;
    fn document_set_referrer_policy(instance: i32, value: i32);
}

pub fn get_referrer_policy(instance: i32) -> i32 {
    unsafe { document_get_referrer_policy(instance) }
}

pub fn set_referrer_policy(instance: i32, value: i32) {
    unsafe {
        document_set_referrer_policy(instance, value);
    }
}
extern "C" {
    fn document_get_anchors(instance: i32) -> i32;
    fn document_set_anchors(instance: i32, value: i32);
}

pub fn get_anchors(instance: i32) -> i32 {
    unsafe { document_get_anchors(instance) }
}

pub fn set_anchors(instance: i32, value: i32) {
    unsafe {
        document_set_anchors(instance, value);
    }
}
extern "C" {
    fn document_get_applets(instance: i32) -> i32;
    fn document_set_applets(instance: i32, value: i32);
}

pub fn get_applets(instance: i32) -> i32 {
    unsafe { document_get_applets(instance) }
}

pub fn set_applets(instance: i32, value: i32) {
    unsafe {
        document_set_applets(instance, value);
    }
}
extern "C" {
    fn document_get_fullscreen(instance: i32) -> i32;
    fn document_set_fullscreen(instance: i32, value: i32);
}

pub fn get_fullscreen(instance: i32) -> i32 {
    unsafe { document_get_fullscreen(instance) }
}

pub fn set_fullscreen(instance: i32, value: i32) {
    unsafe {
        document_set_fullscreen(instance, value);
    }
}
extern "C" {
    fn document_get_fullscreen_enabled(instance: i32) -> i32;
    fn document_set_fullscreen_enabled(instance: i32, value: i32);
}

pub fn get_fullscreen_enabled(instance: i32) -> i32 {
    unsafe { document_get_fullscreen_enabled(instance) }
}

pub fn set_fullscreen_enabled(instance: i32, value: i32) {
    unsafe {
        document_set_fullscreen_enabled(instance, value);
    }
}
extern "C" {
    fn document_exit_fullscreen(instance: i32);
}

pub fn exit_fullscreen(instance: i32) {
    unsafe { document_exit_fullscreen(instance) }
}
extern "C" {
    fn document_get_onfullscreenchange(instance: i32) -> i32;
    fn document_set_onfullscreenchange(instance: i32, value: i32);
}

pub fn get_onfullscreenchange(instance: i32) -> i32 {
    unsafe { document_get_onfullscreenchange(instance) }
}

pub fn set_onfullscreenchange(instance: i32, value: i32) {
    unsafe {
        document_set_onfullscreenchange(instance, value);
    }
}
extern "C" {
    fn document_get_onfullscreenerror(instance: i32) -> i32;
    fn document_set_onfullscreenerror(instance: i32, value: i32);
}

pub fn get_onfullscreenerror(instance: i32) -> i32 {
    unsafe { document_get_onfullscreenerror(instance) }
}

pub fn set_onfullscreenerror(instance: i32, value: i32) {
    unsafe {
        document_set_onfullscreenerror(instance, value);
    }
}
extern "C" {
    fn document_exit_pointer_lock(instance: i32);
}

pub fn exit_pointer_lock(instance: i32) {
    unsafe { document_exit_pointer_lock(instance) }
}
extern "C" {
    fn document_get_onpointerlockchange(instance: i32) -> i32;
    fn document_set_onpointerlockchange(instance: i32, value: i32);
}

pub fn get_onpointerlockchange(instance: i32) -> i32 {
    unsafe { document_get_onpointerlockchange(instance) }
}

pub fn set_onpointerlockchange(instance: i32, value: i32) {
    unsafe {
        document_set_onpointerlockchange(instance, value);
    }
}
extern "C" {
    fn document_get_onpointerlockerror(instance: i32) -> i32;
    fn document_set_onpointerlockerror(instance: i32, value: i32);
}

pub fn get_onpointerlockerror(instance: i32) -> i32 {
    unsafe { document_get_onpointerlockerror(instance) }
}

pub fn set_onpointerlockerror(instance: i32, value: i32) {
    unsafe {
        document_set_onpointerlockerror(instance, value);
    }
}
extern "C" {
    fn document_get_hidden(instance: i32) -> i32;
    fn document_set_hidden(instance: i32, value: i32);
}

pub fn get_hidden(instance: i32) -> i32 {
    unsafe { document_get_hidden(instance) }
}

pub fn set_hidden(instance: i32, value: i32) {
    unsafe {
        document_set_hidden(instance, value);
    }
}
extern "C" {
    fn document_get_visibility_state(instance: i32) -> i32;
    fn document_set_visibility_state(instance: i32, value: i32);
}

pub fn get_visibility_state(instance: i32) -> i32 {
    unsafe { document_get_visibility_state(instance) }
}

pub fn set_visibility_state(instance: i32, value: i32) {
    unsafe {
        document_set_visibility_state(instance, value);
    }
}
extern "C" {
    fn document_get_onvisibilitychange(instance: i32) -> i32;
    fn document_set_onvisibilitychange(instance: i32, value: i32);
}

pub fn get_onvisibilitychange(instance: i32) -> i32 {
    unsafe { document_get_onvisibilitychange(instance) }
}

pub fn set_onvisibilitychange(instance: i32, value: i32) {
    unsafe {
        document_set_onvisibilitychange(instance, value);
    }
}
extern "C" {
    fn document_get_selected_style_sheet_set(instance: i32) -> CString;
    fn document_set_selected_style_sheet_set(instance: i32, value: i32);
}

pub fn get_selected_style_sheet_set(instance: i32) -> String {
    unsafe { cstr_to_string(document_get_selected_style_sheet_set(instance)) }
}

pub fn set_selected_style_sheet_set(instance: i32, value: i32) {
    unsafe {
        document_set_selected_style_sheet_set(instance, value);
    }
}
extern "C" {
    fn document_get_last_style_sheet_set(instance: i32) -> CString;
    fn document_set_last_style_sheet_set(instance: i32, value: i32);
}

pub fn get_last_style_sheet_set(instance: i32) -> String {
    unsafe { cstr_to_string(document_get_last_style_sheet_set(instance)) }
}

pub fn set_last_style_sheet_set(instance: i32, value: i32) {
    unsafe {
        document_set_last_style_sheet_set(instance, value);
    }
}
extern "C" {
    fn document_get_preferred_style_sheet_set(instance: i32) -> CString;
    fn document_set_preferred_style_sheet_set(instance: i32, value: i32);
}

pub fn get_preferred_style_sheet_set(instance: i32) -> String {
    unsafe { cstr_to_string(document_get_preferred_style_sheet_set(instance)) }
}

pub fn set_preferred_style_sheet_set(instance: i32, value: i32) {
    unsafe {
        document_set_preferred_style_sheet_set(instance, value);
    }
}
extern "C" {
    fn document_get_style_sheet_sets(instance: i32) -> i32;
    fn document_set_style_sheet_sets(instance: i32, value: i32);
}

pub fn get_style_sheet_sets(instance: i32) -> i32 {
    unsafe { document_get_style_sheet_sets(instance) }
}

pub fn set_style_sheet_sets(instance: i32, value: i32) {
    unsafe {
        document_set_style_sheet_sets(instance, value);
    }
}
extern "C" {
    fn document_enable_style_sheets_for_set(instance: i32, name: CString);
}

pub fn enable_style_sheets_for_set(instance: i32, name: &str) {
    unsafe { document_enable_style_sheets_for_set(instance, cstr(name)) }
}
extern "C" {
    fn document_caret_position_from_point(instance: i32, x: i32, y: i32) -> i32;
}

pub fn caret_position_from_point(instance: i32, x: i32, y: i32) -> i32 {
    unsafe { document_caret_position_from_point(instance, x, y) }
}
extern "C" {
    fn document_get_scrolling_element(instance: i32) -> i32;
    fn document_set_scrolling_element(instance: i32, value: i32);
}

pub fn get_scrolling_element(instance: i32) -> i32 {
    unsafe { document_get_scrolling_element(instance) }
}

pub fn set_scrolling_element(instance: i32, value: i32) {
    unsafe {
        document_set_scrolling_element(instance, value);
    }
}
extern "C" {
    fn document_query_selector(instance: i32, selectors: CString) -> i32;
}

pub fn query_selector(instance: i32, selectors: &str) -> i32 {
    unsafe { document_query_selector(instance, cstr(selectors)) }
}
extern "C" {
    fn document_query_selector_all(instance: i32, selectors: CString) -> i32;
}

pub fn query_selector_all(instance: i32, selectors: &str) -> i32 {
    unsafe { document_query_selector_all(instance, cstr(selectors)) }
}
extern "C" {
    fn document_get_timeline(instance: i32) -> i32;
    fn document_set_timeline(instance: i32, value: i32);
}

pub fn get_timeline(instance: i32) -> i32 {
    unsafe { document_get_timeline(instance) }
}

pub fn set_timeline(instance: i32, value: i32) {
    unsafe {
        document_set_timeline(instance, value);
    }
}
extern "C" {
    fn document_get_animations(instance: i32) -> i32;
}

pub fn get_animations(instance: i32) -> i32 {
    unsafe { document_get_animations(instance) }
}
extern "C" {
    fn document_get_root_element(instance: i32) -> i32;
    fn document_set_root_element(instance: i32, value: i32);
}

pub fn get_root_element(instance: i32) -> i32 {
    unsafe { document_get_root_element(instance) }
}

pub fn set_root_element(instance: i32, value: i32) {
    unsafe {
        document_set_root_element(instance, value);
    }
}
extern "C" {
    fn document_get_is_srcdoc_document(instance: i32) -> i32;
    fn document_set_is_srcdoc_document(instance: i32, value: i32);
}

pub fn get_is_srcdoc_document(instance: i32) -> i32 {
    unsafe { document_get_is_srcdoc_document(instance) }
}

pub fn set_is_srcdoc_document(instance: i32, value: i32) {
    unsafe {
        document_set_is_srcdoc_document(instance, value);
    }
}
extern "C" {
    fn document_get_sandbox_flags_as_string(instance: i32) -> CString;
    fn document_set_sandbox_flags_as_string(instance: i32, value: i32);
}

pub fn get_sandbox_flags_as_string(instance: i32) -> String {
    unsafe { cstr_to_string(document_get_sandbox_flags_as_string(instance)) }
}

pub fn set_sandbox_flags_as_string(instance: i32, value: i32) {
    unsafe {
        document_set_sandbox_flags_as_string(instance, value);
    }
}
extern "C" {
    fn document_insert_anonymous_content(instance: i32, a_element: i32) -> i32;
}

pub fn insert_anonymous_content(instance: i32, a_element: i32) -> i32 {
    unsafe { document_insert_anonymous_content(instance, a_element) }
}
extern "C" {
    fn document_remove_anonymous_content(instance: i32, a_content: i32);
}

pub fn remove_anonymous_content(instance: i32, a_content: i32) {
    unsafe { document_remove_anonymous_content(instance, a_content) }
}
extern "C" {
    fn document_get_selection(instance: i32) -> i32;
}

pub fn get_selection(instance: i32) -> i32 {
    unsafe { document_get_selection(instance) }
}
extern "C" {
    fn document_get_user_has_interacted(instance: i32) -> i32;
    fn document_set_user_has_interacted(instance: i32, value: i32);
}

pub fn get_user_has_interacted(instance: i32) -> i32 {
    unsafe { document_get_user_has_interacted(instance) }
}

pub fn set_user_has_interacted(instance: i32, value: i32) {
    unsafe {
        document_set_user_has_interacted(instance, value);
    }
}
extern "C" {
    fn document_notify_user_gesture_activation(instance: i32);
}

pub fn notify_user_gesture_activation(instance: i32) {
    unsafe { document_notify_user_gesture_activation(instance) }
}
extern "C" {
    fn document_get_document_flash_classification(instance: i32) -> i32;
    fn document_set_document_flash_classification(instance: i32, value: i32);
}

pub fn get_document_flash_classification(instance: i32) -> i32 {
    unsafe { document_get_document_flash_classification(instance) }
}

pub fn set_document_flash_classification(instance: i32, value: i32) {
    unsafe {
        document_set_document_flash_classification(instance, value);
    }
}
