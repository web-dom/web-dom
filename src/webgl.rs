#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn webgl_get_canvas(instance: DOMReference) -> DOMReference;
    fn webgl_set_canvas(instance: DOMReference, value: DOMReference);
}

pub fn get_canvas(instance: DOMReference) -> DOMReference {
    unsafe { webgl_get_canvas(instance) }
}

pub fn set_canvas(instance: DOMReference, value: DOMReference) {
    unsafe {
        webgl_set_canvas(instance, value);
    }
}
extern "C" {
    fn webgl_get_drawing_buffer_width(instance: DOMReference) -> DOMReference;
    fn webgl_set_drawing_buffer_width(instance: DOMReference, value: DOMReference);
}

pub fn get_drawing_buffer_width(instance: DOMReference) -> DOMReference {
    unsafe { webgl_get_drawing_buffer_width(instance) }
}

pub fn set_drawing_buffer_width(instance: DOMReference, value: DOMReference) {
    unsafe {
        webgl_set_drawing_buffer_width(instance, value);
    }
}
extern "C" {
    fn webgl_get_drawing_buffer_height(instance: DOMReference) -> DOMReference;
    fn webgl_set_drawing_buffer_height(instance: DOMReference, value: DOMReference);
}

pub fn get_drawing_buffer_height(instance: DOMReference) -> DOMReference {
    unsafe { webgl_get_drawing_buffer_height(instance) }
}

pub fn set_drawing_buffer_height(instance: DOMReference, value: DOMReference) {
    unsafe {
        webgl_set_drawing_buffer_height(instance, value);
    }
}
extern "C" {
    fn webgl_get_context_attributes(instance: DOMReference) -> DOMReference;
}

pub fn get_context_attributes(instance: DOMReference) -> DOMReference {
    unsafe { webgl_get_context_attributes(instance) }
}
extern "C" {
    fn webgl_is_context_lost(instance: DOMReference) -> i32;
}

pub fn is_context_lost(instance: DOMReference) -> bool {
    unsafe { 0 != webgl_is_context_lost(instance) }
}
extern "C" {
    fn webgl_get_supported_extensions(instance: DOMReference) -> DOMReference;
}

pub fn get_supported_extensions(instance: DOMReference) -> DOMReference {
    unsafe { webgl_get_supported_extensions(instance) }
}
extern "C" {
    fn webgl_get_extension(instance: DOMReference, get_extension: CString) -> DOMReference;
}

pub fn get_extension(instance: DOMReference, name: &str) -> DOMReference {
    unsafe { webgl_get_extension(instance, to_cstring(name)) }
}
extern "C" {
    fn webgl_active_texture(instance: DOMReference, active_texture: DOMReference);
}

pub fn active_texture(instance: DOMReference, texture: DOMReference) {
    unsafe { webgl_active_texture(instance, texture) }
}
extern "C" {
    fn webgl_attach_shader(
        instance: DOMReference,
        attach_shader: DOMReference,
        attach_shader: DOMReference,
    );
}

pub fn attach_shader(instance: DOMReference, program: DOMReference, shader: DOMReference) {
    unsafe { webgl_attach_shader(instance, program, shader) }
}
extern "C" {
    fn webgl_bind_attrib_location(
        instance: DOMReference,
        bind_attrib_location: DOMReference,
        bind_attrib_location: DOMReference,
        bind_attrib_location: CString,
    );
}

pub fn bind_attrib_location(
    instance: DOMReference,
    program: DOMReference,
    index: DOMReference,
    name: &str,
) {
    unsafe { webgl_bind_attrib_location(instance, program, index, to_cstring(name)) }
}
extern "C" {
    fn webgl_bind_buffer(
        instance: DOMReference,
        bind_buffer: DOMReference,
        bind_buffer: DOMReference,
    );
}

pub fn bind_buffer(instance: DOMReference, target: DOMReference, buffer: DOMReference) {
    unsafe { webgl_bind_buffer(instance, target, buffer) }
}
extern "C" {
    fn webgl_bind_framebuffer(
        instance: DOMReference,
        bind_framebuffer: DOMReference,
        bind_framebuffer: DOMReference,
    );
}

pub fn bind_framebuffer(instance: DOMReference, target: DOMReference, framebuffer: DOMReference) {
    unsafe { webgl_bind_framebuffer(instance, target, framebuffer) }
}
extern "C" {
    fn webgl_bind_renderbuffer(
        instance: DOMReference,
        bind_renderbuffer: DOMReference,
        bind_renderbuffer: DOMReference,
    );
}

pub fn bind_renderbuffer(instance: DOMReference, target: DOMReference, renderbuffer: DOMReference) {
    unsafe { webgl_bind_renderbuffer(instance, target, renderbuffer) }
}
extern "C" {
    fn webgl_bind_texture(
        instance: DOMReference,
        bind_texture: DOMReference,
        bind_texture: DOMReference,
    );
}

pub fn bind_texture(instance: DOMReference, target: DOMReference, texture: DOMReference) {
    unsafe { webgl_bind_texture(instance, target, texture) }
}
extern "C" {
    fn webgl_blend_color(
        instance: DOMReference,
        blend_color: DOMReference,
        blend_color: DOMReference,
        blend_color: DOMReference,
        blend_color: DOMReference,
    );
}

pub fn blend_color(
    instance: DOMReference,
    red: DOMReference,
    green: DOMReference,
    blue: DOMReference,
    alpha: DOMReference,
) {
    unsafe { webgl_blend_color(instance, red, green, blue, alpha) }
}
extern "C" {
    fn webgl_blend_equation(instance: DOMReference, blend_equation: DOMReference);
}

pub fn blend_equation(instance: DOMReference, mode: DOMReference) {
    unsafe { webgl_blend_equation(instance, mode) }
}
extern "C" {
    fn webgl_blend_equation_separate(
        instance: DOMReference,
        blend_equation_separate: DOMReference,
        blend_equation_separate: DOMReference,
    );
}

pub fn blend_equation_separate(
    instance: DOMReference,
    mode_r_g_b: DOMReference,
    mode_alpha: DOMReference,
) {
    unsafe { webgl_blend_equation_separate(instance, mode_r_g_b, mode_alpha) }
}
extern "C" {
    fn webgl_blend_func(instance: DOMReference, blend_func: DOMReference, blend_func: DOMReference);
}

pub fn blend_func(instance: DOMReference, sfactor: DOMReference, dfactor: DOMReference) {
    unsafe { webgl_blend_func(instance, sfactor, dfactor) }
}
extern "C" {
    fn webgl_blend_func_separate(
        instance: DOMReference,
        blend_func_separate: DOMReference,
        blend_func_separate: DOMReference,
        blend_func_separate: DOMReference,
        blend_func_separate: DOMReference,
    );
}

pub fn blend_func_separate(
    instance: DOMReference,
    src_r_g_b: DOMReference,
    dst_r_g_b: DOMReference,
    src_alpha: DOMReference,
    dst_alpha: DOMReference,
) {
    unsafe { webgl_blend_func_separate(instance, src_r_g_b, dst_r_g_b, src_alpha, dst_alpha) }
}
extern "C" {
    fn webgl_check_framebuffer_status(
        instance: DOMReference,
        check_framebuffer_status: DOMReference,
    ) -> DOMReference;
}

pub fn check_framebuffer_status(instance: DOMReference, target: DOMReference) -> DOMReference {
    unsafe { webgl_check_framebuffer_status(instance, target) }
}
extern "C" {
    fn webgl_clear(instance: DOMReference, clear: DOMReference);
}

pub fn clear(instance: DOMReference, mask: DOMReference) {
    unsafe { webgl_clear(instance, mask) }
}
extern "C" {
    fn webgl_clear_color(
        instance: DOMReference,
        clear_color: DOMReference,
        clear_color: DOMReference,
        clear_color: DOMReference,
        clear_color: DOMReference,
    );
}

