use std::time::SystemTime;
use rand::prelude::ThreadRng;
use rand::Rng;
use crate::particle::Particle;
extern crate nalgebra_glm as glm;

#[derive(PartialEq)]
pub enum FORCE_TYPE {
    ISL,
    LJ,
    PROPORTIONAL,
    MOUSE,
    GRAVITY
}

#[derive(PartialEq)]
pub enum MOUSE_STATE {
    ATTRACTIVE,
    REPULSIVE
}

#[derive(PartialEq, Clone, Debug, Copy)]
pub enum DOMAIN_MODE {
    WRAP,
    INFINITE,
    WALL
}

const EPSILON: f32 = 0.000000001;

pub struct Simulation {
    pub particles: Vec<Particle>,
    pub t_start: f64,
    pub t: f64,
    pub dt: f32,
    pub attractive_force: f32,
    pub repulsive_force: f32,
    pub drag: f32,
    pub microsteps: i32,
    mouse_position: glm::Vec2, //Normalized (0,1)
    pub mouse_state: MOUSE_STATE,
    pub mouse_active: bool,
    gravity: bool
}

impl Default for Simulation {
    fn default() -> Self {
        Self::new(
            0.01 / 1000.0,
            0.001,
            0.000002,
            0.5,
            10.0,
            200,
            1,
            false,
            DOMAIN_MODE::WALL
        )
    }
}

impl Simulation {
    pub fn new(dt: f32, attractive_force: f32, repulsive_force: f32, drag: f32, max_spawn_velocity: f32, num_particles: i32, microsteps: i32, gravity: bool, domain_mode: DOMAIN_MODE) -> Self {
        let mut rng = rand::thread_rng();
        let mut initial_state = Vec::new();
        for _ in 0..num_particles {
            initial_state.push(Particle::new(
                Simulation::rand_coord(rng.clone()),
                glm::vec2(
                    rng.gen_range(-max_spawn_velocity..max_spawn_velocity),
                    rng.gen_range(-max_spawn_velocity..max_spawn_velocity)
                ),
                domain_mode
            ));
        }

        Self {
            particles: initial_state,
            t: 0.0,
            t_start: (SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as f64) / 1000.0,
            dt,
            attractive_force,
            repulsive_force,
            drag,
            microsteps,
            mouse_position: glm::vec2(0.0, 0.0),
            mouse_state: MOUSE_STATE::ATTRACTIVE,
            mouse_active: false,
            gravity
        }
    }

    pub fn set_mouse_position(&mut self, x: f32, y: f32) {
        self.mouse_position = glm::vec2(x, y);
    }

    pub fn on_mouse_click(&mut self) {
        self.mouse_active = !self.mouse_active;
    }

    pub fn next_mouse_mode(&mut self) {
        self.mouse_state = if self.mouse_state == MOUSE_STATE::ATTRACTIVE { MOUSE_STATE::REPULSIVE } else { MOUSE_STATE::ATTRACTIVE };
    }

    fn remap_trig(value: f32) -> f32 {
        (value + 1.0) / 2.0
    }

    pub fn rand_coord(mut rng: ThreadRng) -> glm::Vec2 {
        let theta = rng.gen_range(0.0..(2.0 * std::f32::consts::PI));
        let max_r = 0.7;
        let shift = (1.0 - max_r) / 2.0;
        let r = rng.gen_range(0.6..max_r);
        glm::vec2(
            (Simulation::remap_trig(theta.cos()) * r) + shift,
            (Simulation::remap_trig(theta.sin()) * r) + shift
        )
    }

    pub fn apply_force(&mut self, force: f32, force_type: FORCE_TYPE) {
        for i in 0..self.particles.len() {
            if force_type == FORCE_TYPE::PROPORTIONAL {
                let vx_sign = self.particles[i].velocity.x.signum();
                let vy_sign = self.particles[i].velocity.y.signum();

                self.particles[i].new_acceleration.x -= vx_sign * self.drag * self.particles[i].velocity.x.powi(2);
                self.particles[i].new_acceleration.y -= vy_sign * self.drag * self.particles[i].velocity.y.powi(2);
                continue;
            }

            if force_type == FORCE_TYPE::GRAVITY {
                self.particles[i].new_acceleration.y -= force;
                continue;
            }

            if force_type == FORCE_TYPE::MOUSE {
                if !self.mouse_active {
                    continue;
                }

                let distance = self.mouse_position - self.particles[i].position;
                let mut potential = glm::vec2(0.0, 0.0);
                potential.x = if self.mouse_state == MOUSE_STATE::REPULSIVE { -1.0 } else { 1.0 } * self.lj_potential(distance.x);
                potential.y = if self.mouse_state == MOUSE_STATE::REPULSIVE { -1.0 } else { 1.0 } * self.lj_potential(distance.y);

                let mut a_x = force * potential.x;
                let mut a_y = force * potential.y;

                if distance.x < 0.0 {
                    a_x *= -1.0;
                }

                if distance.y < 0.0 {
                    a_y *= -1.0;
                }

                self.particles[i].new_acceleration.x += a_x;
                self.particles[i].new_acceleration.y += a_y;
                continue;
            }

            for j in 0..self.particles.len() {
                if i != j {
                    let distance = self.particles[j].position - self.particles[i].position;
                    let mut potential = glm::vec2(0.0, 0.0);

                    match force_type {
                        FORCE_TYPE::ISL => {
                            potential.x = -1.0 * self.isl_potential(distance.x);
                            potential.y = -1.0 * self.isl_potential(distance.y);
                        },
                        FORCE_TYPE::LJ => {
                            potential.x = self.lj_potential(distance.x);
                            potential.y = self.lj_potential(distance.y);
                        },
                        _ => {}
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
        for _ in 0..self.microsteps {
            self.microstep();
        }
    }

    fn microstep(&mut self) {
        self.t = (SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as f64 / 1000.0) - self.t_start;

        self.apply_force(self.attractive_force, FORCE_TYPE::ISL);
        self.apply_force(self.repulsive_force, FORCE_TYPE::LJ);

        if self.gravity {
            self.apply_force(10000000.0, FORCE_TYPE::GRAVITY);
        }

        self.apply_force(self.drag, FORCE_TYPE::PROPORTIONAL);
        self.apply_force(0.001, FORCE_TYPE::MOUSE);

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