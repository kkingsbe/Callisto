use std::time::SystemTime;
use rand::Rng;
use crate::particle::Particle;
extern crate nalgebra_glm as glm;

pub struct Simulation {
    pub particles: Vec<Particle>,
    pub t_start: f64,
    pub t: f64,
    pub dt: f32,
    pub attractive_force: f32,
    pub repulsive_force: f32,
}

impl Simulation {
    pub fn new(dt: f32, attractive_force: f32, repulsive_force: f32, num_particles: i32) -> Self {
        let mut rng = rand::thread_rng();
        let mut initial_state = Vec::new();
        for _ in 0..num_particles {
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

        Self {
            particles: initial_state,
            t: 0.0,
            t_start: (SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as f64) / 1000.0,
            dt,
            attractive_force,
            repulsive_force
        }
    }

    pub fn apply_force(&mut self, force: f32) {
        for i in 0..self.particles.len() {
            for j in 0..self.particles.len() {
                if i != j {
                    //Calculate attractive forces
                    let distance = self.particles[j].position - self.particles[i].position;

                    // Avoid division by zero by adding a small epsilon
                    let epsilon = 0.0001;
                    let mut a_x = force / (distance.x.powi(2) + epsilon);
                    let mut a_y = force / (distance.y.powi(2) + epsilon);

                    if distance.x < 0.0 {
                        a_x *= -1.0;
                    }

                    if distance.y < 0.0 {
                        a_y *= -1.0;
                    }

                    // Update acceleration
                    self.particles[i].new_acceleration.x += a_x;
                    self.particles[i].new_acceleration.y += a_y;
                }
            }
        }
    }

    pub fn step(&mut self) {
        self.t = (SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as f64 / 1000.0) - self.t_start;

        self.apply_force(self.attractive_force);
        self.apply_force(-self.repulsive_force);

        for particle in &mut self.particles {
            particle.update(self.dt);
        }
    }

    pub fn pack(&self) -> Vec<f32> {
        self.particles.iter()
            .flat_map(|p| p.to_flat().into_iter())
            .collect()
    }
}