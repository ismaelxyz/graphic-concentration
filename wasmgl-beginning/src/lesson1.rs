// Gráficos
use glow::HasContext;

// Comunicación primitiva (no tan primitiva) entre Rust y JS
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// Vinculante con las web API´s
use web_sys::HtmlCanvasElement;

/// Obtener una Canvas dentro del archivo html actual mediante su id.
fn canvas_element(canvas_id: &str) -> Option<web_sys::HtmlCanvasElement> {
    let document = web_sys::window()?.document()?;
    let canvas = document.get_element_by_id(canvas_id)?;
    canvas.dyn_into::<web_sys::HtmlCanvasElement>().ok()
}

/// Extraer de una Canvas el contexto de gl y entregar ese contexto a glow.
fn init_webgl2(canvas: &HtmlCanvasElement) -> Option<glow::Context> {
    let gl2_ctx = canvas
        .get_context("webgl2")
        .expect("Failed to query about WebGL2 context");

    let gl2_ctx = gl2_ctx?
        .dyn_into::<web_sys::WebGl2RenderingContext>()
        .unwrap();

    Some(glow::Context::from_webgl2_context(gl2_ctx))
}

#[wasm_bindgen]
pub fn lesson1(canvas_id: &str) -> Result<(), JsValue> {
    let canvas = canvas_element(canvas_id).unwrap();
    let gl = init_webgl2(&canvas).unwrap();

    unsafe {
        gl.clear_color(0.4, 0.9, 0.5, 0.9);
        gl.clear(glow::COLOR_BUFFER_BIT);
    }

    Ok(())
}
