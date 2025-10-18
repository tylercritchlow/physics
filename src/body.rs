use crate::collision::CollisionShape;
use crate::vector::Vec3;

// StaticBody: Has position and shape for collision, but no movement properties.
pub struct StaticBody {
    pub position: Vec3,
    pub shape: CollisionShape,
}

impl StaticBody {
    pub fn new(position: Vec3, shape: CollisionShape) -> Self {
        Self { position, shape }
    }
}

// RigidBody: Dynamic body with mass, velocity, and acceleration.
pub struct RigidBody {
    pub position: Vec3,
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub mass: f32,
    pub shape: CollisionShape,
    force_accumulator: Vec3,
}

impl RigidBody {
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            velocity: Vec3::zero(),
            acceleration: Vec3::zero(),
            mass: 1.0,
            shape: CollisionShape::Sphere { radius: 0.5 },
            force_accumulator: Vec3::zero(),
        }
    }

    pub fn with_mass(position: Vec3, mass: f32) -> Self {
        Self {
            position,
            velocity: Vec3::zero(),
            acceleration: Vec3::zero(),
            mass,
            shape: CollisionShape::Sphere { radius: 0.5 },
            force_accumulator: Vec3::zero(),
        }
    }

    pub fn with_shape(position: Vec3, mass: f32, shape: CollisionShape) -> Self {
        Self {
            position,
            velocity: Vec3::zero(),
            acceleration: Vec3::zero(),
            mass,
            shape,
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

// Body enum: Represents either a dynamic RigidBody or a static StaticBody.
pub enum Body {
    Rigid(RigidBody),
    Static(StaticBody),
}

impl Body {
    // Helper to get mutable RigidBody if it's a Rigid variant
    pub fn as_rigid_body_mut(&mut self) -> Option<&mut RigidBody> {
        match self {
            Body::Rigid(body) => Some(body),
            Body::Static(_) => None,
        }
    }

    // Helper to get immutable RigidBody if it's a Rigid variant
    pub fn as_rigid_body(&self) -> Option<&RigidBody> {
        match self {
            Body::Rigid(body) => Some(body),
            Body::Static(_) => None,
        }
    }

    // Helper to get mutable StaticBody if it's a Static variant
    pub fn as_static_body_mut(&mut self) -> Option<&mut StaticBody> {
        match self {
            Body::Static(body) => Some(body),
            Body::Rigid(_) => None,
        }
    }

    // Helper to get immutable StaticBody if it's a Static variant
    pub fn as_static_body(&self) -> Option<&StaticBody> {
        match self {
            Body::Static(body) => Some(body),
            Body::Rigid(_) => None,
        }
    }

    // Common accessors for position and shape
    pub fn position(&self) -> &Vec3 {
        match self {
            Body::Rigid(body) => &body.position,
            Body::Static(body) => &body.position,
        }
    }

    pub fn shape(&self) -> &CollisionShape {
        match self {
            Body::Rigid(body) => &body.shape,
            Body::Static(body) => &body.shape,
        }
    }

    // Update only if it's a RigidBody
    pub fn update(&mut self, dt: f32) {
        if let Body::Rigid(body) = self {
            body.update(dt);
        }
    }
}