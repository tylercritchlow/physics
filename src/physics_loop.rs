use crate::body::RigidBody;
use crate::vector::Vec3;

pub struct PhysicsWorld {
    pub bodies: Vec<RigidBody>,
    accumulator: f32,
    fixed_timestep: f32,
    pub gravity: Vec3,
}

impl PhysicsWorld {
    pub fn new(fixed_timestep: f32) -> Self {
        Self {
            bodies: Vec::new(),
            accumulator: 0.0,
            fixed_timestep,
            gravity: Vec3::new(0.0, -9.8, 0.0),
        }
    }

    pub fn with_gravity(fixed_timestep: f32, gravity: Vec3) -> Self {
        Self {
            bodies: Vec::new(),
            accumulator: 0.0,
            fixed_timestep,
            gravity,
        }
    }

    pub fn add_body(&mut self, body: RigidBody) {
        self.bodies.push(body);
    }

    pub fn update(&mut self, delta_time: f32) {
        self.accumulator += delta_time;

        while self.accumulator >= self.fixed_timestep - f32::EPSILON {
            self.fixed_update();
            self.accumulator -= self.fixed_timestep;
        }
    }

    fn fixed_update(&mut self) {
        for body in &mut self.bodies {
            let gravity_force = self.gravity * body.mass;
            body.apply_force(gravity_force);
        }

        for body in &mut self.bodies {
            body.update(self.fixed_timestep);
        }
    }
}
