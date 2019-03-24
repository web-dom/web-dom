#[allow(unused_imports)]
use crate::*;
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
