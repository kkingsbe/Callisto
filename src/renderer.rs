use std::ptr;
use gl::types::GLuint;
use crate::shader::{Shader, ShaderError};
use crate::shaderprogram::ShaderProgram;
use crate::uniform::UniformValue;
extern crate nalgebra_glm as glm;
use crate::simulation::Simulation;

pub struct Renderer {
    pub program: ShaderProgram,
    pub simulation: Simulation
}

impl Renderer {
    pub fn new() -> Result<Self, ShaderError> {
        let FRAGMENT_SHADER_SOURCE = include_str!("shaders/visualize.frag");

        println!("{}", FRAGMENT_SHADER_SOURCE);

        let simulation: Simulation = Default::default();

        unsafe {
            let mut fragment_shader = Shader::new("visualize".to_string(), FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER)?;
            fragment_shader.add_uniform("u_resolution".to_string(), UniformValue::Float(800.0));
            fragment_shader.add_uniform("u_time".to_string(), UniformValue::Float(0.0));
            fragment_shader.add_uniform("u_tracer_data".to_string(), UniformValue::Array_F(vec!(0.0, 0.0).repeat(simulation.particles.len())));

            let program = ShaderProgram::new(vec!(fragment_shader))?;

            Ok(Self { program, simulation })
        }
    }

    pub fn draw(&mut self) {
        self.simulation.step();

        let program_id = self.program.id;
        let shader = self.program.get_shader("visualize".to_string()).unwrap();

        shader.update_uniform_value("u_time".to_string(), UniformValue::Float(self.simulation.t));
        shader.update_uniform_value("u_resolution".to_string(), UniformValue::Float(800.0));
        shader.update_uniform_value("u_tracer_data".to_string(), UniformValue::Array_F(self.simulation.pack()));
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