pub fn clear_color(
    instance: DOMReference,
    red: DOMReference,
    green: DOMReference,
    blue: DOMReference,
    alpha: DOMReference,
) {
    unsafe { webgl_clear_color(instance, red, green, blue, alpha) }
}
extern "C" {
    fn webgl_clear_depth(instance: DOMReference, clear_depth: DOMReference);
}

pub fn clear_depth(instance: DOMReference, depth: DOMReference) {
    unsafe { webgl_clear_depth(instance, depth) }
}
extern "C" {
    fn webgl_clear_stencil(instance: DOMReference, clear_stencil: DOMReference);
}

pub fn clear_stencil(instance: DOMReference, s: DOMReference) {
    unsafe { webgl_clear_stencil(instance, s) }
}
extern "C" {
    fn webgl_color_mask(
        instance: DOMReference,
        color_mask: DOMReference,
        color_mask: DOMReference,
        color_mask: DOMReference,
        color_mask: DOMReference,
    );
}

pub fn color_mask(
    instance: DOMReference,
    red: DOMReference,
    green: DOMReference,
    blue: DOMReference,
    alpha: DOMReference,
) {
    unsafe { webgl_color_mask(instance, red, green, blue, alpha) }
}
extern "C" {
    fn webgl_compile_shader(instance: DOMReference, compile_shader: DOMReference);
}

pub fn compile_shader(instance: DOMReference, shader: DOMReference) {
    unsafe { webgl_compile_shader(instance, shader) }
}
extern "C" {
    fn webgl_copy_tex_image2_d(
        instance: DOMReference,
        copy_tex_image2_d: DOMReference,
        copy_tex_image2_d: DOMReference,
        copy_tex_image2_d: DOMReference,
        copy_tex_image2_d: DOMReference,
        copy_tex_image2_d: DOMReference,
        copy_tex_image2_d: DOMReference,
        copy_tex_image2_d: DOMReference,
        copy_tex_image2_d: DOMReference,
    );
}

pub fn copy_tex_image2_d(
    instance: DOMReference,
    target: DOMReference,
    level: DOMReference,
    internalformat: DOMReference,
    x: DOMReference,
    y: DOMReference,
    width: DOMReference,
    height: DOMReference,
    border: DOMReference,
) {
    unsafe {
        webgl_copy_tex_image2_d(
            instance,
            target,
            level,
            internalformat,
            x,
            y,
            width,
            height,
            border,
        )
    }
}
extern "C" {
    fn webgl_copy_tex_sub_image2_d(
        instance: DOMReference,
        copy_tex_sub_image2_d: DOMReference,
        copy_tex_sub_image2_d: DOMReference,
        copy_tex_sub_image2_d: DOMReference,
        copy_tex_sub_image2_d: DOMReference,
        copy_tex_sub_image2_d: DOMReference,
        copy_tex_sub_image2_d: DOMReference,
        copy_tex_sub_image2_d: DOMReference,
        copy_tex_sub_image2_d: DOMReference,
    );
}

pub fn copy_tex_sub_image2_d(
    instance: DOMReference,
    target: DOMReference,
    level: DOMReference,
    xoffset: DOMReference,
    yoffset: DOMReference,
    x: DOMReference,
    y: DOMReference,
    width: DOMReference,
    height: DOMReference,
) {
    unsafe {
        webgl_copy_tex_sub_image2_d(
            instance, target, level, xoffset, yoffset, x, y, width, height,
        )
    }
}
extern "C" {
    fn webgl_create_buffer(instance: DOMReference) -> DOMReference;
}

pub fn create_buffer(instance: DOMReference) -> DOMReference {
    unsafe { webgl_create_buffer(instance) }
}
extern "C" {
    fn webgl_create_framebuffer(instance: DOMReference) -> DOMReference;
}

pub fn create_framebuffer(instance: DOMReference) -> DOMReference {
    unsafe { webgl_create_framebuffer(instance) }
}
extern "C" {
    fn webgl_create_program(instance: DOMReference) -> DOMReference;
}

pub fn create_program(instance: DOMReference) -> DOMReference {
    unsafe { webgl_create_program(instance) }
}
extern "C" {
    fn webgl_create_renderbuffer(instance: DOMReference) -> DOMReference;
}

pub fn create_renderbuffer(instance: DOMReference) -> DOMReference {
    unsafe { webgl_create_renderbuffer(instance) }
}
extern "C" {
    fn webgl_create_shader(instance: DOMReference, create_shader: DOMReference) -> DOMReference;
}

pub fn create_shader(instance: DOMReference, shader_type: DOMReference) -> DOMReference {
    unsafe { webgl_create_shader(instance, shader_type) }
}
extern "C" {
    fn webgl_create_texture(instance: DOMReference) -> DOMReference;
}

pub fn create_texture(instance: DOMReference) -> DOMReference {
    unsafe { webgl_create_texture(instance) }
}
extern "C" {
    fn webgl_cull_face(instance: DOMReference, cull_face: DOMReference);
}

pub fn cull_face(instance: DOMReference, mode: DOMReference) {
    unsafe { webgl_cull_face(instance, mode) }
}
extern "C" {
    fn webgl_delete_buffer(instance: DOMReference, delete_buffer: DOMReference);
}

pub fn delete_buffer(instance: DOMReference, buffer: DOMReference) {
    unsafe { webgl_delete_buffer(instance, buffer) }
}
extern "C" {
    fn webgl_delete_framebuffer(instance: DOMReference, delete_framebuffer: DOMReference);
}

pub fn delete_framebuffer(instance: DOMReference, framebuffer: DOMReference) {
    unsafe { webgl_delete_framebuffer(instance, framebuffer) }
}
extern "C" {
    fn webgl_delete_program(instance: DOMReference, delete_program: DOMReference);
}

pub fn delete_program(instance: DOMReference, program: DOMReference) {
    unsafe { webgl_delete_program(instance, program) }
}
extern "C" {
    fn webgl_delete_renderbuffer(instance: DOMReference, delete_renderbuffer: DOMReference);
}

pub fn delete_renderbuffer(instance: DOMReference, renderbuffer: DOMReference) {
    unsafe { webgl_delete_renderbuffer(instance, renderbuffer) }
}
extern "C" {
    fn webgl_delete_shader(instance: DOMReference, delete_shader: DOMReference);
}

pub fn delete_shader(instance: DOMReference, shader: DOMReference) {
    unsafe { webgl_delete_shader(instance, shader) }
}
extern "C" {
    fn webgl_delete_texture(instance: DOMReference, delete_texture: DOMReference);
}

pub fn delete_texture(instance: DOMReference, texture: DOMReference) {
    unsafe { webgl_delete_texture(instance, texture) }
}
extern "C" {
    fn webgl_depth_func(instance: DOMReference, depth_func: DOMReference);
}

pub fn depth_func(instance: DOMReference, func: DOMReference) {
    unsafe { webgl_depth_func(instance, func) }
}
extern "C" {
    fn webgl_depth_mask(instance: DOMReference, depth_mask: DOMReference);
}

pub fn depth_mask(instance: DOMReference, flag: DOMReference) {
    unsafe { webgl_depth_mask(instance, flag) }
}
extern "C" {
    fn webgl_depth_range(
        instance: DOMReference,
        depth_range: DOMReference,
        depth_range: DOMReference,
    );
}

pub fn depth_range(instance: DOMReference, z_near: DOMReference, z_far: DOMReference) {
    unsafe { webgl_depth_range(instance, z_near, z_far) }
}
extern "C" {
    fn webgl_detach_shader(
        instance: DOMReference,
        detach_shader: DOMReference,
        detach_shader: DOMReference,
    );
}

pub fn detach_shader(instance: DOMReference, program: DOMReference, shader: DOMReference) {
    unsafe { webgl_detach_shader(instance, program, shader) }
}
extern "C" {
    fn webgl_disable(instance: DOMReference, disable: DOMReference);
}

pub fn disable(instance: DOMReference, cap: DOMReference) {
    unsafe { webgl_disable(instance, cap) }
}
extern "C" {
    fn webgl_disable_vertex_attrib_array(
        instance: DOMReference,
        disable_vertex_attrib_array: DOMReference,
    );
}

