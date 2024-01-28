use std::ptr;
use std::time::SystemTime;
use gl::types::GLuint;
use rand::Rng;
use crate::shader::{Shader, ShaderError};
use crate::shaderprogram::ShaderProgram;
use crate::uniform::UniformValue;
extern crate nalgebra_glm as glm;
use crate::particle::Particle;

const num_particles: usize = 100;

pub struct Renderer {
    pub t: f64,
    t_start: f64,
    rng: rand::rngs::ThreadRng,
    pub program: ShaderProgram,
    particles: Vec<Particle>
}

impl Renderer {
    pub fn new() -> Result<Self, ShaderError> {
        let t_start = (SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as f64) / 1000.0;
        let FRAGMENT_SHADER_SOURCE = include_str!("shaders/visualize.frag");
        //let VERTEX_SHADER_SOURCE = include_str!("shaders/test3d.vert");

        println!("{}", FRAGMENT_SHADER_SOURCE);

        let mut rng = rand::thread_rng();
        let mut initial_state = Vec::new(); //vec!(Particle::new(rand::thread_rng(), glm::vec2(0.5, 0.5), glm::vec2(0.0, 0.0))).repeat(100);
        for i in 0..num_particles {
            initial_state.push(Particle::new(
                glm::vec2(
                    rng.gen_range(0.1..0.9),
                    rng.gen_range(0.1..0.9)
                ),
                glm::vec2(
                    rng.gen_range(-0.2..0.2),
                    rng.gen_range(-0.2..0.2)
                )
            ));
        }

        println!("{:#?}", initial_state);

        unsafe {
            let mut fragment_shader = Shader::new("visualize".to_string(), FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER)?;
            fragment_shader.add_uniform("u_resolution".to_string(), UniformValue::Float(800.0));
            fragment_shader.add_uniform("u_time".to_string(), UniformValue::Float(0.0));
            fragment_shader.add_uniform("u_tracer_data".to_string(), UniformValue::Array_F(vec!(0.0, 0.0).repeat(num_particles)));

            let program = ShaderProgram::new(vec!(fragment_shader))?;


            Ok(Self { program, t_start, t: 0.0, rng, particles: initial_state })
        }
    }

    fn update_time(&mut self) {
        self.t = (SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as f64 / 1000.0) - self.t_start;
    }

    pub fn draw(&mut self) {
        self.update_time();

        for i in 0..self.particles.len() {
            self.particles[i].new_acceleration = glm::vec2(0.0, 0.0);

            for j in 0..self.particles.len() {
                if i != j {
                    let distance = self.particles[j].position - self.particles[i].position;

                    // Avoid division by zero by adding a small epsilon
                    let epsilon = 0.0001;
                    let a_x = 0.1 / (distance.x.powi(2) + epsilon);
                    let a_y = 0.1 / (distance.y.powi(2) + epsilon);

                    // Update acceleration
                    self.particles[i].new_acceleration.x += a_x;
                    self.particles[i].new_acceleration.y += a_y;
                }
            }
        }

        for particle in &mut self.particles {
            particle.update(0.01 / 1000.0);
        }

        let program_id = self.program.id;
        let shader = self.program.get_shader("visualize".to_string()).unwrap();

        shader.update_uniform_value("u_time".to_string(), UniformValue::Float(self.t));
        shader.update_uniform_value("u_resolution".to_string(), UniformValue::Float(800.0));
        shader.update_uniform_value("u_tracer_data".to_string(), UniformValue::Array_F(
            self.particles.iter()
                .flat_map(|p| p.to_flat().into_iter())
                .collect()
        ));
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