use crate::body::Body;
use crate::collision::{sphere_vs_plane, CollisionInfo, CollisionShape, sphere_vs_sphere, sphere_vs_aabb, aabb_vs_aabb};
use crate::vector::Vec3;

pub struct PhysicsWorld {
    pub bodies: Vec<Body>,
    accumulator: f32,
    fixed_timestep: f32,
    pub gravity: Vec3,
    pub restitution: f32,
    pub friction: f32,
}

impl PhysicsWorld {
    pub fn new(fixed_timestep: f32) -> Self {
        Self {
            bodies: Vec::new(),
            accumulator: 0.0,
            fixed_timestep,
            gravity: Vec3::new(0.0, -9.8, 0.0),
            restitution: 0.5,
            friction: 0.25,
        }
    }

    pub fn with_gravity(fixed_timestep: f32, gravity: Vec3) -> Self {
        Self {
            bodies: Vec::new(),
            accumulator: 0.0,
            fixed_timestep,
            gravity,
            restitution: 0.5,
            friction: 0.25,
        }
    }

    // Modified add_body to accept Body enum
    pub fn add_body(&mut self, body: Body) {
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
        // Apply gravity and update positions only for RigidBody instances
        for body in &mut self.bodies {
            if let Body::Rigid(rigid_body) = body {
                let gravity_force = self.gravity * rigid_body.mass;
                rigid_body.apply_force(gravity_force);
                rigid_body.update(self.fixed_timestep);
            }
        }

        // Detect and resolve collisions
        self.resolve_collisions();
    }

