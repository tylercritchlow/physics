use crate::vector::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CollisionShape {
    Sphere { radius: f32 },
    Plane { normal: Vec3, distance: f32 },
    AABB { min: Vec3, max: Vec3 },
}

#[derive(Debug, Clone, Copy)]
pub struct CollisionInfo {
    pub is_colliding: bool,
    pub normal: Vec3,
    pub penetration_depth: f32,
    pub contact_point: Vec3,
}

impl CollisionInfo {
    pub fn none() -> Self {
        Self {
            is_colliding: false,
            normal: Vec3::zero(),
            penetration_depth: 0.0,
            contact_point: Vec3::zero(),
        }
    }

    pub fn new(normal: Vec3, penetration_depth: f32, contact_point: Vec3) -> Self {
        Self {
            is_colliding: true,
            normal,
            penetration_depth,
            contact_point,
        }
    }
}

// Sphere vs Sphere collision
pub fn sphere_vs_sphere(
    pos_a: Vec3,
    radius_a: f32,
    pos_b: Vec3,
    radius_b: f32,
) -> CollisionInfo {
    let delta = pos_b - pos_a;
    let distance = delta.magnitude();
    let sum_radii = radius_a + radius_b;

    if distance <= sum_radii {
        let penetration = sum_radii - distance;
        let normal = if distance > 0.0 {
            delta.normalize()
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        
        let contact_point = pos_a + normal * radius_a;

        CollisionInfo::new(normal, penetration, contact_point)
    } else {
        CollisionInfo::none()
    }
}

// Sphere vs Plane collision - TODO: Needs implementation following research
pub fn sphere_vs_plane(
    sphere_pos: Vec3,
    sphere_radius: f32,
    plane_normal: Vec3,
    plane_distance: f32,
) -> CollisionInfo {
    // Placeholder - returns no collision for now
    CollisionInfo::none()
}

// AABB vs AABB collision - TODO: Needs implementation following research
pub fn aabb_vs_aabb(min_a: Vec3, max_a: Vec3, min_b: Vec3, max_b: Vec3) -> CollisionInfo {
    // Placeholder - returns no collision for now
    CollisionInfo::none()
}

// Sphere vs AABB collision - TODO: Needs complete rewrite following research
pub fn sphere_vs_aabb(
    sphere_pos: Vec3,
    sphere_radius: f32,
    aabb_min: Vec3,
    aabb_max: Vec3,
) -> CollisionInfo {
    // Placeholder - returns no collision for now
    CollisionInfo::none()
}
