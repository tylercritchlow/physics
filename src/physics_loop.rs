use crate::body::RigidBody;
use crate::collision::{sphere_vs_sphere, CollisionShape};
use crate::vector::Vec3;

pub struct PhysicsWorld {
    pub bodies: Vec<RigidBody>,
    accumulator: f32,
    fixed_timestep: f32,
    pub gravity: Vec3,
    pub restitution: f32,
}

impl PhysicsWorld {
    pub fn new(fixed_timestep: f32) -> Self {
        Self {
            bodies: Vec::new(),
            accumulator: 0.0,
            fixed_timestep,
            gravity: Vec3::new(0.0, -9.8, 0.0),
            restitution: 0.5,
        }
    }

    pub fn with_gravity(fixed_timestep: f32, gravity: Vec3) -> Self {
        Self {
            bodies: Vec::new(),
            accumulator: 0.0,
            fixed_timestep,
            gravity,
            restitution: 0.5,
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
        // Apply gravity
        for body in &mut self.bodies {
            let gravity_force = self.gravity * body.mass;
            body.apply_force(gravity_force);
        }

        // Update positions
        for body in &mut self.bodies {
            body.update(self.fixed_timestep);
        }

        // Detect and resolve collisions
        self.resolve_collisions();
    }

    fn resolve_collisions(&mut self) {
        let body_count = self.bodies.len();

        for i in 0..body_count {
            for j in (i + 1)..body_count {
                // Get collision info based on shape types
                let collision_info = match (&self.bodies[i].shape, &self.bodies[j].shape) {
                    (
                        CollisionShape::Sphere { radius: r1 },
                        CollisionShape::Sphere { radius: r2 },
                    ) => sphere_vs_sphere(
                        self.bodies[i].position,
                        *r1,
                        self.bodies[j].position,
                        *r2,
                    ),
                    _ => continue, // Skip unsupported shape combinations for now
                };

                if collision_info.is_colliding {
                    // Get data we need before borrowing mutably
                    let pos_i = self.bodies[i].position;
                    let pos_j = self.bodies[j].position;
                    let vel_i = self.bodies[i].velocity;
                    let vel_j = self.bodies[j].velocity;
                    let mass_i = self.bodies[i].mass;
                    let mass_j = self.bodies[j].mass;

                    // Separate bodies
                    let separation = collision_info.normal * collision_info.penetration_depth;
                    let total_mass = mass_i + mass_j;
                    self.bodies[i].position = pos_i - separation * (mass_j / total_mass);
                    self.bodies[j].position = pos_j + separation * (mass_i / total_mass);

                    // Calculate relative velocity along collision normal
                    let relative_velocity = vel_i - vel_j;
                    let velocity_along_normal = relative_velocity.dot(&collision_info.normal);

                    // If bodies are separating (velocity_along_normal < 0), skip
                    if velocity_along_normal < 0.0 {
                        continue;
                    }

                    // Calculate impulse
                    let e = self.restitution;
                    let impulse_magnitude = -(1.0 + e) * velocity_along_normal / (1.0 / mass_i + 1.0 / mass_j);
                    let impulse = collision_info.normal * impulse_magnitude;

                    // Apply impulse
                    self.bodies[i].velocity = vel_i + impulse * (1.0 / mass_i);
                    self.bodies[j].velocity = vel_j - impulse * (1.0 / mass_j);
                }
            }
        }
    }
}
