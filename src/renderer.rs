use std::ptr;
use gl::types::GLuint;
use crate::shader::{Shader, ShaderError};
use crate::shaderprogram::ShaderProgram;
use crate::uniform::UniformValue;
extern crate nalgebra_glm as glm;
use crate::simulation::Simulation;

pub enum KEY {
    LCTRL
}
pub struct Renderer {
    pub program: ShaderProgram,
    pub simulation: Simulation,
    active_particle_index: usize,
    mouse_position: glm::Vec2,
    screensize: glm::Vec2
}

impl Renderer {
    pub fn new() -> Result<Self, ShaderError> {
        let FRAGMENT_SHADER_SOURCE = include_str!("shaders/visualize.frag");

        println!("{}", FRAGMENT_SHADER_SOURCE);

        let simulation: Simulation = Default::default();

        unsafe {
            let mut fragment_shader = Shader::new("visualize".to_string(), FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER)?;
            fragment_shader.add_uniform("u_mouse_active".to_string(), UniformValue::Bool(false));
            fragment_shader.add_uniform("u_mouse_attractive".to_string(), UniformValue::Bool(true));
            fragment_shader.add_uniform("u_mouse_position".to_string(), UniformValue::Vec2(glm::vec2(0.0, 0.0)));
            fragment_shader.add_uniform("u_resolution".to_string(), UniformValue::Vec2(glm::vec2(0.0, 0.0)));
            fragment_shader.add_uniform("u_time".to_string(), UniformValue::Float(0.0));
            fragment_shader.add_uniform("u_tracer_data".to_string(), UniformValue::Array_F(vec!(0.0, 0.0).repeat(simulation.particles.len())));

            let program = ShaderProgram::new(vec!(fragment_shader))?;

            Ok(Self { program, simulation, mouse_position: glm::vec2(0.0, 0.0), active_particle_index: 0, screensize: glm::vec2(800.0, 800.0) })
        }
    }

    pub fn set_mouse_position(&mut self, x: f32, y: f32) {
        self.mouse_position = glm::vec2(x, y);
        self.simulation.set_mouse_position(x / self.screensize.x, y / self.screensize.y);
    }

    pub fn set_screen_size(&mut self, width: f32, height: f32) {
        self.screensize = glm::vec2(width, height);
        //self.simulation.set_screen_size(width, height);
    }

    pub fn on_mouse_click(&mut self) {
        self.simulation.on_mouse_click();
    }

    pub fn on_keypress(&mut self, key: KEY) {
        match key {
            KEY::LCTRL => {
                self.simulation.next_mouse_mode();
            }
        }
    }

    fn update_uniforms(&mut self) {
        let program_id = self.program.id;
        let shader = self.program.get_shader("visualize".to_string()).unwrap();
        shader.update_uniform_value("u_mouse_active".to_string(), UniformValue::Bool(self.simulation.mouse_active));
        shader.update_uniform_value("u_mouse_attractive".to_string(), UniformValue::Bool(self.simulation.mouse_state == crate::simulation::MOUSE_STATE::ATTRACTIVE));
        shader.update_uniform_value("u_mouse_position".to_string(), UniformValue::Vec2(self.mouse_position));
        shader.update_uniform_value("u_time".to_string(), UniformValue::Float(self.simulation.t));
        shader.update_uniform_value("u_resolution".to_string(), UniformValue::Vec2(self.screensize));
        shader.update_uniform_value("u_tracer_data".to_string(), UniformValue::Array_F(self.simulation.particles[self.active_particle_index].to_flat()));
        shader.apply_uniforms(program_id);
    }

    pub fn draw(&mut self) {
        self.simulation.step();
        self.update_uniforms();

        let vertex_data: [f32; 20] = [
            -1.0, -1.0, 1.0, 0.0, 0.0, //Bottom left, red
            1.0, -1.0, 0.0, 1.0, 0.0, //Bottom right, green
            -1.0, 1.0, 0.0, 0.0, 1.0, //Top left, blue
            1.0, 1.0, 1.0, 1.0, 1.0 //Top right, white
        ];

        unsafe {
            let mut vbo: GLuint = 0;
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
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

            for i in 0..self.simulation.particles.len() {
                self.active_particle_index = i;
                self.update_uniforms();
                self.program.apply();
                gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
            }
        }
    }
}