pub fn disable_vertex_attrib_array(instance: DOMReference, index: DOMReference) {
    unsafe { webgl_disable_vertex_attrib_array(instance, index) }
}
extern "C" {
    fn webgl_draw_arrays(
        instance: DOMReference,
        draw_arrays: DOMReference,
        draw_arrays: DOMReference,
        draw_arrays: DOMReference,
    );
}

pub fn draw_arrays(
    instance: DOMReference,
    mode: DOMReference,
    first: DOMReference,
    count: DOMReference,
) {
    unsafe { webgl_draw_arrays(instance, mode, first, count) }
}
extern "C" {
    fn webgl_draw_elements(
        instance: DOMReference,
        draw_elements: DOMReference,
        draw_elements: DOMReference,
        draw_elements: DOMReference,
        draw_elements: DOMReference,
    );
}

pub fn draw_elements(
    instance: DOMReference,
    mode: DOMReference,
    count: DOMReference,
    element_type: DOMReference,
    offset: DOMReference,
) {
    unsafe { webgl_draw_elements(instance, mode, count, element_type, offset) }
}
extern "C" {
    fn webgl_enable(instance: DOMReference, enable: DOMReference);
}

pub fn enable(instance: DOMReference, cap: DOMReference) {
    unsafe { webgl_enable(instance, cap) }
}
extern "C" {
    fn webgl_enable_vertex_attrib_array(
        instance: DOMReference,
        enable_vertex_attrib_array: DOMReference,
    );
}

pub fn enable_vertex_attrib_array(instance: DOMReference, index: DOMReference) {
    unsafe { webgl_enable_vertex_attrib_array(instance, index) }
}
extern "C" {
    fn webgl_finish(instance: DOMReference);
}

pub fn finish(instance: DOMReference) {
    unsafe { webgl_finish(instance) }
}
extern "C" {
    fn webgl_flush(instance: DOMReference);
}

pub fn flush(instance: DOMReference) {
    unsafe { webgl_flush(instance) }
}
extern "C" {
    fn webgl_framebuffer_renderbuffer(
        instance: DOMReference,
        framebuffer_renderbuffer: DOMReference,
        framebuffer_renderbuffer: DOMReference,
        framebuffer_renderbuffer: DOMReference,
        framebuffer_renderbuffer: DOMReference,
    );
}

pub fn framebuffer_renderbuffer(
    instance: DOMReference,
    target: DOMReference,
    attachment: DOMReference,
    renderbuffertarget: DOMReference,
    renderbuffer: DOMReference,
) {
    unsafe {
        webgl_framebuffer_renderbuffer(
            instance,
            target,
            attachment,
            renderbuffertarget,
            renderbuffer,
        )
    }
}
extern "C" {
    fn webgl_framebuffer_texture2_d(
        instance: DOMReference,
        framebuffer_texture2_d: DOMReference,
        framebuffer_texture2_d: DOMReference,
        framebuffer_texture2_d: DOMReference,
        framebuffer_texture2_d: DOMReference,
        framebuffer_texture2_d: DOMReference,
    );
}

pub fn framebuffer_texture2_d(
    instance: DOMReference,
    target: DOMReference,
    attachment: DOMReference,
    textarget: DOMReference,
    texture: DOMReference,
    level: DOMReference,
) {
    unsafe { webgl_framebuffer_texture2_d(instance, target, attachment, textarget, texture, level) }
}
extern "C" {
    fn webgl_front_face(instance: DOMReference, front_face: DOMReference);
}

pub fn front_face(instance: DOMReference, mode: DOMReference) {
    unsafe { webgl_front_face(instance, mode) }
}
extern "C" {
    fn webgl_generate_mipmap(instance: DOMReference, generate_mipmap: DOMReference);
}

pub fn generate_mipmap(instance: DOMReference, target: DOMReference) {
    unsafe { webgl_generate_mipmap(instance, target) }
}
extern "C" {
    fn webgl_get_active_attrib(
        instance: DOMReference,
        get_active_attrib: DOMReference,
        get_active_attrib: DOMReference,
    ) -> DOMReference;
}

pub fn get_active_attrib(
    instance: DOMReference,
    program: DOMReference,
    index: DOMReference,
) -> DOMReference {
    unsafe { webgl_get_active_attrib(instance, program, index) }
}
extern "C" {
    fn webgl_get_active_uniform(
        instance: DOMReference,
        get_active_uniform: DOMReference,
        get_active_uniform: DOMReference,
    ) -> DOMReference;
}

pub fn get_active_uniform(
    instance: DOMReference,
    program: DOMReference,
    index: DOMReference,
) -> DOMReference {
    unsafe { webgl_get_active_uniform(instance, program, index) }
}
extern "C" {
    fn webgl_get_attached_shaders(
        instance: DOMReference,
        get_attached_shaders: DOMReference,
    ) -> DOMReference;
}

pub fn get_attached_shaders(instance: DOMReference, program: DOMReference) -> DOMReference {
    unsafe { webgl_get_attached_shaders(instance, program) }
}
extern "C" {
    fn webgl_get_attrib_location(
        instance: DOMReference,
        get_attrib_location: DOMReference,
        get_attrib_location: CString,
    ) -> DOMReference;
}

pub fn get_attrib_location(
    instance: DOMReference,
    program: DOMReference,
    name: &str,
) -> DOMReference {
    unsafe { webgl_get_attrib_location(instance, program, to_cstring(name)) }
}
extern "C" {
    fn webgl_get_buffer_parameter(
        instance: DOMReference,
        get_buffer_parameter: DOMReference,
        get_buffer_parameter: DOMReference,
    ) -> DOMReference;
}

pub fn get_buffer_parameter(
    instance: DOMReference,
    target: DOMReference,
    pname: DOMReference,
) -> DOMReference {
    unsafe { webgl_get_buffer_parameter(instance, target, pname) }
}
extern "C" {
    fn webgl_get_parameter(instance: DOMReference, get_parameter: DOMReference) -> DOMReference;
}

pub fn get_parameter(instance: DOMReference, pname: DOMReference) -> DOMReference {
    unsafe { webgl_get_parameter(instance, pname) }
}
extern "C" {
    fn webgl_get_error(instance: DOMReference) -> DOMReference;
}

pub fn get_error(instance: DOMReference) -> DOMReference {
    unsafe { webgl_get_error(instance) }
}
extern "C" {
    fn webgl_get_framebuffer_attachment_parameter(
        instance: DOMReference,
        get_framebuffer_attachment_parameter: DOMReference,
        get_framebuffer_attachment_parameter: DOMReference,
        get_framebuffer_attachment_parameter: DOMReference,
    ) -> DOMReference;
}

pub fn get_framebuffer_attachment_parameter(
    instance: DOMReference,
    target: DOMReference,
    attachment: DOMReference,
    pname: DOMReference,
) -> DOMReference {
    unsafe { webgl_get_framebuffer_attachment_parameter(instance, target, attachment, pname) }
}
extern "C" {
    fn webgl_get_program_parameter(
        instance: DOMReference,
        get_program_parameter: DOMReference,
        get_program_parameter: DOMReference,
    ) -> DOMReference;
}

pub fn get_program_parameter(
    instance: DOMReference,
    program: DOMReference,
    pname: DOMReference,
) -> DOMReference {
    unsafe { webgl_get_program_parameter(instance, program, pname) }
}
extern "C" {
    fn webgl_get_program_info_log(
        instance: DOMReference,
        get_program_info_log: DOMReference,
    ) -> DOMReference;
}

pub fn get_program_info_log(instance: DOMReference, program: DOMReference) -> DOMReference {
    unsafe { webgl_get_program_info_log(instance, program) }
}
extern "C" {
    fn webgl_get_renderbuffer_parameter(
        instance: DOMReference,
        get_renderbuffer_parameter: DOMReference,
        get_renderbuffer_parameter: DOMReference,
    ) -> DOMReference;
}