    fn resolve_collisions(&mut self) {
        let body_count = self.bodies.len();

        // Collect collision data first
        let mut collisions_to_resolve: Vec<(usize, usize, CollisionInfo)> = Vec::new();

        for i in 0..body_count {
            for j in (i + 1)..body_count {
                let body_i = &self.bodies[i];
                let body_j = &self.bodies[j];

                let collision_info = match (body_i.shape(), body_j.shape()) {
                    (
                        CollisionShape::Sphere { radius: r1 },
                        CollisionShape::Sphere { radius: r2 },
                    ) => sphere_vs_sphere(*body_i.position(), *r1, *body_j.position(), *r2),
                    (
                        CollisionShape::Sphere { radius },
                        CollisionShape::Plane { normal, distance },
                    ) => sphere_vs_plane(*body_i.position(), *radius, *normal, *distance),
                    (
                        CollisionShape::Plane { normal, distance },
                        CollisionShape::Sphere { radius },
                    ) => {
                        let mut info =
                            sphere_vs_plane(*body_j.position(), *radius, *normal, *distance);
                        if info.is_colliding {
                            info.normal = -info.normal;
                        }
                        info
                    }
                    (
                        CollisionShape::Sphere { radius },
                        CollisionShape::AABB { min, max },
                    ) => sphere_vs_aabb(*body_i.position(), *radius, *min, *max),
                    (
                        CollisionShape::AABB { min, max },
                        CollisionShape::Sphere { radius },
                    ) => {
                        let mut info = sphere_vs_aabb(*body_j.position(), *radius, *min, *max);
                        if info.is_colliding {
                            info.normal = -info.normal;
                        }
                        info
                    }
                    (
                        CollisionShape::AABB { min: min1, max: max1 },
                        CollisionShape::AABB { min: min2, max: max2 },
                    ) => aabb_vs_aabb(*min1, *max1, *min2, *max2),
                    _ => continue,
                };

                if collision_info.is_colliding {
                    // Only resolve collision if at least one body is Rigid
                    if body_i.as_rigid_body().is_some() || body_j.as_rigid_body().is_some() {
                        collisions_to_resolve.push((i, j, collision_info));
                    }
                }
            }
        }

        // Apply resolutions
        for (i, j, collision_info) in collisions_to_resolve {
            // Get mutable references to the bodies using split_at_mut
            let (body1, body2) = if i < j {
                let (left, right) = self.bodies.split_at_mut(j);
                (&mut left[i], &mut right[0])
            } else {
                let (left, right) = self.bodies.split_at_mut(i);
                (&mut right[0], &mut left[j])
            };

            // Extract all necessary immutable data *before* getting mutable RigidBody options
            let pos_i = *body1.position();
            let pos_j = *body2.position();

            let vel_i_initial = body1.as_rigid_body().map_or(Vec3::zero(), |b| b.velocity);
            let vel_j_initial = body2.as_rigid_body().map_or(Vec3::zero(), |b| b.velocity);

            let mass_i = body1.as_rigid_body().map_or(f32::INFINITY, |b| b.mass);
            let mass_j = body2.as_rigid_body().map_or(f32::INFINITY, |b| b.mass);

            let friction_i = body1.friction();
            let friction_j = body2.friction();

            // Now get mutable RigidBody options
            let mut rigid_body_i_option = body1.as_rigid_body_mut();
            let mut rigid_body_j_option = body2.as_rigid_body_mut();

            // If both are static, no resolution needed
            if rigid_body_i_option.is_none() && rigid_body_j_option.is_none() {
                continue;
            }

            // Separate bodies
            let separation = collision_info.normal * collision_info.penetration_depth;

            if let Some(rb_i) = rigid_body_i_option.as_mut() {
                let move_amount = if mass_j.is_infinite() {
                    separation
                } else {
                    separation * (mass_j / (mass_i + mass_j))
                };
                rb_i.position = pos_i - move_amount;
            }
            if let Some(rb_j) = rigid_body_j_option.as_mut() {
                let move_amount = if mass_i.is_infinite() {
                    separation
                } else {
                    separation * (mass_i / (mass_i + mass_j))
                };
                rb_j.position = pos_j + move_amount;
            }

            // Calculate relative velocity along collision normal
            let relative_velocity = vel_i_initial - vel_j_initial;
            let velocity_along_normal = relative_velocity.dot(&collision_info.normal);

            // If bodies are separating (velocity_along_normal < 0), skip impulse resolution.
            if velocity_along_normal < 0.0 {
                continue;
            }

            // Calculate normal impulse (with Baumgarte stabilization for penetration)
            let e = self.restitution;
            const PENETRATION_BIAS: f32 = 0.2;
            let impulse_magnitude_normal = (-(1.0 + e) * velocity_along_normal
                - PENETRATION_BIAS * collision_info.penetration_depth / self.fixed_timestep)
                / (1.0 / mass_i + 1.0 / mass_j);
            let normal_impulse = collision_info.normal * impulse_magnitude_normal;

            // Apply normal impulse
            if let Some(rb_i) = rigid_body_i_option.as_mut() {
                rb_i.velocity = vel_i_initial + normal_impulse * (1.0 / mass_i);
            }
            if let Some(rb_j) = rigid_body_j_option.as_mut() {
                rb_j.velocity = vel_j_initial - normal_impulse * (1.0 / mass_j);
            }

            // --- Friction Calculation ---
            let combined_friction = (friction_i + friction_j) * 0.5; // Average friction

            // Recalculate relative velocity after normal impulse
            let current_vel_i = rigid_body_i_option
                .as_ref()
                .map_or(vel_i_initial, |b| b.velocity);
            let current_vel_j = rigid_body_j_option
                .as_ref()
                .map_or(vel_j_initial, |b| b.velocity);
            let relative_velocity_after_normal = current_vel_i - current_vel_j;

            // Calculate tangential velocity
            let tangent_direction = relative_velocity_after_normal
                - collision_info.normal
                    * relative_velocity_after_normal.dot(&collision_info.normal);
            let tangent_magnitude = tangent_direction.magnitude();

            if tangent_magnitude > f32::EPSILON {
                // Avoid division by zero
                let unit_tangent = tangent_direction.normalize();

                // Calculate tangential impulse magnitude
                let impulse_magnitude_tangent = -relative_velocity_after_normal.dot(&unit_tangent)
                    / (1.0 / mass_i + 1.0 / mass_j);

                // Apply Coulomb's friction model
                let friction_impulse_magnitude = impulse_magnitude_tangent.clamp(
                    -impulse_magnitude_normal.abs() * combined_friction,
                    impulse_magnitude_normal.abs() * combined_friction,
                );

                let friction_impulse = unit_tangent * friction_impulse_magnitude;

                // Apply friction impulse
                if let Some(rb_i) = rigid_body_i_option.as_mut() {
                    rb_i.velocity += friction_impulse * (1.0 / mass_i);
                }
                if let Some(rb_j) = rigid_body_j_option.as_mut() {
                    rb_j.velocity -= friction_impulse * (1.0 / mass_j);
                }
            }
        }
    }
}
