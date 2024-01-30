extern crate nalgebra_glm as glm;

use crate::simulation::DOMAIN_MODE;

#[derive(Clone, Debug)]
pub struct Particle {
    pub position: glm::Vec2,
    pub velocity: glm::Vec2,
    acceleration: glm::Vec2,
    pub new_acceleration: glm::Vec2,
    domain_wrap: DOMAIN_MODE
}

impl Particle {
    pub fn new(position: glm::Vec2, velocity: glm::Vec2, domain_wrap: DOMAIN_MODE) -> Self {
        Self {
            position,
            velocity,
            acceleration: glm::vec2(0.0, 0.0),
            new_acceleration: glm::vec2(0.0, 0.0),
            domain_wrap
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
        self.new_acceleration = glm::vec2(0.0, 0.0);

        match self.domain_wrap {
            DOMAIN_MODE::WRAP => {
                if self.position.x > 1.0 {
                    self.position.x = 0.0;
                }

                if self.position.x < 0.0 {
                    self.position.x = 1.0;
                }

                if self.position.y > 1.0 {
                    self.position.y = 0.0;
                }

                if self.position.y < 0.0 {
                    self.position.y = 1.0;
                }
            },
            DOMAIN_MODE::WALL => {
                if self.position.x > 1.0 {
                  self.position.x = 1.0;
                  self.velocity.x = -self.velocity.x;
                }
                if self.position.x < 0.0 {
                  self.position.x = 0.0;
                  self.velocity.x = -self.velocity.x;
                }
                if self.position.y > 1.0 {
                  self.position.y = 1.0;
                  self.velocity.y = -self.velocity.y;
                }
                if self.position.y < 0.0 {
                  self.position.y = 0.0;
                  self.velocity.y = -self.velocity.y;
                }
            },
            _ => {}
        }
    }
}