pub fn get_renderbuffer_parameter(
    instance: DOMReference,
    target: DOMReference,
    pname: DOMReference,
) -> DOMReference {
    unsafe { webgl_get_renderbuffer_parameter(instance, target, pname) }
}
extern "C" {
    fn webgl_get_shader_parameter(
        instance: DOMReference,
        get_shader_parameter: DOMReference,
        get_shader_parameter: DOMReference,
    ) -> DOMReference;
}

pub fn get_shader_parameter(
    instance: DOMReference,
    shader: DOMReference,
    pname: DOMReference,
) -> DOMReference {
    unsafe { webgl_get_shader_parameter(instance, shader, pname) }
}
extern "C" {
    fn webgl_get_shader_precision_format(
        instance: DOMReference,
        get_shader_precision_format: DOMReference,
        get_shader_precision_format: DOMReference,
    ) -> DOMReference;
}

pub fn get_shader_precision_format(
    instance: DOMReference,
    shadertype: DOMReference,
    precisiontype: DOMReference,
) -> DOMReference {
    unsafe { webgl_get_shader_precision_format(instance, shadertype, precisiontype) }
}
extern "C" {
    fn webgl_get_shader_info_log(
        instance: DOMReference,
        get_shader_info_log: DOMReference,
    ) -> DOMReference;
}

pub fn get_shader_info_log(instance: DOMReference, shader: DOMReference) -> DOMReference {
    unsafe { webgl_get_shader_info_log(instance, shader) }
}
extern "C" {
    fn webgl_get_shader_source(
        instance: DOMReference,
        get_shader_source: DOMReference,
    ) -> DOMReference;
}

pub fn get_shader_source(instance: DOMReference, shader: DOMReference) -> DOMReference {
    unsafe { webgl_get_shader_source(instance, shader) }
}
extern "C" {
    fn webgl_get_tex_parameter(
        instance: DOMReference,
        get_tex_parameter: DOMReference,
        get_tex_parameter: DOMReference,
    ) -> DOMReference;
}

pub fn get_tex_parameter(
    instance: DOMReference,
    target: DOMReference,
    pname: DOMReference,
) -> DOMReference {
    unsafe { webgl_get_tex_parameter(instance, target, pname) }
}
extern "C" {
    fn webgl_get_uniform(
        instance: DOMReference,
        get_uniform: DOMReference,
        get_uniform: DOMReference,
    ) -> DOMReference;
}

pub fn get_uniform(
    instance: DOMReference,
    program: DOMReference,
    location: DOMReference,
) -> DOMReference {
    unsafe { webgl_get_uniform(instance, program, location) }
}
extern "C" {
    fn webgl_get_uniform_location(
        instance: DOMReference,
        get_uniform_location: DOMReference,
        get_uniform_location: CString,
    ) -> DOMReference;
}

pub fn get_uniform_location(
    instance: DOMReference,
    program: DOMReference,
    name: &str,
) -> DOMReference {
    unsafe { webgl_get_uniform_location(instance, program, to_cstring(name)) }
}
extern "C" {
    fn webgl_get_vertex_attrib(
        instance: DOMReference,
        get_vertex_attrib: DOMReference,
        get_vertex_attrib: DOMReference,
    ) -> DOMReference;
}

pub fn get_vertex_attrib(
    instance: DOMReference,
    index: DOMReference,
    pname: DOMReference,
) -> DOMReference {
    unsafe { webgl_get_vertex_attrib(instance, index, pname) }
}
extern "C" {
    fn webgl_get_vertex_attrib_offset(
        instance: DOMReference,
        get_vertex_attrib_offset: DOMReference,
        get_vertex_attrib_offset: DOMReference,
    ) -> DOMReference;
}

pub fn get_vertex_attrib_offset(
    instance: DOMReference,
    index: DOMReference,
    pname: DOMReference,
) -> DOMReference {
    unsafe { webgl_get_vertex_attrib_offset(instance, index, pname) }
}
extern "C" {
    fn webgl_hint(instance: DOMReference, hint: DOMReference, hint: DOMReference);
}

pub fn hint(instance: DOMReference, target: DOMReference, mode: DOMReference) {
    unsafe { webgl_hint(instance, target, mode) }
}
extern "C" {
    fn webgl_is_buffer(instance: DOMReference, is_buffer: DOMReference) -> DOMReference;
}

pub fn is_buffer(instance: DOMReference, buffer: DOMReference) -> DOMReference {
    unsafe { webgl_is_buffer(instance, buffer) }
}
extern "C" {
    fn webgl_is_enabled(instance: DOMReference, is_enabled: DOMReference) -> DOMReference;
}

pub fn is_enabled(instance: DOMReference, cap: DOMReference) -> DOMReference {
    unsafe { webgl_is_enabled(instance, cap) }
}
extern "C" {
    fn webgl_is_framebuffer(instance: DOMReference, is_framebuffer: DOMReference) -> DOMReference;
}

pub fn is_framebuffer(instance: DOMReference, framebuffer: DOMReference) -> DOMReference {
    unsafe { webgl_is_framebuffer(instance, framebuffer) }
}
extern "C" {
    fn webgl_is_program(instance: DOMReference, is_program: DOMReference) -> DOMReference;
}

pub fn is_program(instance: DOMReference, program: DOMReference) -> DOMReference {
    unsafe { webgl_is_program(instance, program) }
}
extern "C" {
    fn webgl_is_renderbuffer(instance: DOMReference, is_renderbuffer: DOMReference)
        -> DOMReference;
}

pub fn is_renderbuffer(instance: DOMReference, renderbuffer: DOMReference) -> DOMReference {
    unsafe { webgl_is_renderbuffer(instance, renderbuffer) }
}
extern "C" {
    fn webgl_is_shader(instance: DOMReference, is_shader: DOMReference) -> DOMReference;
}

pub fn is_shader(instance: DOMReference, shader: DOMReference) -> DOMReference {
    unsafe { webgl_is_shader(instance, shader) }
}
extern "C" {
    fn webgl_is_texture(instance: DOMReference, is_texture: DOMReference) -> DOMReference;
}

pub fn is_texture(instance: DOMReference, texture: DOMReference) -> DOMReference {
    unsafe { webgl_is_texture(instance, texture) }
}
extern "C" {
    fn webgl_line_width(instance: DOMReference, line_width: DOMReference);
}

pub fn line_width(instance: DOMReference, width: DOMReference) {
    unsafe { webgl_line_width(instance, width) }
}
extern "C" {
    fn webgl_link_program(instance: DOMReference, link_program: DOMReference);
}

pub fn link_program(instance: DOMReference, program: DOMReference) {
    unsafe { webgl_link_program(instance, program) }
}
extern "C" {
    fn webgl_pixel_storei(
        instance: DOMReference,
        pixel_storei: DOMReference,
        pixel_storei: DOMReference,
    );
}

pub fn pixel_storei(instance: DOMReference, pname: DOMReference, param: DOMReference) {
    unsafe { webgl_pixel_storei(instance, pname, param) }
}
extern "C" {
    fn webgl_polygon_offset(
        instance: DOMReference,
        polygon_offset: DOMReference,
        polygon_offset: DOMReference,
    );
}

pub fn polygon_offset(instance: DOMReference, factor: DOMReference, units: DOMReference) {
    unsafe { webgl_polygon_offset(instance, factor, units) }
}
extern "C" {
    fn webgl_renderbuffer_storage(
        instance: DOMReference,
        renderbuffer_storage: DOMReference,
        renderbuffer_storage: DOMReference,
        renderbuffer_storage: DOMReference,
        renderbuffer_storage: DOMReference,
    );
}

pub fn renderbuffer_storage(
    instance: DOMReference,
    target: DOMReference,
    internalformat: DOMReference,
    width: DOMReference,
    height: DOMReference,
) {
    unsafe { webgl_renderbuffer_storage(instance, target, internalformat, width, height) }
}
extern "C" {
    fn webgl_sample_coverage(
        instance: DOMReference,
        sample_coverage: DOMReference,
        sample_coverage: DOMReference,
    );
}

