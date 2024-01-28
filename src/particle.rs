extern crate nalgebra_glm as glm;

use rand::prelude::ThreadRng;
use rand::Rng;

#[derive(Clone, Debug)]
pub struct Particle {
    pub position: glm::Vec2,
    velocity: glm::Vec2,
    acceleration: glm::Vec2,
    pub new_acceleration: glm::Vec2
}

impl Particle {
    pub fn new(position: glm::Vec2, velocity: glm::Vec2) -> Self {
        Self {
            position,
            velocity,
            acceleration: glm::vec2(0.0, 0.0),
            new_acceleration: glm::vec2(0.0, 0.0)
        }
    }

    pub fn to_flat(&self) -> Vec<f32> {
        vec!(self.position.x, self.position.y)
    }

    pub fn update(&mut self, dt: f32) {
        let new_pos = self.position + (self.velocity * dt) + (self.acceleration * (dt * dt * 0.5));
        let new_vel = self.velocity + (self.acceleration + self.new_acceleration) * (dt * 0.5);
        self.position = new_pos;
        self.velocity = new_vel;
        self.acceleration = self.new_acceleration;

        if(self.position.x > 1.0) {
            self.position.x = 0.0;
        }

        if(self.position.x < 0.0) {
            self.position.x = 1.0;
        }

        if(self.position.y > 1.0) {
            self.position.y = 0.0;
        }

        if(self.position.y < 0.0) {
            self.position.y = 1.0;
        }
    }
}