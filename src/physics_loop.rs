use crate::body::RigidBody;

pub struct PhysicsWorld {
    pub bodies: Vec<RigidBody>,
    accumulator: f32,
    fixed_timestep: f32,
}

impl PhysicsWorld {
    pub fn new(fixed_timestep: f32) -> Self {
        Self {
            bodies: Vec::new(),
            accumulator: 0.0,
            fixed_timestep,
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
            body.update(self.fixed_timestep);
        }
    }
}
