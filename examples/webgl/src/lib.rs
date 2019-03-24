use web_dom::*;
#[no_mangle]
pub fn main() -> () {
    let doc = window::get_document(window());
    let canvas = document::query_selector(doc, "#screen");
    let ctx = htmlcanvas::get_context(canvas, "webgl");
    let vertices: Vec<f32> = vec![-0.5, 0.5, 0.0, -0.5, -0.5, 0.0, 0.5, -0.5, 0.0];
    let indices: Vec<u16> = vec![0, 1, 2];
    let vertices = create_uint8array(&vertices.into_bytes());
    let indices = create_uint8array(&indices.into_bytes());
    let vertex_buffer = webgl::create_buffer(ctx);
    webgl::bind_buffer(ctx,webgl::ARRAY_BUFFER,vertex_buffer);
    webgl::buffer_data(ctx,webgl::ARRAY_BUFFER,vertices,webgl::STATIC_DRAW);
    webgl::bind_buffer(ctx,webgl::ARRAY_BUFFER,NULL);
    let index_buffer = webgl::create_buffer(ctx);
    webgl::bind_buffer(ctx,webgl::ELEMENT_ARRAY_BUFFER,index_buffer);
    webgl::buffer_data(ctx,webgl::ELEMENT_ARRAY_BUFFER,indices,webgl::STATIC_DRAW);
    webgl::bind_buffer(ctx,webgl::ELEMENT_ARRAY_BUFFER,NULL);
    let vertex_shader = webgl::create_shader(ctx,webgl::VERTEX_SHADER);
    webgl::shader_source(ctx,vertex_shader,include_str!("vert.glsl"));
    webgl::compile_shader(ctx,vertex_shader);
    console::log(&webgl::get_shader_info_log(ctx,vertex_shader));
    let fragment_shader = webgl::create_shader(ctx,webgl::FRAGMENT_SHADER);
    webgl::shader_source(ctx,fragment_shader,include_str!("frag.glsl"));
    webgl::compile_shader(ctx,fragment_shader);
    console::log(&webgl::get_shader_info_log(ctx,fragment_shader));
    let program = webgl::create_program(ctx);
    webgl::attach_shader(ctx,program,vertex_shader);
    webgl::attach_shader(ctx,program,fragment_shader);
    webgl::link_program(ctx,program);
    webgl::use_program(ctx,program);
    webgl::bind_buffer(ctx,webgl::ARRAY_BUFFER,vertex_buffer);
    webgl::bind_buffer(ctx,webgl::ELEMENT_ARRAY_BUFFER,index_buffer);
    let coord = webgl::get_attrib_location(ctx,program, "coordinates");
    webgl::vertex_attrib_pointer(ctx,coord,3.0,webgl::FLOAT,false,0.0,0.0);
    webgl::enable_vertex_attrib_array(ctx,coord);
    webgl::clear_color(ctx,0.0,0.0,0.0,1.0);
    webgl::enable(ctx,webgl::DEPTH_TEST);


    webgl::clear(ctx,webgl::COLOR_BUFFER_BIT);
    webgl::clear(ctx,webgl::DEPTH_BUFFER_BIT);
    webgl::viewport(ctx,0.0,0.0,1.0,1.0);
    webgl::draw_elements(ctx,webgl::TRIANGLES,3.0,webgl::UNSIGNED_SHORT,0.0);
}
