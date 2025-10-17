use crate::vector::Vec3;

pub struct RigidBody {
    pub position: Vec3,
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub mass: f32,
    force_accumulator: Vec3,
}

impl RigidBody {
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            velocity: Vec3::zero(),
            acceleration: Vec3::zero(),
            mass: 1.0,
            force_accumulator: Vec3::zero(),
        }
    }

    pub fn with_mass(position: Vec3, mass: f32) -> Self {
        Self {
            position,
            velocity: Vec3::zero(),
            acceleration: Vec3::zero(),
            mass,
            force_accumulator: Vec3::zero(),
        }
    }

    pub fn apply_force(&mut self, force: Vec3) {
        self.force_accumulator += force;
    }

    pub fn clear_forces(&mut self) {
        self.force_accumulator = Vec3::zero();
    }

    pub fn update(&mut self, dt: f32) {
        if self.mass > 0.0 {
            self.acceleration = self.force_accumulator * (1.0 / self.mass);
        }

        self.velocity += self.acceleration * dt;
        self.position += self.velocity * dt;

        self.clear_forces();
    }
}
