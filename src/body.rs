use crate::vector::Vec3;

pub struct RigidBody {
    pub position: Vec3,
    pub velocity: Vec3,
    pub acceleration: Vec3,
}

impl RigidBody {
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            velocity: Vec3::zero(),
            acceleration: Vec3::zero(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.velocity += self.acceleration * dt;
        self.position += self.velocity * dt;
    }
}
