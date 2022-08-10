use crate::prelude::*;
use std::{mem::size_of, ptr};

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
const INDICES: [u32; 6] = [
    0, 1, 2,
    0, 2, 3,
];

const VERTEX_SHADER: &str = "
    // specify we are indeed using modern opengl
    #version 330 

    // vertex position attribute
    layout(location = 0) in vec3 vertex_position; 

    // interpolated vertex position
    out vec3 local_position; 

    void main(void) {
        local_position = vertex_position;
        // set vertex position
        gl_Position = vec4(vertex_position, 1.0); 
    }
";

const FRAGMENT_SHADER: &str = "// specify we are indeed using modern opengl
    #version 330 core

    // output of our shader
    out vec4 fragment_colour; 

    // interpolated vertex position
    in vec3 local_position;  

    void main(void) {
        // set the output colour based on the vertex position
        fragment_colour = vec4(local_position / 2.0 + 0.5, 1.0);
    }
";

pub struct First {
    data: ProgramData,
}

impl Program for First {
    fn create() -> Self {
        unsafe {
            let vertex_shader = compile_shader(gl::VERTEX_SHADER, VERTEX_SHADER);
            let fragment_shader = compile_shader(gl::FRAGMENT_SHADER, FRAGMENT_SHADER);
            let shader_program = link_shader(vertex_shader, fragment_shader);

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            // Create vertex array object
            let mut vao = 0;
            gl::GenVertexArrays(1, &mut vao);

            // Create vertex buffer object
            let mut vbo = 0;
            gl::GenBuffers(1, &mut vbo);

            // Create index buffer object
            let mut ibo = 0;
            gl::GenBuffers(1, &mut ibo);

            First {
                data: ProgramData {
                    id: shader_program,
                    vao,
                    vbo,
                    ibo,
                },
            }
        }
    }

    fn data(&self) -> &ProgramData {
        &self.data
    }

    fn run(&mut self) {
        unsafe {
            gl::BindVertexArray(self.data.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.data.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (VERTEX_POSITIONS.len() * size_of::<f32>()) as isize,
                VERTEX_POSITIONS.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // Position attribute
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());
            gl::EnableVertexAttribArray(0);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.data.ibo);

            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (size_of::<u32>() * INDICES.len()) as isize,
                INDICES.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::DrawElements(
                gl::TRIANGLES,
                INDICES.len() as i32,
                gl::UNSIGNED_INT,
                ptr::null(),
            );

            gl::DisableVertexAttribArray(0);
        }
    }
}
