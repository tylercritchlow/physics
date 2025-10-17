use physics::{aabb_vs_aabb, sphere_vs_aabb, sphere_vs_plane, sphere_vs_sphere, Vec3};

// Sphere vs Sphere Tests
#[test]
fn test_sphere_vs_sphere_no_collision() {
    let result = sphere_vs_sphere(
        Vec3::new(0.0, 0.0, 0.0),
        1.0,
        Vec3::new(5.0, 0.0, 0.0),
        1.0,
    );
    assert!(!result.is_colliding);
}

#[test]
fn test_sphere_vs_sphere_touching() {
    let result = sphere_vs_sphere(
        Vec3::new(0.0, 0.0, 0.0),
        1.0,
        Vec3::new(2.0, 0.0, 0.0),
        1.0,
    );
    assert!(result.is_colliding);
    assert!((result.penetration_depth - 0.0).abs() < 0.0001);
}

#[test]
fn test_sphere_vs_sphere_overlapping() {
    let result = sphere_vs_sphere(
        Vec3::new(0.0, 0.0, 0.0),
        1.0,
        Vec3::new(1.0, 0.0, 0.0),
        1.0,
    );
    assert!(result.is_colliding);
    assert!((result.penetration_depth - 1.0).abs() < 0.0001);
    assert!((result.normal.x - 1.0).abs() < 0.0001);
    assert_eq!(result.normal.y, 0.0);
    assert_eq!(result.normal.z, 0.0);
}

#[test]
fn test_sphere_vs_sphere_same_position() {
    let result = sphere_vs_sphere(Vec3::zero(), 1.0, Vec3::zero(), 1.0);
    assert!(result.is_colliding);
    assert!((result.penetration_depth - 2.0).abs() < 0.0001);
}

#[test]
fn test_sphere_vs_sphere_3d_overlap() {
    let result = sphere_vs_sphere(
        Vec3::new(0.0, 0.0, 0.0),
        1.0,
        Vec3::new(0.5, 0.5, 0.5),
        1.0,
    );
    assert!(result.is_colliding);
    let expected_distance = (0.5_f32 * 0.5 + 0.5 * 0.5 + 0.5 * 0.5).sqrt();
    let expected_penetration = 2.0 - expected_distance;
    assert!((result.penetration_depth - expected_penetration).abs() < 0.0001);
}

// Sphere vs Plane Tests
#[test]
fn test_sphere_vs_plane_no_collision() {
    let result = sphere_vs_plane(
        Vec3::new(0.0, 5.0, 0.0),
        1.0,
        Vec3::new(0.0, 1.0, 0.0),
        0.0,
    );
    assert!(!result.is_colliding);
}

#[test]
fn test_sphere_vs_plane_touching() {
    let result = sphere_vs_plane(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Vec3::new(0.0, 1.0, 0.0),
        0.0,
    );
    assert!(result.is_colliding);
    assert!((result.penetration_depth - 0.0).abs() < 0.0001);
}

#[test]
fn test_sphere_vs_plane_penetrating() {
    let result = sphere_vs_plane(
        Vec3::new(0.0, 0.5, 0.0),
        1.0,
        Vec3::new(0.0, 1.0, 0.0),
        0.0,
    );
    assert!(result.is_colliding);
    assert!((result.penetration_depth - 0.5).abs() < 0.0001);
    assert_eq!(result.normal, Vec3::new(0.0, 1.0, 0.0));
}

#[test]
fn test_sphere_vs_plane_below_plane() {
    let result = sphere_vs_plane(
        Vec3::new(0.0, -0.5, 0.0),
        1.0,
        Vec3::new(0.0, 1.0, 0.0),
        0.0,
    );
    assert!(result.is_colliding);
    assert!((result.penetration_depth - 1.5).abs() < 0.0001);
}

#[test]
fn test_sphere_vs_plane_angled() {
    let normal = Vec3::new(0.0, 1.0, 1.0).normalize();
    let result = sphere_vs_plane(Vec3::new(0.0, 0.5, 0.5), 1.0, normal, 0.0);
    assert!(result.is_colliding);
}

// AABB vs AABB Tests
#[test]
fn test_aabb_vs_aabb_no_collision() {
    let result = aabb_vs_aabb(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(5.0, 0.0, 0.0),
        Vec3::new(6.0, 1.0, 1.0),
    );
    assert!(!result.is_colliding);
}

#[test]
fn test_aabb_vs_aabb_touching() {
    let result = aabb_vs_aabb(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(2.0, 1.0, 1.0),
    );
    assert!(result.is_colliding);
    assert!((result.penetration_depth - 0.0).abs() < 0.0001);
}

