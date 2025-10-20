use crate::collision::CollisionShape;
use crate::vector::Vec3;

pub struct StaticBody {
    pub position: Vec3,
    pub shape: CollisionShape,
    pub friction: f32,
}

impl StaticBody {
    pub fn new(position: Vec3, shape: CollisionShape) -> Self {
        Self { position, shape, friction: 0.25 }
    }
}

pub struct RigidBody {
    pub position: Vec3,
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub mass: f32,
    pub shape: CollisionShape,
    pub friction: f32,
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
            friction: 0.25,
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
            friction: 0.25,
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
            friction: 0.25,
            force_accumulator: Vec3::zero(),
        }
    }

    pub fn apply_force(&mut self, force: Vec3) {
        self.force_accumulator += force;
    }

    pub fn clear_forces(&mut self) {
        self.force_accumulator = Vec3::zero();
    }

    // New method: Integrates velocity based on acceleration
    pub fn integrate_velocity(&mut self, dt: f32) {
        if self.mass > 0.0 {
            self.acceleration = self.force_accumulator * (1.0 / self.mass);
        }
        self.velocity += self.acceleration * dt;
    }

    // New method: Integrates position based on velocity
    pub fn integrate_position(&mut self, dt: f32) {
        self.position += self.velocity * dt;
    }

    pub fn update(&mut self, dt: f32) {
        self.integrate_velocity(dt);
        self.integrate_position(dt);
        self.clear_forces();
    }
}

pub enum Body {
    Rigid(RigidBody),
    Static(StaticBody),
}

impl Body {
    pub fn as_rigid_body_mut(&mut self) -> Option<&mut RigidBody> {
        match self {
            Body::Rigid(body) => Some(body),
            Body::Static(_) => None,
        }
    }

    pub fn as_rigid_body(&self) -> Option<&RigidBody> {
        match self {
            Body::Rigid(body) => Some(body),
            Body::Static(_) => None,
        }
    }

    pub fn as_static_body_mut(&mut self) -> Option<&mut StaticBody> {
        match self {
            Body::Static(body) => Some(body),
            Body::Rigid(_) => None,
        }
    }

    pub fn as_static_body(&self) -> Option<&StaticBody> {
        match self {
            Body::Static(body) => Some(body),
            Body::Rigid(_) => None,
        }
    }

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

    pub fn friction(&self) -> f32 {
        match self {
            Body::Rigid(body) => body.friction,
            Body::Static(body) => body.friction,
        }
    }
}