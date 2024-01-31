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
    pub density_map_program: ShaderProgram,
    pub visualize_program: ShaderProgram,
    pub simulation: Simulation,
    active_particle_index: usize,
    mouse_position: glm::Vec2,
    screensize: glm::Vec2
}

impl Renderer {
    pub fn new() -> Result<Self, ShaderError> {
        let VISUALIZE_SHADER_SOURCE = include_str!("shaders/visualize.frag");
        let DENSIY_MAP_SHADER_SOURCE = include_str!("shaders/densitymap.frag");

        println!("{}", VISUALIZE_SHADER_SOURCE);
        println!("{}", DENSIY_MAP_SHADER_SOURCE);

        let simulation: Simulation = Default::default();

        unsafe {
            let mut density_map = Shader::new("density_map".to_string(), DENSIY_MAP_SHADER_SOURCE, gl::FRAGMENT_SHADER)?;
            density_map.add_uniform("u_resolution".to_string(), UniformValue::Vec2(glm::vec2(0.0, 0.0)));
            density_map.add_uniform("u_tracer_data".to_string(), UniformValue::Vec2(glm::vec2(0.0, 0.0)));

            let mut visualize = Shader::new("visualize".to_string(), VISUALIZE_SHADER_SOURCE, gl::FRAGMENT_SHADER)?;
            visualize.add_uniform("u_mouse_active".to_string(), UniformValue::Bool(false));
            visualize.add_uniform("u_mouse_attractive".to_string(), UniformValue::Bool(true));
            visualize.add_uniform("u_mouse_position".to_string(), UniformValue::Vec2(glm::vec2(0.0, 0.0)));
            visualize.add_uniform("u_resolution".to_string(), UniformValue::Vec2(glm::vec2(0.0, 0.0)));
            visualize.add_uniform("u_time".to_string(), UniformValue::Float(0.0));

            let density_map_program = ShaderProgram::new(vec!(density_map))?;
            let visualize_program = ShaderProgram::new(vec!(visualize))?;

            Ok(Self { density_map_program, visualize_program, simulation, mouse_position: glm::vec2(0.0, 0.0), active_particle_index: 0, screensize: glm::vec2(800.0, 800.0) })
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
        let program_id = self.density_map_program.id;
        let density_map_shader = self.density_map_program.get_shader("density_map".to_string()).unwrap();
        density_map_shader.update_uniform_value("u_resolution".to_string(), UniformValue::Vec2(self.screensize));
        density_map_shader.update_uniform_value("u_tracer_data".to_string(), UniformValue::Array_F(self.simulation.particles[self.active_particle_index].to_flat()));
        density_map_shader.apply_uniforms(program_id);

        let program_id = self.visualize_program.id;
        let visualize_shader = self.visualize_program.get_shader("visualize".to_string()).unwrap();
        visualize_shader.update_uniform_value("u_mouse_active".to_string(), UniformValue::Bool(self.simulation.mouse_active));
        visualize_shader.update_uniform_value("u_mouse_attractive".to_string(), UniformValue::Bool(self.simulation.mouse_state == crate::simulation::MOUSE_STATE::ATTRACTIVE));
        visualize_shader.update_uniform_value("u_mouse_position".to_string(), UniformValue::Vec2(self.mouse_position));
        visualize_shader.update_uniform_value("u_time".to_string(), UniformValue::Float(self.simulation.t));
        visualize_shader.update_uniform_value("u_resolution".to_string(), UniformValue::Vec2(self.screensize));
        visualize_shader.apply_uniforms(program_id);
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
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            for i in 0..self.simulation.particles.len() {
                self.active_particle_index = i;
                self.update_uniforms();
                self.density_map_program.apply();
                gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
            }

            //self.visualize_program.apply();
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
        }
    }
}