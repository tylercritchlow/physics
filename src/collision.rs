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
}

impl CollisionInfo {
    pub fn none() -> Self {
        Self {
            is_colliding: false,
            normal: Vec3::zero(),
            penetration_depth: 0.0,
        }
    }

    pub fn new(normal: Vec3, penetration_depth: f32) -> Self {
        Self {
            is_colliding: true,
            normal,
            penetration_depth,
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
            Vec3::new(1.0, 0.0, 0.0) // Default normal if spheres are at same position
        };

        CollisionInfo::new(normal, penetration)
    } else {
        CollisionInfo::none()
    }
}

// Sphere vs Plane collision
pub fn sphere_vs_plane(
    sphere_pos: Vec3,
    sphere_radius: f32,
    plane_normal: Vec3,
    plane_distance: f32,
) -> CollisionInfo {
    let distance_from_plane = sphere_pos.dot(&plane_normal) - plane_distance;

    if distance_from_plane <= sphere_radius {
        let penetration = sphere_radius - distance_from_plane;
        CollisionInfo::new(plane_normal, penetration)
    } else {
        CollisionInfo::none()
    }
}

// AABB vs AABB collision
pub fn aabb_vs_aabb(min_a: Vec3, max_a: Vec3, min_b: Vec3, max_b: Vec3) -> CollisionInfo {
    // Check for overlap on all axes
    if max_a.x < min_b.x || min_a.x > max_b.x {
        return CollisionInfo::none();
    }
    if max_a.y < min_b.y || min_a.y > max_b.y {
        return CollisionInfo::none();
    }
    if max_a.z < min_b.z || min_a.z > max_b.z {
        return CollisionInfo::none();
    }

    // Calculate overlaps on each axis
    let overlap_x = (max_a.x - min_b.x).min(max_b.x - min_a.x);
    let overlap_y = (max_a.y - min_b.y).min(max_b.y - min_a.y);
    let overlap_z = (max_a.z - min_b.z).min(max_b.z - min_a.z);

    // Find the axis of least penetration
    let (normal, penetration) = if overlap_x < overlap_y && overlap_x < overlap_z {
        let n = if (max_a.x + min_a.x) < (max_b.x + min_b.x) {
            Vec3::new(-1.0, 0.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        (n, overlap_x)
    } else if overlap_y < overlap_z {
        let n = if (max_a.y + min_a.y) < (max_b.y + min_b.y) {
            Vec3::new(0.0, -1.0, 0.0)
        } else {
            Vec3::new(0.0, 1.0, 0.0)
        };
        (n, overlap_y)
    } else {
        let n = if (max_a.z + min_a.z) < (max_b.z + min_b.z) {
            Vec3::new(0.0, 0.0, -1.0)
        } else {
            Vec3::new(0.0, 0.0, 1.0)
        };
        (n, overlap_z)
    };

    CollisionInfo::new(normal, penetration)
}

// Sphere vs AABB collision
pub fn sphere_vs_aabb(
    sphere_pos: Vec3,
    sphere_radius: f32,
    aabb_min: Vec3,
    aabb_max: Vec3,
) -> CollisionInfo {
    // Find the closest point on the AABB to the sphere center
    let closest_x = sphere_pos.x.max(aabb_min.x).min(aabb_max.x);
    let closest_y = sphere_pos.y.max(aabb_min.y).min(aabb_max.y);
    let closest_z = sphere_pos.z.max(aabb_min.z).min(aabb_max.z);
    let closest_point = Vec3::new(closest_x, closest_y, closest_z);

    let delta = sphere_pos - closest_point;
    let distance_squared = delta.magnitude_squared();

    if distance_squared <= sphere_radius * sphere_radius {
        let distance = distance_squared.sqrt();
        let penetration = sphere_radius - distance;

        let normal = if distance > 0.0 {
            delta.normalize()
        } else {
            // Sphere center is inside AABB, use closest axis
            let dx = (sphere_pos.x - aabb_min.x).min(aabb_max.x - sphere_pos.x);
            let dy = (sphere_pos.y - aabb_min.y).min(aabb_max.y - sphere_pos.y);
            let dz = (sphere_pos.z - aabb_min.z).min(aabb_max.z - sphere_pos.z);

            if dx < dy && dx < dz {
                if sphere_pos.x < (aabb_min.x + aabb_max.x) * 0.5 {
                    Vec3::new(-1.0, 0.0, 0.0)
                } else {
                    Vec3::new(1.0, 0.0, 0.0)
                }
            } else if dy < dz {
                if sphere_pos.y < (aabb_min.y + aabb_max.y) * 0.5 {
                    Vec3::new(0.0, -1.0, 0.0)
                } else {
                    Vec3::new(0.0, 1.0, 0.0)
                }
            } else {
                if sphere_pos.z < (aabb_min.z + aabb_max.z) * 0.5 {
                    Vec3::new(0.0, 0.0, -1.0)
                } else {
                    Vec3::new(0.0, 0.0, 1.0)
                }
            }
        };

        CollisionInfo::new(normal, penetration)
    } else {
        CollisionInfo::none()
    }
}