#[test]
fn test_aabb_vs_aabb_overlapping_x() {
    let result = aabb_vs_aabb(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(2.0, 1.0, 1.0),
        Vec3::new(1.5, 0.0, 0.0),
        Vec3::new(3.5, 1.0, 1.0),
    );
    assert!(result.is_colliding);
    assert!((result.penetration_depth - 0.5).abs() < 0.0001);
}

#[test]
fn test_aabb_vs_aabb_overlapping_y() {
    let result = aabb_vs_aabb(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 2.0, 1.0),
        Vec3::new(0.0, 1.5, 0.0),
        Vec3::new(1.0, 3.5, 1.0),
    );
    assert!(result.is_colliding);
    assert!((result.penetration_depth - 0.5).abs() < 0.0001);
}

#[test]
fn test_aabb_vs_aabb_overlapping_z() {
    let result = aabb_vs_aabb(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 1.0, 2.0),
        Vec3::new(0.0, 0.0, 1.5),
        Vec3::new(1.0, 1.0, 3.5),
    );
    assert!(result.is_colliding);
    assert!((result.penetration_depth - 0.5).abs() < 0.0001);
}

#[test]
fn test_aabb_vs_aabb_fully_contained() {
    let result = aabb_vs_aabb(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(5.0, 5.0, 5.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(2.0, 2.0, 2.0),
    );
    assert!(result.is_colliding);
}

#[test]
fn test_aabb_vs_aabb_normal_direction() {
    let result = aabb_vs_aabb(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(0.5, 0.0, 0.0),
        Vec3::new(1.5, 1.0, 1.0),
    );
    assert!(result.is_colliding);
    // Normal should point in X direction (axis of least penetration)
    assert!((result.normal.x.abs() - 1.0).abs() < 0.0001);
}

// Sphere vs AABB Tests
#[test]
fn test_sphere_vs_aabb_no_collision() {
    let result = sphere_vs_aabb(
        Vec3::new(0.0, 0.0, 0.0),
        1.0,
        Vec3::new(5.0, 0.0, 0.0),
        Vec3::new(6.0, 1.0, 1.0),
    );
    assert!(!result.is_colliding);
}

#[test]
fn test_sphere_vs_aabb_touching_face() {
    let result = sphere_vs_aabb(
        Vec3::new(0.0, 0.0, 0.0),
        1.0,
        Vec3::new(1.0, -0.5, -0.5),
        Vec3::new(2.0, 0.5, 0.5),
    );
    assert!(result.is_colliding);
    assert!((result.penetration_depth - 0.0).abs() < 0.0001);
}

#[test]
fn test_sphere_vs_aabb_penetrating_face() {
    let result = sphere_vs_aabb(
        Vec3::new(0.0, 0.0, 0.0),
        1.0,
        Vec3::new(0.5, -0.5, -0.5),
        Vec3::new(1.5, 0.5, 0.5),
    );
    assert!(result.is_colliding);
    assert!((result.penetration_depth - 0.5).abs() < 0.0001);
    assert!((result.normal.x - (-1.0)).abs() < 0.0001);
}

#[test]
fn test_sphere_vs_aabb_touching_edge() {
    let result = sphere_vs_aabb(
        Vec3::new(0.0, 2.0, 0.0),
        1.0,
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 1.0, 1.0),
    );
    assert!(result.is_colliding);
}

#[test]
fn test_sphere_vs_aabb_touching_corner() {
    let sphere_pos = Vec3::new(2.0, 2.0, 2.0);
    let result = sphere_vs_aabb(
        sphere_pos,
        1.0,
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 1.0, 1.0),
    );

    let corner = Vec3::new(1.0, 1.0, 1.0);
    let distance = (sphere_pos - corner).magnitude();
    assert_eq!(result.is_colliding, distance < 1.0);
}

#[test]
fn test_sphere_vs_aabb_center_inside() {
    let result = sphere_vs_aabb(
        Vec3::new(0.5, 0.5, 0.5),
        1.0,
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 1.0, 1.0),
    );
    assert!(result.is_colliding);
    // When center is inside, normal should point toward nearest face
    assert!(
        result.normal.x.abs() == 1.0
            || result.normal.y.abs() == 1.0
            || result.normal.z.abs() == 1.0
    );
}

#[test]
fn test_sphere_vs_aabb_fully_inside() {
    let result = sphere_vs_aabb(
        Vec3::new(2.5, 2.5, 2.5),
        0.5,
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(5.0, 5.0, 5.0),
    );
    assert!(result.is_colliding);
}
