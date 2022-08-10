use crate::prelude::*;
use std::{mem::size_of, ptr};

const VERTEX_SHADER: &str = "
    #version 330 core
    layout (location = 0) in vec3 aPos;
    layout (location = 1) in vec3 aColor;

    out vec3 ourColor;

    void main() {
       gl_Position = vec4(aPos, 1.0);
       ourColor = aColor;
    }
";

const FRAGMENT_SHADER: &str = "
    #version 330 core
    out vec4 FragColor;

    in vec3 ourColor;

    void main() {
       FragColor = vec4(ourColor, 0.02f);
    }
";

pub struct Second {
    data: ProgramData,
    vertices: Vec<f32>,
}

impl Program for Second {

    fn create() -> Self {
        #[rustfmt::skip]
        let vertices: Vec<f32> = vec![
            //X     Y    Z      R    G    B
            -0.5, -0.5, 0.0,   0.0, 0.5, 0.0,
             0.0, -0.5, 0.0,   1.0, 0.0, 0.0,
             0.0,  0.0, 0.0,   1.0, 0.0, 0.0,
             0.5,  0.5, 0.0,   0.5, 0.0, 0.0,
             0.0,  0.5, 0.0,   0.0, 1.0, 0.0,
             0.0,  0.0, 0.0,   0.0, 1.0, 0.0
        ];

        let vertex_shader = compile_shader(gl::VERTEX_SHADER, VERTEX_SHADER);
        let fragment_shader = compile_shader(gl::FRAGMENT_SHADER, FRAGMENT_SHADER);
        let shader_program = link_shader(vertex_shader, fragment_shader);

        let (mut vbo, mut vao) = (0, 0);

        unsafe {
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
        }

        Second {
            data: ProgramData {
                id: shader_program,
                vao,
                vbo,
                ibo: 0,
            },
            vertices,
        }
    }

    fn data(&self) -> &ProgramData {
        &self.data
    }

    fn run(&mut self) {
        unsafe {
            // bind the Vertex Array Object first, then bind and set vertex
            // buffer(s), and then configure vertex attributes(s).
            gl::BindVertexArray(self.data.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.data.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.vertices.len() * size_of::<f32>()) as isize,
                self.vertices.as_ptr() as *const _,
                gl::DYNAMIC_DRAW,
            );

            let stride = 6 * size_of::<f32>() as i32;
            // Position attribute
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
            gl::EnableVertexAttribArray(0);

            // Color attribute
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (3 * size_of::<f32>()) as *const _,
            );
            gl::EnableVertexAttribArray(1);

            gl::DrawArrays(gl::TRIANGLES, 0, self.vertices.len() as i32 / 2_i32);

            gl::DisableVertexAttribArray(0);
            gl::DisableVertexAttribArray(1);
        }
    }
}
