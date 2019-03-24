use web_dom::*;

fn create_shader(win:DOMReference, ctx:DOMReference, shaderType:f32, source: &str) -> DOMReference{
    let shader = webgl::create_shader(ctx,shaderType);
    webgl::shader_source(ctx,shader,source);
    webgl::compile_shader(ctx,shader);
    let success = webgl::get_shader_parameter(ctx,shader,webgl::COMPILE_STATUS);
    console::log(&format!("{}",success));
    shader
  //gl.deleteShader(shader);
}

fn start_program(win:DOMReference,ctx:DOMReference) -> DOMReference{
    let vertex_shader = create_shader(win,ctx,webgl::VERTEX_SHADER,include_str!("vert.glsl"));
    let fragment_shader = create_shader(win,ctx,webgl::FRAGMENT_SHADER,include_str!("frag.glsl"));
    let program = webgl::create_program(ctx);
    webgl::attach_shader(ctx,program,vertex_shader);
    webgl::attach_shader(ctx,program,fragment_shader);
    webgl::link_program(ctx,program);
    webgl::use_program(ctx,program);
    program
}

#[no_mangle]
pub fn main() -> () {
    let win = window();
    let doc = window::get_document(win);
    let canvas = document::query_selector(doc, "#screen");
    let ctx = htmlcanvas::get_context(canvas, "webgl");
    let program = start_program(win,ctx);

    let vertices: Vec<f32> = vec![
            -0.5,0.5,0.0,
            0.0,0.5,0.0,
            -0.25,0.25,0.0,
         ];
    let vertices = create_f32array(&vertices.into_bytes());
    let vertex_buffer = webgl::create_buffer(ctx);
    webgl::bind_buffer(ctx,webgl::ARRAY_BUFFER,vertex_buffer);
    webgl::buffer_data(ctx,webgl::ARRAY_BUFFER,vertices,webgl::STATIC_DRAW);
    let coord = webgl::get_attrib_location(ctx,program, "a_position");
    webgl::vertex_attrib_pointer(ctx,coord,3.0,webgl::FLOAT,false,0.0,0.0);
    webgl::viewport(ctx,0.0,0.0,500.0,500.0);
    webgl::clear_color(ctx,0.0,0.0,0.0,0.0);
    webgl::clear(ctx,webgl::COLOR_BUFFER_BIT);
    webgl::enable_vertex_attrib_array(ctx,coord);
    webgl::bind_buffer(ctx,webgl::ARRAY_BUFFER,vertex_buffer);
    webgl::draw_arrays(ctx,webgl::POINTS,0.0,3.0);
}