pub fn sample_coverage(instance: DOMReference, value: DOMReference, invert: DOMReference) {
    unsafe { webgl_sample_coverage(instance, value, invert) }
}
extern "C" {
    fn webgl_scissor(
        instance: DOMReference,
        scissor: DOMReference,
        scissor: DOMReference,
        scissor: DOMReference,
        scissor: DOMReference,
    );
}

pub fn scissor(
    instance: DOMReference,
    x: DOMReference,
    y: DOMReference,
    width: DOMReference,
    height: DOMReference,
) {
    unsafe { webgl_scissor(instance, x, y, width, height) }
}
extern "C" {
    fn webgl_shader_source(
        instance: DOMReference,
        shader_source: DOMReference,
        shader_source: CString,
    );
}

pub fn shader_source(instance: DOMReference, shader: DOMReference, source: &str) {
    unsafe { webgl_shader_source(instance, shader, to_cstring(source)) }
}
extern "C" {
    fn webgl_stencil_func(
        instance: DOMReference,
        stencil_func: DOMReference,
        stencil_func: DOMReference,
        stencil_func: DOMReference,
    );
}

pub fn stencil_func(
    instance: DOMReference,
    func: DOMReference,
    setencel_ref: DOMReference,
    mask: DOMReference,
) {
    unsafe { webgl_stencil_func(instance, func, setencel_ref, mask) }
}
extern "C" {
    fn webgl_stencil_func_separate(
        instance: DOMReference,
        stencil_func_separate: DOMReference,
        stencil_func_separate: DOMReference,
        stencil_func_separate: DOMReference,
        stencil_func_separate: DOMReference,
    );
}

pub fn stencil_func_separate(
    instance: DOMReference,
    face: DOMReference,
    func: DOMReference,
    setencel_ref: DOMReference,
    mask: DOMReference,
) {
    unsafe { webgl_stencil_func_separate(instance, face, func, setencel_ref, mask) }
}
extern "C" {
    fn webgl_stencil_mask(instance: DOMReference, stencil_mask: DOMReference);
}

pub fn stencil_mask(instance: DOMReference, mask: DOMReference) {
    unsafe { webgl_stencil_mask(instance, mask) }
}
extern "C" {
    fn webgl_stencil_mask_separate(
        instance: DOMReference,
        stencil_mask_separate: DOMReference,
        stencil_mask_separate: DOMReference,
    );
}

pub fn stencil_mask_separate(instance: DOMReference, face: DOMReference, mask: DOMReference) {
    unsafe { webgl_stencil_mask_separate(instance, face, mask) }
}
extern "C" {
    fn webgl_stencil_op(
        instance: DOMReference,
        stencil_op: DOMReference,
        stencil_op: DOMReference,
        stencil_op: DOMReference,
    );
}

pub fn stencil_op(
    instance: DOMReference,
    fail: DOMReference,
    zfail: DOMReference,
    zpass: DOMReference,
) {
    unsafe { webgl_stencil_op(instance, fail, zfail, zpass) }
}
extern "C" {
    fn webgl_stencil_op_separate(
        instance: DOMReference,
        stencil_op_separate: DOMReference,
        stencil_op_separate: DOMReference,
        stencil_op_separate: DOMReference,
        stencil_op_separate: DOMReference,
    );
}

pub fn stencil_op_separate(
    instance: DOMReference,
    face: DOMReference,
    fail: DOMReference,
    zfail: DOMReference,
    zpass: DOMReference,
) {
    unsafe { webgl_stencil_op_separate(instance, face, fail, zfail, zpass) }
}
extern "C" {
    fn webgl_tex_parameterf(
        instance: DOMReference,
        tex_parameterf: DOMReference,
        tex_parameterf: DOMReference,
        tex_parameterf: DOMReference,
    );
}

pub fn tex_parameterf(
    instance: DOMReference,
    target: DOMReference,
    pname: DOMReference,
    param: DOMReference,
) {
    unsafe { webgl_tex_parameterf(instance, target, pname, param) }
}
extern "C" {
    fn webgl_tex_parameteri(
        instance: DOMReference,
        tex_parameteri: DOMReference,
        tex_parameteri: DOMReference,
        tex_parameteri: DOMReference,
    );
}

pub fn tex_parameteri(
    instance: DOMReference,
    target: DOMReference,
    pname: DOMReference,
    param: DOMReference,
) {
    unsafe { webgl_tex_parameteri(instance, target, pname, param) }
}
extern "C" {
    fn webgl_uniform1f(instance: DOMReference, uniform1f: DOMReference, uniform1f: DOMReference);
}

pub fn uniform1f(instance: DOMReference, location: DOMReference, x: DOMReference) {
    unsafe { webgl_uniform1f(instance, location, x) }
}
extern "C" {
    fn webgl_uniform2f(
        instance: DOMReference,
        uniform2f: DOMReference,
        uniform2f: DOMReference,
        uniform2f: DOMReference,
    );
}

pub fn uniform2f(instance: DOMReference, location: DOMReference, x: DOMReference, y: DOMReference) {
    unsafe { webgl_uniform2f(instance, location, x, y) }
}
extern "C" {
    fn webgl_uniform3f(
        instance: DOMReference,
        uniform3f: DOMReference,
        uniform3f: DOMReference,
        uniform3f: DOMReference,
        uniform3f: DOMReference,
    );
}

pub fn uniform3f(
    instance: DOMReference,
    location: DOMReference,
    x: DOMReference,
    y: DOMReference,
    z: DOMReference,
) {
    unsafe { webgl_uniform3f(instance, location, x, y, z) }
}
extern "C" {
    fn webgl_uniform4f(
        instance: DOMReference,
        uniform4f: DOMReference,
        uniform4f: DOMReference,
        uniform4f: DOMReference,
        uniform4f: DOMReference,
        uniform4f: DOMReference,
    );
}

pub fn uniform4f(
    instance: DOMReference,
    location: DOMReference,
    x: DOMReference,
    y: DOMReference,
    z: DOMReference,
    w: DOMReference,
) {
    unsafe { webgl_uniform4f(instance, location, x, y, z, w) }
}
extern "C" {
    fn webgl_uniform1i(instance: DOMReference, uniform1i: DOMReference, uniform1i: DOMReference);
}

pub fn uniform1i(instance: DOMReference, location: DOMReference, x: DOMReference) {
    unsafe { webgl_uniform1i(instance, location, x) }
}
extern "C" {
    fn webgl_uniform2i(
        instance: DOMReference,
        uniform2i: DOMReference,
        uniform2i: DOMReference,
        uniform2i: DOMReference,
    );
}

pub fn uniform2i(instance: DOMReference, location: DOMReference, x: DOMReference, y: DOMReference) {
    unsafe { webgl_uniform2i(instance, location, x, y) }
}
extern "C" {
    fn webgl_uniform3i(
        instance: DOMReference,
        uniform3i: DOMReference,
        uniform3i: DOMReference,
        uniform3i: DOMReference,
        uniform3i: DOMReference,
    );
}

pub fn uniform3i(
    instance: DOMReference,
    location: DOMReference,
    x: DOMReference,
    y: DOMReference,
    z: DOMReference,
) {
    unsafe { webgl_uniform3i(instance, location, x, y, z) }
}
extern "C" {
    fn webgl_uniform4i(
        instance: DOMReference,
        uniform4i: DOMReference,
        uniform4i: DOMReference,
        uniform4i: DOMReference,
        uniform4i: DOMReference,
        uniform4i: DOMReference,
    );
}

pub fn uniform4i(
    instance: DOMReference,
    location: DOMReference,
    x: DOMReference,
    y: DOMReference,
    z: DOMReference,
    w: DOMReference,
) {
    unsafe { webgl_uniform4i(instance, location, x, y, z, w) }
}
extern "C" {
    fn webgl_use_program(instance: DOMReference, use_program: DOMReference);
}

pub fn use_program(instance: DOMReference, program: DOMReference) {
    unsafe { webgl_use_program(instance, program) }
}
extern "C" {
    fn webgl_validate_program(instance: DOMReference, validate_program: DOMReference);
}

