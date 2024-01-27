use std::ffi::CString;
use std::ptr;
use std::time::SystemTime;
use gl::types::{GLfloat, GLint, GLuint};
use crate::shader::{Shader, ShaderError};
use crate::shaderprogram::ShaderProgram;
use crate::uniform::UniformValue;

pub struct Renderer {
    pub t: f64,
    t_start: f64,
    pub program: ShaderProgram
}

impl Renderer {
    pub fn new() -> Result<Self, ShaderError> {
        let t_start = (SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as f64) / 1000.0;
        //let FRAGMENT_SHADER_SOURCE = include_str!("shaders/graph_test.frag");
        //let FRAGMENT_SHADER_SOURCE = include_str!("shaders/mix_test.frag");
        let FRAGMENT_SHADER_SOURCE = include_str!("shaders/colorwheel.frag");

        println!("{}", FRAGMENT_SHADER_SOURCE);
        unsafe {
            let mut fragment_shader = Shader::new("Colorwheel".to_string(), FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER)?;
            fragment_shader.add_uniform("u_resolution".to_string(), UniformValue::Float(800.0));
            fragment_shader.add_uniform("u_time".to_string(), UniformValue::Float(0.0));
            let program = ShaderProgram::new(vec!(fragment_shader))?;

            Ok(Self { program, t_start, t: 0.0 })
        }
    }

    fn update_time(&mut self) {
        self.t = (SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as f64 / 1000.0) - self.t_start;
    }

    pub fn draw(&mut self) {
        self.update_time();

        let program_id = self.program.id;
        let shader = self.program.get_shader("Colorwheel".to_string()).unwrap();
        shader.update_uniform_value("u_time".to_string(), UniformValue::Float(self.t));
        shader.update_uniform_value("u_resolution".to_string(), UniformValue::Float(800.0));
        shader.apply_uniforms(program_id);

        let vertex_data: [f32; 20] = [
            -1.0, -1.0, 1.0, 0.0, 0.0, //Bottom left, red
            1.0, -1.0, 0.0, 1.0, 0.0, //Bottom right, green
            -1.0, 1.0, 0.0, 0.0, 1.0, //Top left, blue
            1.0, 1.0, 1.0, 1.0, 1.0 //Top right, white
        ];

        unsafe {
            let mut vbo: GLuint = 0;
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertex_data.len() * std::mem::size_of::<f32>()) as isize,
                vertex_data.as_ptr() as *const _,
                gl::STATIC_DRAW
            );

            let mut vao: GLuint = 0;
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 5 * std::mem::size_of::<f32>() as i32, ptr::null());
            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 5 * std::mem::size_of::<f32>() as i32, (2 * std::mem::size_of::<f32>()) as *const _);
            gl::EnableVertexAttribArray(1);
        }

        unsafe {
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            self.program.apply();
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
        }
    }
}