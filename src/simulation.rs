use std::time::SystemTime;
use rand::prelude::ThreadRng;
use rand::Rng;
use crate::particle::Particle;
extern crate nalgebra_glm as glm;

const EPSILON: f32 = 0.000000001;

pub struct Simulation {
    pub particles: Vec<Particle>,
    pub t_start: f64,
    pub t: f64,
    pub dt: f32,
    pub attractive_force: f32,
    pub repulsive_force: f32
}

impl Simulation {
    pub fn new(dt: f32, attractive_force: f32, repulsive_force: f32, num_particles: i32) -> Self {
        let mut rng = rand::thread_rng();
        let mut initial_state = Vec::new();
        for _ in 0..num_particles {
            initial_state.push(Particle::new(
                Simulation::rand_coord(rng.clone()),
                glm::vec2(
                    rng.gen_range(-0.5..0.9),
                    rng.gen_range(-0.5..0.9)
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

    fn remap_trig(value: f32) -> f32 {
        (value + 1.0) / 2.0
    }

    pub fn rand_coord(mut rng: ThreadRng) -> glm::Vec2 {
        let theta = rng.gen_range(0.0..(2.0 * std::f32::consts::PI));
        let max_r = 0.9;
        let shift = (1.0 - max_r) / 2.0;
        let r = rng.gen_range(0.8..max_r);
        glm::vec2(
            Simulation::remap_trig(theta.cos()) * r + shift,
            Simulation::remap_trig(theta.sin()) * r + shift
        )
    }

    pub fn apply_force(&mut self, force: f32, is_attractive: bool) {
        for i in 0..self.particles.len() {
            for j in 0..self.particles.len() {
                if i != j {
                    let distance = self.particles[j].position - self.particles[i].position;
                    let mut potential = glm::vec2(0.0, 0.0);

                    if is_attractive {
                        potential.x = -1.0 * self.isl_potential(distance.x);
                        potential.y = -1.0 * self.isl_potential(distance.y);
                    } else {
                        potential.x = self.lj_potential(distance.x);
                        potential.y = self.lj_potential(distance.y);
                    }

                    // Avoid division by zero by adding a small epsilon
                    let mut a_x = force * potential.x;
                    let mut a_y = force * potential.y;

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


    fn lj_potential(&self, r: f32) -> f32 {
        let r6 = r.powi(6);
        let r12 = r6.powi(2);
        4.0 * (1.0 / (r12 + EPSILON) - 1.0 / (r6 + EPSILON))
    }

    fn isl_potential(&self, r: f32) -> f32 {
        1.0 / (r.powi(2) + EPSILON)
    }

    pub fn step(&mut self) {
        self.t = (SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as f64 / 1000.0) - self.t_start;

        self.apply_force(self.attractive_force, true);
        self.apply_force(self.repulsive_force, false);

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