pub fn validate_program(instance: DOMReference, program: DOMReference) {
    unsafe { webgl_validate_program(instance, program) }
}
extern "C" {
    fn webgl_vertex_attrib1f(
        instance: DOMReference,
        vertex_attrib1f: DOMReference,
        vertex_attrib1f: DOMReference,
    );
}

pub fn vertex_attrib1f(instance: DOMReference, indx: DOMReference, x: DOMReference) {
    unsafe { webgl_vertex_attrib1f(instance, indx, x) }
}
extern "C" {
    fn webgl_vertex_attrib1fv(
        instance: DOMReference,
        vertex_attrib1fv: DOMReference,
        vertex_attrib1fv: DOMReference,
    );
}

pub fn vertex_attrib1fv(instance: DOMReference, indx: DOMReference, values: DOMReference) {
    unsafe { webgl_vertex_attrib1fv(instance, indx, values) }
}
extern "C" {
    fn webgl_vertex_attrib2f(
        instance: DOMReference,
        vertex_attrib2f: DOMReference,
        vertex_attrib2f: DOMReference,
        vertex_attrib2f: DOMReference,
    );
}

pub fn vertex_attrib2f(
    instance: DOMReference,
    indx: DOMReference,
    x: DOMReference,
    y: DOMReference,
) {
    unsafe { webgl_vertex_attrib2f(instance, indx, x, y) }
}
extern "C" {
    fn webgl_vertex_attrib2fv(
        instance: DOMReference,
        vertex_attrib2fv: DOMReference,
        vertex_attrib2fv: DOMReference,
    );
}

pub fn vertex_attrib2fv(instance: DOMReference, indx: DOMReference, values: DOMReference) {
    unsafe { webgl_vertex_attrib2fv(instance, indx, values) }
}
extern "C" {
    fn webgl_vertex_attrib3f(
        instance: DOMReference,
        vertex_attrib3f: DOMReference,
        vertex_attrib3f: DOMReference,
        vertex_attrib3f: DOMReference,
        vertex_attrib3f: DOMReference,
    );
}

pub fn vertex_attrib3f(
    instance: DOMReference,
    indx: DOMReference,
    x: DOMReference,
    y: DOMReference,
    z: DOMReference,
) {
    unsafe { webgl_vertex_attrib3f(instance, indx, x, y, z) }
}
extern "C" {
    fn webgl_vertex_attrib3fv(
        instance: DOMReference,
        vertex_attrib3fv: DOMReference,
        vertex_attrib3fv: DOMReference,
    );
}

pub fn vertex_attrib3fv(instance: DOMReference, indx: DOMReference, values: DOMReference) {
    unsafe { webgl_vertex_attrib3fv(instance, indx, values) }
}
extern "C" {
    fn webgl_vertex_attrib4f(
        instance: DOMReference,
        vertex_attrib4f: DOMReference,
        vertex_attrib4f: DOMReference,
        vertex_attrib4f: DOMReference,
        vertex_attrib4f: DOMReference,
        vertex_attrib4f: DOMReference,
    );
}

pub fn vertex_attrib4f(
    instance: DOMReference,
    indx: DOMReference,
    x: DOMReference,
    y: DOMReference,
    z: DOMReference,
    w: DOMReference,
) {
    unsafe { webgl_vertex_attrib4f(instance, indx, x, y, z, w) }
}
extern "C" {
    fn webgl_vertex_attrib4fv(
        instance: DOMReference,
        vertex_attrib4fv: DOMReference,
        vertex_attrib4fv: DOMReference,
    );
}

pub fn vertex_attrib4fv(instance: DOMReference, indx: DOMReference, values: DOMReference) {
    unsafe { webgl_vertex_attrib4fv(instance, indx, values) }
}
extern "C" {
    fn webgl_vertex_attrib_pointer(
        instance: DOMReference,
        vertex_attrib_pointer: DOMReference,
        vertex_attrib_pointer: DOMReference,
        vertex_attrib_pointer: DOMReference,
        vertex_attrib_pointer: DOMReference,
        vertex_attrib_pointer: DOMReference,
        vertex_attrib_pointer: DOMReference,
    );
}

pub fn vertex_attrib_pointer(
    instance: DOMReference,
    indx: DOMReference,
    size: DOMReference,
    pointer_type: DOMReference,
    normalized: DOMReference,
    stride: DOMReference,
    offset: DOMReference,
) {
    unsafe {
        webgl_vertex_attrib_pointer(
            instance,
            indx,
            size,
            pointer_type,
            normalized,
            stride,
            offset,
        )
    }
}
extern "C" {
    fn webgl_viewport(
        instance: DOMReference,
        viewport: DOMReference,
        viewport: DOMReference,
        viewport: DOMReference,
        viewport: DOMReference,
    );
}

pub fn viewport(
    instance: DOMReference,
    x: DOMReference,
    y: DOMReference,
    width: DOMReference,
    height: DOMReference,
) {
    unsafe { webgl_viewport(instance, x, y, width, height) }
}
extern "C" {
    fn webgl_buffer_data(
        instance: DOMReference,
        buffer_data: DOMReference,
        buffer_data: DOMReference,
        buffer_data: DOMReference,
    );
}

pub fn buffer_data(
    instance: DOMReference,
    target: DOMReference,
    size: DOMReference,
    usage: DOMReference,
) {
    unsafe { webgl_buffer_data(instance, target, size, usage) }
}
extern "C" {
    fn webgl_buffer_data_1(
        instance: DOMReference,
        buffer_data_1: DOMReference,
        buffer_data_1: DOMReference,
        buffer_data_1: DOMReference,
    );
}

pub fn buffer_data_1(
    instance: DOMReference,
    target: DOMReference,
    data: DOMReference,
    usage: DOMReference,
) {
    unsafe { webgl_buffer_data_1(instance, target, data, usage) }
}
extern "C" {
    fn webgl_buffer_data_2(
        instance: DOMReference,
        buffer_data_2: DOMReference,
        buffer_data_2: DOMReference,
        buffer_data_2: DOMReference,
    );
}

pub fn buffer_data_2(
    instance: DOMReference,
    target: DOMReference,
    data: DOMReference,
    usage: DOMReference,
) {
    unsafe { webgl_buffer_data_2(instance, target, data, usage) }
}
extern "C" {
    fn webgl_buffer_sub_data(
        instance: DOMReference,
        buffer_sub_data: DOMReference,
        buffer_sub_data: DOMReference,
        buffer_sub_data: DOMReference,
    );
}

pub fn buffer_sub_data(
    instance: DOMReference,
    target: DOMReference,
    offset: DOMReference,
    data: DOMReference,
) {
    unsafe { webgl_buffer_sub_data(instance, target, offset, data) }
}
extern "C" {
    fn webgl_buffer_sub_data_1(
        instance: DOMReference,
        buffer_sub_data_1: DOMReference,
        buffer_sub_data_1: DOMReference,
        buffer_sub_data_1: DOMReference,
    );
}

pub fn buffer_sub_data_1(
    instance: DOMReference,
    target: DOMReference,
    offset: DOMReference,
    data: DOMReference,
) {
    unsafe { webgl_buffer_sub_data_1(instance, target, offset, data) }
}
extern "C" {
    fn webgl_compressed_tex_image2_d(
        instance: DOMReference,
        compressed_tex_image2_d: DOMReference,
        compressed_tex_image2_d: DOMReference,
        compressed_tex_image2_d: DOMReference,
        compressed_tex_image2_d: DOMReference,
        compressed_tex_image2_d: DOMReference,
        compressed_tex_image2_d: DOMReference,
        compressed_tex_image2_d: DOMReference,
    );
}

pub fn compressed_tex_image2_d(
    instance: DOMReference,
    target: DOMReference,
    level: DOMReference,
    internalformat: DOMReference,
    width: DOMReference,
    height: DOMReference,
    border: DOMReference,
    data: DOMReference,
) {
    unsafe {
        webgl_compressed_tex_image2_d(
            instance,
            target,
            level,
            internalformat,
            width,
            height,
            border,
            data,
        )
    }
}
extern "C" {
    fn webgl_compressed_tex_sub_image2_d(
        instance: DOMReference,
        compressed_tex_sub_image2_d: DOMReference,
        compressed_tex_sub_image2_d: DOMReference,
        compressed_tex_sub_image2_d: DOMReference,
        compressed_tex_sub_image2_d: DOMReference,
        compressed_tex_sub_image2_d: DOMReference,
        compressed_tex_sub_image2_d: DOMReference,
        compressed_tex_sub_image2_d: DOMReference,
        compressed_tex_sub_image2_d: DOMReference,
    );
}

