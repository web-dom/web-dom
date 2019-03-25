use web_dom::*;

fn create_shader(ctx: DOMReference, shader_type: f32, source: &str) -> DOMReference {
    let shader = webgl::create_shader(ctx, shader_type);
    webgl::shader_source(ctx, shader, source);
    webgl::compile_shader(ctx, shader);
    shader
}

fn start_program(ctx: DOMReference) -> DOMReference {
    let vertex_shader = create_shader(ctx, webgl::VERTEX_SHADER, include_str!("vert.glsl"));
    let fragment_shader = create_shader(ctx, webgl::FRAGMENT_SHADER, include_str!("frag.glsl"));
    let program = webgl::create_program(ctx);
    webgl::attach_shader(ctx, program, vertex_shader);
    webgl::attach_shader(ctx, program, fragment_shader);
    webgl::link_program(ctx, program);
    webgl::use_program(ctx, program);
    program
}

#[no_mangle]
pub fn main() -> () {
    let win = window();
    let doc = window::get_document(win);
    let canvas = document::query_selector(doc, "#screen");
    let ctx = htmlcanvas::get_context(canvas, "webgl");

    webgl::clear_color(ctx, 0.75, 0.85, 0.8, 1.0);
    webgl::clear(ctx, webgl::COLOR_BUFFER_BIT);

    // create a program and get its attribute and uniforms
    let program = start_program(ctx);
    let position_location = webgl::get_attrib_location(ctx, program, "vertPosition");
    let color_location = webgl::get_attrib_location(ctx, program, "vertColor");
    webgl::use_program(ctx, NULL);

    // setup data buffer
    let vertices: Vec<f32> = vec![
        // X, Y,       R, G, B
        0.0, 0.5, 1.0, 1.0, 0.0, -0.5, -0.5, 0.7, 0.0, 1.0, 0.5, -0.5, 0.1, 1.0, 0.6,
    ];
    let vertices = create_f32array(&vertices.into_bytes());
    let vertex_buffer = webgl::create_buffer(ctx);
    webgl::bind_buffer(ctx, webgl::ARRAY_BUFFER, vertex_buffer);
    webgl::buffer_data(ctx, webgl::ARRAY_BUFFER, vertices, webgl::STATIC_DRAW);
    webgl::bind_buffer(ctx, webgl::ARRAY_BUFFER, NULL);

    // setup for drawing
    webgl::use_program(ctx, program);

    // draw
    webgl::bind_buffer(ctx, webgl::ARRAY_BUFFER, vertex_buffer);
    webgl::enable_vertex_attrib_array(ctx, position_location);
    webgl::enable_vertex_attrib_array(ctx, color_location);
    webgl::vertex_attrib_pointer(
        ctx,
        position_location,
        2.0,
        webgl::FLOAT,
        false,
        5.0 * 4.0,
        0.0,
    );
    webgl::vertex_attrib_pointer(
        ctx,
        color_location,
        3.0,
        webgl::FLOAT,
        false,
        5.0 * 4.0,
        2.0 * 4.0,
    );
    webgl::bind_buffer(ctx, webgl::ARRAY_BUFFER, NULL);

    webgl::draw_arrays(ctx, webgl::TRIANGLES, 0.0, 3.0);
}
