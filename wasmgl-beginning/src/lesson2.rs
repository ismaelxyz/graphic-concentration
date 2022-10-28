use crate::{compile_shader, link_program};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::WebGl2RenderingContext;

pub fn draw(context: &WebGl2RenderingContext, vert_count: i32) {
    context.clear_color(0.4, 0.9, 0.5, 0.9);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vert_count);
}

#[wasm_bindgen]
pub fn lesson2(canvas_id: &str) -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(canvas_id).unwrap();
    let canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    let vert_shader = compile_shader(
        &context,
        WebGl2RenderingContext::VERTEX_SHADER,
        r##"#version 300 es
        in vec4 position;

        void main(){
            gl_Position = position;
        }
        "##,
    )?;

    let frag_shader = compile_shader(
        &context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        r##"#version 300 es
        precision highp float;

        out vec4 outColor;

        void main() {
            outColor = vec4(0.5, 0.8, 0.4, 0.8);
        }
        "##,
    )?;

    let program = link_program(&context, &vert_shader, &frag_shader)?;
    context.use_program(Some(&program));

    #[rustfmt::skip]
    let vertices: [f32; 9] = [
    //   X    Y    Z 
       -0.7, -0.7, 0.0, // 
        0.7, -0.7, 0.0, // 
        0.0, 0.7, 0.0   //
    ];
    let position_attribute_location = context.get_attrib_location(&program, "position");
    let buffer = context.create_buffer().ok_or("Failed to create buffer")?;
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

    // Note that `Float32Array::view` is somewhat dangerous (hence the
    //`unsafe`!). This is creating a raw view into our module's
    // `Web Assembly. Memory`buffer, butif we allocate more pages for our self
    // (aka do a memory allocationin Rust) it'll cause the buffer to change,
    // causing the`Float32Array` to be invalid.
    //
    // As a result, after`Float32Array::view` we have to be very careful not to
    // do any memory allocations before it's dropped.

    unsafe {
        let positions_array_buf_view = js_sys::Float32Array::view(&vertices);
        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &positions_array_buf_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    let vao = context
        .create_vertex_array()
        .ok_or("Could not create vertex array object")?;
    context.bind_vertex_array(Some(&vao));

    context.vertex_attrib_pointer_with_i32(0, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(position_attribute_location as u32);

    context.bind_vertex_array(Some(&vao));

    let vert_count = (vertices.len() / 3) as i32;
    draw(&context, vert_count);
    Ok(())
}