pub fn compressed_tex_sub_image2_d(
    instance: DOMReference,
    target: DOMReference,
    level: DOMReference,
    xoffset: DOMReference,
    yoffset: DOMReference,
    width: DOMReference,
    height: DOMReference,
    format: DOMReference,
    data: DOMReference,
) {
    unsafe {
        webgl_compressed_tex_sub_image2_d(
            instance, target, level, xoffset, yoffset, width, height, format, data,
        )
    }
}
extern "C" {
    fn webgl_read_pixels(
        instance: DOMReference,
        read_pixels: DOMReference,
        read_pixels: DOMReference,
        read_pixels: DOMReference,
        read_pixels: DOMReference,
        read_pixels: DOMReference,
        read_pixels: DOMReference,
        read_pixels: DOMReference,
    );
}

pub fn read_pixels(
    instance: DOMReference,
    x: DOMReference,
    y: DOMReference,
    width: DOMReference,
    height: DOMReference,
    format: DOMReference,
    pixel_type: DOMReference,
    pixels: DOMReference,
) {
    unsafe { webgl_read_pixels(instance, x, y, width, height, format, pixel_type, pixels) }
}
extern "C" {
    fn webgl_tex_image2_d(
        instance: DOMReference,
        tex_image2_d: DOMReference,
        tex_image2_d: DOMReference,
        tex_image2_d: DOMReference,
        tex_image2_d: DOMReference,
        tex_image2_d: DOMReference,
        tex_image2_d: DOMReference,
        tex_image2_d: DOMReference,
        tex_image2_d: DOMReference,
        tex_image2_d: DOMReference,
    );
}

pub fn tex_image2_d(
    instance: DOMReference,
    target: DOMReference,
    level: DOMReference,
    internalformat: DOMReference,
    width: DOMReference,
    height: DOMReference,
    border: DOMReference,
    format: DOMReference,
    image_type: DOMReference,
    pixels: DOMReference,
) {
    unsafe {
        webgl_tex_image2_d(
            instance,
            target,
            level,
            internalformat,
            width,
            height,
            border,
            format,
            image_type,
            pixels,
        )
    }
}
extern "C" {
    fn webgl_tex_image2_d_1(
        instance: DOMReference,
        tex_image2_d_1: DOMReference,
        tex_image2_d_1: DOMReference,
        tex_image2_d_1: DOMReference,
        tex_image2_d_1: DOMReference,
        tex_image2_d_1: DOMReference,
        tex_image2_d_1: DOMReference,
    );
}

pub fn tex_image2_d_1(
    instance: DOMReference,
    target: DOMReference,
    level: DOMReference,
    internalformat: DOMReference,
    format: DOMReference,
    image_type: DOMReference,
    pixels: DOMReference,
) {
    unsafe {
        webgl_tex_image2_d_1(
            instance,
            target,
            level,
            internalformat,
            format,
            image_type,
            pixels,
        )
    }
}
extern "C" {
    fn webgl_tex_image2_d_2(
        instance: DOMReference,
        tex_image2_d_2: DOMReference,
        tex_image2_d_2: DOMReference,
        tex_image2_d_2: DOMReference,
        tex_image2_d_2: DOMReference,
        tex_image2_d_2: DOMReference,
        tex_image2_d_2: DOMReference,
    );
}

pub fn tex_image2_d_2(
    instance: DOMReference,
    target: DOMReference,
    level: DOMReference,
    internalformat: DOMReference,
    format: DOMReference,
    image_type: DOMReference,
    pixels: DOMReference,
) {
    unsafe {
        webgl_tex_image2_d_2(
            instance,
            target,
            level,
            internalformat,
            format,
            image_type,
            pixels,
        )
    }
}
extern "C" {
    fn webgl_tex_image2_d_3(
        instance: DOMReference,
        tex_image2_d_3: DOMReference,
        tex_image2_d_3: DOMReference,
        tex_image2_d_3: DOMReference,
        tex_image2_d_3: DOMReference,
        tex_image2_d_3: DOMReference,
        tex_image2_d_3: DOMReference,
    );
}

pub fn tex_image2_d_3(
    instance: DOMReference,
    target: DOMReference,
    level: DOMReference,
    internalformat: DOMReference,
    format: DOMReference,
    image_type: DOMReference,
    image: DOMReference,
) {
    unsafe {
        webgl_tex_image2_d_3(
            instance,
            target,
            level,
            internalformat,
            format,
            image_type,
            image,
        )
    }
}
extern "C" {
    fn webgl_tex_image2_d_4(
        instance: DOMReference,
        tex_image2_d_4: DOMReference,
        tex_image2_d_4: DOMReference,
        tex_image2_d_4: DOMReference,
        tex_image2_d_4: DOMReference,
        tex_image2_d_4: DOMReference,
        tex_image2_d_4: DOMReference,
    );
}

pub fn tex_image2_d_4(
    instance: DOMReference,
    target: DOMReference,
    level: DOMReference,
    internalformat: DOMReference,
    format: DOMReference,
    image_type: DOMReference,
    canvas: DOMReference,
) {
    unsafe {
        webgl_tex_image2_d_4(
            instance,
            target,
            level,
            internalformat,
            format,
            image_type,
            canvas,
        )
    }
}
extern "C" {
    fn webgl_tex_image2_d_5(
        instance: DOMReference,
        tex_image2_d_5: DOMReference,
        tex_image2_d_5: DOMReference,
        tex_image2_d_5: DOMReference,
        tex_image2_d_5: DOMReference,
        tex_image2_d_5: DOMReference,
        tex_image2_d_5: DOMReference,
    );
}

pub fn tex_image2_d_5(
    instance: DOMReference,
    target: DOMReference,
    level: DOMReference,
    internalformat: DOMReference,
    format: DOMReference,
    image_type: DOMReference,
    video: DOMReference,
) {
    unsafe {
        webgl_tex_image2_d_5(
            instance,
            target,
            level,
            internalformat,
            format,
            image_type,
            video,
        )
    }
}
extern "C" {
    fn webgl_tex_sub_image2_d(
        instance: DOMReference,
        tex_sub_image2_d: DOMReference,
        tex_sub_image2_d: DOMReference,
        tex_sub_image2_d: DOMReference,
        tex_sub_image2_d: DOMReference,
        tex_sub_image2_d: DOMReference,
        tex_sub_image2_d: DOMReference,
        tex_sub_image2_d: DOMReference,
        tex_sub_image2_d: DOMReference,
        tex_sub_image2_d: DOMReference,
    );
}

pub fn tex_sub_image2_d(
    instance: DOMReference,
    target: DOMReference,
    level: DOMReference,
    xoffset: DOMReference,
    yoffset: DOMReference,
    width: DOMReference,
    height: DOMReference,
    format: DOMReference,
    image_type: DOMReference,
    pixels: DOMReference,
) {
    unsafe {
        webgl_tex_sub_image2_d(
            instance, target, level, xoffset, yoffset, width, height, format, image_type, pixels,
        )
    }
}
extern "C" {
    fn webgl_tex_sub_image2_d_1(
        instance: DOMReference,
        tex_sub_image2_d_1: DOMReference,
        tex_sub_image2_d_1: DOMReference,
        tex_sub_image2_d_1: DOMReference,
        tex_sub_image2_d_1: DOMReference,
        tex_sub_image2_d_1: DOMReference,
        tex_sub_image2_d_1: DOMReference,
        tex_sub_image2_d_1: DOMReference,
    );
}

