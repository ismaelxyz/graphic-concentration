use crate::{compile_shader, link_program};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::WebGl2RenderingContext;

const VERTEX_SHADER: &str = include_str!("./vert.glsl");
const FRAGMENT_SHADER: &str = include_str!("./frag.glsl");

/// 3d coordinates for each vertex
#[rustfmt::skip]
const VERTEX_POSITIONS: [f32; 12] = [
    // X    Y    Z
    -0.5,  0.5, 1.0,
    -0.5, -0.5, 1.0,
     0.5, -0.5, 1.0,
     0.5,  0.5, 1.0,
];

/// Indices for the first and second triangles
#[rustfmt::skip]
const INDICES: [i32; 6] = [
    0, 1, 2,
    0, 2, 3,
];

/// Draw our first triangle
pub fn draw(context: &WebGl2RenderingContext) {
    context.clear_color(0.4, 0.9, 0.5, 0.9);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    context.draw_elements_with_i32(
        WebGl2RenderingContext::TRIANGLES,
        INDICES.len() as i32,
        WebGl2RenderingContext::UNSIGNED_INT,
        0,
    );
}

#[wasm_bindgen]
pub fn lesson3(canvas_id: &str) -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(canvas_id).unwrap();
    let canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    // Get WebGL context from the browser
    let context = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    let vert_shader = compile_shader(
        &context,
        WebGl2RenderingContext::VERTEX_SHADER,
        VERTEX_SHADER,
    )?;

    let frag_shader = compile_shader(
        &context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        FRAGMENT_SHADER,
    )?;

    let program = link_program(&context, &vert_shader, &frag_shader)?;
    context.use_program(Some(&program));

    // Create vertex array object
    let vao = context
        .create_vertex_array()
        .ok_or("Could not create vertex array object")?;
    context.bind_vertex_array(Some(&vao));

    // Create vertex buffer object
    let position_attribute_location = context.get_attrib_location(&program, "vertex_position");
    let vbo = context
        .create_buffer()
        .ok_or("Could not create vertex buffer object")?;
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vbo));

    unsafe {
        let positions_vbo_view = js_sys::Float32Array::view(&VERTEX_POSITIONS);
        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &positions_vbo_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    context.vertex_attrib_pointer_with_i32(0, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(position_attribute_location as u32);

    // Create index buffer object
    let ibo = context
        .create_buffer()
        .ok_or("Could not create index buffer object")?;
    context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&ibo));

    unsafe {
        let positions_ibo_view = js_sys::Int32Array::view(&INDICES);
        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            &positions_ibo_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    draw(&context);
    Ok(())
}
