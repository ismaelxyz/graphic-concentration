mod lesson1;
mod lesson2;
mod lesson3;

use std::sync::{Arc, Mutex};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

pub(crate) fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub(crate) fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct LessonCursor {
    position: u8,
    canvas_id: String,
}

const LESSON_MAX: u8 = 3;
const LESSON_MIN: u8 = 1;

#[wasm_bindgen]
impl LessonCursor {
    #[wasm_bindgen(constructor)]
    pub fn new(id: &str) -> Self {
        Self {
            position: 0,
            canvas_id: id.into(),
        }
    }

    #[inline(always)]
    fn display(&self) {
        match self.position {
            1 => lesson1::lesson1(&self.canvas_id),
            2 => lesson2::lesson2(&self.canvas_id),
            3 => lesson3::lesson3(&self.canvas_id),
            _ => Ok(()),
        }
        .unwrap()
    }

    pub fn next(&mut self) {
        if self.position < LESSON_MAX {
            self.position += 1;
        }

        self.display();
    }

    pub fn back(&mut self) {
        if self.position > LESSON_MIN {
            self.position -= 1;
        }

        self.display();
    }
}


#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let lessons = Arc::new(Mutex::new(LessonCursor::new("app-canvas")));
    let qlessons = Arc::clone(&lessons);

    let func =
        Closure::wrap(Box::new(move || (*lessons.lock().unwrap()).back()) as Box<dyn FnMut()>);
    let back = document
        .query_selector("#screen #back")?
        .unwrap()
        .dyn_into::<web_sys::HtmlButtonElement>()?;
    back.set_onclick(Some(func.as_ref().unchecked_ref()));

    let func2 =
        Closure::wrap(Box::new(move || (*qlessons.lock().unwrap()).next()) as Box<dyn FnMut()>);
    let next = document
        .query_selector("#screen #next")?
        .unwrap()
        .dyn_into::<web_sys::HtmlButtonElement>()?;
    next.set_onclick(Some(func2.as_ref().unchecked_ref()));

    func.forget();
    func2.forget();

    /*
        rustup target add wasm32-unknown-emscripten
        wasm32-unknown-emscripten
        wasm32-unknown-unknown
        wasm32-wasi
    */


    Ok(())
}