pub fn tex_sub_image2_d_1(
    instance: DOMReference,
    target: DOMReference,
    level: DOMReference,
    xoffset: DOMReference,
    yoffset: DOMReference,
    format: DOMReference,
    image_type: DOMReference,
    pixels: DOMReference,
) {
    unsafe {
        webgl_tex_sub_image2_d_1(
            instance, target, level, xoffset, yoffset, format, image_type, pixels,
        )
    }
}
extern "C" {
    fn webgl_tex_sub_image2_d_2(
        instance: DOMReference,
        tex_sub_image2_d_2: DOMReference,
        tex_sub_image2_d_2: DOMReference,
        tex_sub_image2_d_2: DOMReference,
        tex_sub_image2_d_2: DOMReference,
        tex_sub_image2_d_2: DOMReference,
        tex_sub_image2_d_2: DOMReference,
        tex_sub_image2_d_2: DOMReference,
    );
}

pub fn tex_sub_image2_d_2(
    instance: DOMReference,
    target: DOMReference,
    level: DOMReference,
    xoffset: DOMReference,
    yoffset: DOMReference,
    format: DOMReference,
    image_type: DOMReference,
    pixels: DOMReference,
) {
    unsafe {
        webgl_tex_sub_image2_d_2(
            instance, target, level, xoffset, yoffset, format, image_type, pixels,
        )
    }
}
extern "C" {
    fn webgl_tex_sub_image2_d_3(
        instance: DOMReference,
        tex_sub_image2_d_3: DOMReference,
        tex_sub_image2_d_3: DOMReference,
        tex_sub_image2_d_3: DOMReference,
        tex_sub_image2_d_3: DOMReference,
        tex_sub_image2_d_3: DOMReference,
        tex_sub_image2_d_3: DOMReference,
        tex_sub_image2_d_3: DOMReference,
    );
}

pub fn tex_sub_image2_d_3(
    instance: DOMReference,
    target: DOMReference,
    level: DOMReference,
    xoffset: DOMReference,
    yoffset: DOMReference,
    format: DOMReference,
    image_type: DOMReference,
    image: DOMReference,
) {
    unsafe {
        webgl_tex_sub_image2_d_3(
            instance, target, level, xoffset, yoffset, format, image_type, image,
        )
    }
}
extern "C" {
    fn webgl_tex_sub_image2_d_4(
        instance: DOMReference,
        tex_sub_image2_d_4: DOMReference,
        tex_sub_image2_d_4: DOMReference,
        tex_sub_image2_d_4: DOMReference,
        tex_sub_image2_d_4: DOMReference,
        tex_sub_image2_d_4: DOMReference,
        tex_sub_image2_d_4: DOMReference,
        tex_sub_image2_d_4: DOMReference,
    );
}

pub fn tex_sub_image2_d_4(
    instance: DOMReference,
    target: DOMReference,
    level: DOMReference,
    xoffset: DOMReference,
    yoffset: DOMReference,
    format: DOMReference,
    image_type: DOMReference,
    canvas: DOMReference,
) {
    unsafe {
        webgl_tex_sub_image2_d_4(
            instance, target, level, xoffset, yoffset, format, image_type, canvas,
        )
    }
}
extern "C" {
    fn webgl_tex_sub_image2_d_5(
        instance: DOMReference,
        tex_sub_image2_d_5: DOMReference,
        tex_sub_image2_d_5: DOMReference,
        tex_sub_image2_d_5: DOMReference,
        tex_sub_image2_d_5: DOMReference,
        tex_sub_image2_d_5: DOMReference,
        tex_sub_image2_d_5: DOMReference,
        tex_sub_image2_d_5: DOMReference,
    );
}

pub fn tex_sub_image2_d_5(
    instance: DOMReference,
    target: DOMReference,
    level: DOMReference,
    xoffset: DOMReference,
    yoffset: DOMReference,
    format: DOMReference,
    image_type: DOMReference,
    video: DOMReference,
) {
    unsafe {
        webgl_tex_sub_image2_d_5(
            instance, target, level, xoffset, yoffset, format, image_type, video,
        )
    }
}
extern "C" {
    fn webgl_uniform1fv(instance: DOMReference, uniform1fv: DOMReference, uniform1fv: DOMReference);
}

pub fn uniform1fv(instance: DOMReference, location: DOMReference, data: DOMReference) {
    unsafe { webgl_uniform1fv(instance, location, data) }
}
extern "C" {
    fn webgl_uniform2fv(instance: DOMReference, uniform2fv: DOMReference, uniform2fv: DOMReference);
}

pub fn uniform2fv(instance: DOMReference, location: DOMReference, data: DOMReference) {
    unsafe { webgl_uniform2fv(instance, location, data) }
}
extern "C" {
    fn webgl_uniform3fv(instance: DOMReference, uniform3fv: DOMReference, uniform3fv: DOMReference);
}

pub fn uniform3fv(instance: DOMReference, location: DOMReference, data: DOMReference) {
    unsafe { webgl_uniform3fv(instance, location, data) }
}
extern "C" {
    fn webgl_uniform4fv(instance: DOMReference, uniform4fv: DOMReference, uniform4fv: DOMReference);
}

pub fn uniform4fv(instance: DOMReference, location: DOMReference, data: DOMReference) {
    unsafe { webgl_uniform4fv(instance, location, data) }
}
extern "C" {
    fn webgl_uniform1iv(instance: DOMReference, uniform1iv: DOMReference, uniform1iv: DOMReference);
}

pub fn uniform1iv(instance: DOMReference, location: DOMReference, data: DOMReference) {
    unsafe { webgl_uniform1iv(instance, location, data) }
}
extern "C" {
    fn webgl_uniform2iv(instance: DOMReference, uniform2iv: DOMReference, uniform2iv: DOMReference);
}

pub fn uniform2iv(instance: DOMReference, location: DOMReference, data: DOMReference) {
    unsafe { webgl_uniform2iv(instance, location, data) }
}
extern "C" {
    fn webgl_uniform3iv(instance: DOMReference, uniform3iv: DOMReference, uniform3iv: DOMReference);
}

pub fn uniform3iv(instance: DOMReference, location: DOMReference, data: DOMReference) {
    unsafe { webgl_uniform3iv(instance, location, data) }
}
extern "C" {
    fn webgl_uniform4iv(instance: DOMReference, uniform4iv: DOMReference, uniform4iv: DOMReference);
}

pub fn uniform4iv(instance: DOMReference, location: DOMReference, data: DOMReference) {
    unsafe { webgl_uniform4iv(instance, location, data) }
}
extern "C" {
    fn webgl_uniform_matrix2fv(
        instance: DOMReference,
        uniform_matrix2fv: DOMReference,
        uniform_matrix2fv: DOMReference,
        uniform_matrix2fv: DOMReference,
    );
}

pub fn uniform_matrix2fv(
    instance: DOMReference,
    location: DOMReference,
    transpose: DOMReference,
    data: DOMReference,
) {
    unsafe { webgl_uniform_matrix2fv(instance, location, transpose, data) }
}
extern "C" {
    fn webgl_uniform_matrix3fv(
        instance: DOMReference,
        uniform_matrix3fv: DOMReference,
        uniform_matrix3fv: DOMReference,
        uniform_matrix3fv: DOMReference,
    );
}

pub fn uniform_matrix3fv(
    instance: DOMReference,
    location: DOMReference,
    transpose: DOMReference,
    data: DOMReference,
) {
    unsafe { webgl_uniform_matrix3fv(instance, location, transpose, data) }
}
extern "C" {
    fn webgl_uniform_matrix4fv(
        instance: DOMReference,
        uniform_matrix4fv: DOMReference,
        uniform_matrix4fv: DOMReference,
        uniform_matrix4fv: DOMReference,
    );
}

pub fn uniform_matrix4fv(
    instance: DOMReference,
    location: DOMReference,
    transpose: DOMReference,
    data: DOMReference,
) {
    unsafe { webgl_uniform_matrix4fv(instance, location, transpose, data) }
}
extern "C" {
    fn webgl_commit(instance: DOMReference);
}

pub fn commit(instance: DOMReference) {
    unsafe { webgl_commit(instance) }
}
