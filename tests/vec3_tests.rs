use physics::Vec3;

#[test]
fn test_vec3_creation() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(v.x, 1.0);
    assert_eq!(v.y, 2.0);
    assert_eq!(v.z, 3.0);
}

#[test]
fn test_vec3_zero() {
    let v = Vec3::zero();
    assert_eq!(v.x, 0.0);
    assert_eq!(v.y, 0.0);
    assert_eq!(v.z, 0.0);
}

#[test]
fn test_vec3_addition() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(4.0, 5.0, 6.0);
    let result = v1 + v2;
    assert_eq!(result.x, 5.0);
    assert_eq!(result.y, 7.0);
    assert_eq!(result.z, 9.0);
}

#[test]
fn test_vec3_add_assign() {
    let mut v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(4.0, 5.0, 6.0);
    v1 += v2;
    assert_eq!(v1.x, 5.0);
    assert_eq!(v1.y, 7.0);
    assert_eq!(v1.z, 9.0);
}

#[test]
fn test_vec3_scalar_multiplication() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    let result = v * 2.0;
    assert_eq!(result.x, 2.0);
    assert_eq!(result.y, 4.0);
    assert_eq!(result.z, 6.0);
}

#[test]
fn test_vec3_scalar_multiplication_zero() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    let result = v * 0.0;
    assert_eq!(result.x, 0.0);
    assert_eq!(result.y, 0.0);
    assert_eq!(result.z, 0.0);
}

#[test]
fn test_vec3_negative_scalar() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    let result = v * -1.0;
    assert_eq!(result.x, -1.0);
    assert_eq!(result.y, -2.0);
    assert_eq!(result.z, -3.0);
}

#[test]
fn test_vec3_copy_trait() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = v1;
    assert_eq!(v1.x, v2.x);
    assert_eq!(v1.y, v2.y);
    assert_eq!(v1.z, v2.z);
}

#[test]
fn test_vec3_magnitude() {
    let v = Vec3::new(3.0, 4.0, 0.0);
    assert_eq!(v.magnitude(), 5.0);
}

#[test]
fn test_vec3_magnitude_3d() {
    let v = Vec3::new(1.0, 2.0, 2.0);
    assert_eq!(v.magnitude(), 3.0);
}

#[test]
fn test_vec3_magnitude_zero() {
    let v = Vec3::zero();
    assert_eq!(v.magnitude(), 0.0);
}

#[test]
fn test_vec3_magnitude_squared() {
    let v = Vec3::new(3.0, 4.0, 0.0);
    assert_eq!(v.magnitude_squared(), 25.0);
}

#[test]
fn test_vec3_magnitude_squared_3d() {
    let v = Vec3::new(1.0, 2.0, 2.0);
    assert_eq!(v.magnitude_squared(), 9.0);
}

#[test]
fn test_vec3_normalize() {
    let v = Vec3::new(3.0, 4.0, 0.0);
    let normalized = v.normalize();
    assert_eq!(normalized.x, 0.6);
    assert_eq!(normalized.y, 0.8);
    assert_eq!(normalized.z, 0.0);
    // Check that magnitude is 1
    let mag = normalized.magnitude();
    assert!((mag - 1.0).abs() < 0.0001);
}

#[test]
fn test_vec3_normalize_3d() {
    let v = Vec3::new(1.0, 2.0, 2.0);
    let normalized = v.normalize();
    assert!((normalized.x - 1.0/3.0).abs() < 0.0001);
    assert!((normalized.y - 2.0/3.0).abs() < 0.0001);
    assert!((normalized.z - 2.0/3.0).abs() < 0.0001);
    // Check that magnitude is 1
    let mag = normalized.magnitude();
    assert!((mag - 1.0).abs() < 0.0001);
}

#[test]
fn test_vec3_normalize_zero() {
    let v = Vec3::zero();
    let normalized = v.normalize();
    assert_eq!(normalized.x, 0.0);
    assert_eq!(normalized.y, 0.0);
    assert_eq!(normalized.z, 0.0);
}

#[test]
fn test_vec3_normalized_alias() {
    let v = Vec3::new(3.0, 4.0, 0.0);
    let n1 = v.normalize();
    let n2 = v.normalized();
    assert_eq!(n1, n2);
}

#[test]
fn test_vec3_subtraction() {
    let v1 = Vec3::new(5.0, 7.0, 9.0);
    let v2 = Vec3::new(1.0, 2.0, 3.0);
    let result = v1 - v2;
    assert_eq!(result.x, 4.0);
    assert_eq!(result.y, 5.0);
    assert_eq!(result.z, 6.0);
}

#[test]
fn test_vec3_sub_assign() {
    let mut v1 = Vec3::new(5.0, 7.0, 9.0);
    let v2 = Vec3::new(1.0, 2.0, 3.0);
    v1 -= v2;
    assert_eq!(v1.x, 4.0);
    assert_eq!(v1.y, 5.0);
    assert_eq!(v1.z, 6.0);
}

// Dot Product Tests
#[test]
fn test_vec3_dot_product() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(4.0, 5.0, 6.0);
    let result = v1.dot(&v2);
    assert_eq!(result, 32.0); // 1*4 + 2*5 + 3*6 = 4 + 10 + 18 = 32
}

#[test]
fn test_vec3_dot_product_perpendicular() {
    let v1 = Vec3::new(1.0, 0.0, 0.0);
    let v2 = Vec3::new(0.0, 1.0, 0.0);
    let result = v1.dot(&v2);
    assert_eq!(result, 0.0); // Perpendicular vectors have dot product of 0
}

#[test]
fn test_vec3_dot_product_parallel() {
    let v1 = Vec3::new(2.0, 0.0, 0.0);
    let v2 = Vec3::new(3.0, 0.0, 0.0);
    let result = v1.dot(&v2);
    assert_eq!(result, 6.0); // Parallel vectors
}

#[test]
fn test_vec3_dot_product_opposite() {
    let v1 = Vec3::new(1.0, 0.0, 0.0);
    let v2 = Vec3::new(-1.0, 0.0, 0.0);
    let result = v1.dot(&v2);
    assert_eq!(result, -1.0); // Opposite direction gives negative dot product
}

#[test]
fn test_vec3_dot_product_with_zero() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::zero();
    let result = v1.dot(&v2);
    assert_eq!(result, 0.0);
}

#[test]
fn test_vec3_dot_product_angle_calculation() {
    // Two unit vectors at 60 degrees (cos(60°) = 0.5)
    let v1 = Vec3::new(1.0, 0.0, 0.0);
    let v2 = Vec3::new(0.5, 0.866025, 0.0); // cos(60°), sin(60°), 0
    let result = v1.dot(&v2);
    assert!((result - 0.5).abs() < 0.0001);
}

// Cross Product Tests
#[test]
fn test_vec3_cross_product_basis_vectors() {
    let x_axis = Vec3::new(1.0, 0.0, 0.0);
    let y_axis = Vec3::new(0.0, 1.0, 0.0);
    let result = x_axis.cross(&y_axis);
    assert_eq!(result.x, 0.0);
    assert_eq!(result.y, 0.0);
    assert_eq!(result.z, 1.0); // x × y = z
}

#[test]
fn test_vec3_cross_product_reverse_order() {
    let x_axis = Vec3::new(1.0, 0.0, 0.0);
    let y_axis = Vec3::new(0.0, 1.0, 0.0);
    let result = y_axis.cross(&x_axis);
    assert_eq!(result.x, 0.0);
    assert_eq!(result.y, 0.0);
    assert_eq!(result.z, -1.0); // y × x = -z (anti-commutative)
}

#[test]
fn test_vec3_cross_product_parallel_vectors() {
    let v1 = Vec3::new(2.0, 0.0, 0.0);
    let v2 = Vec3::new(4.0, 0.0, 0.0);
    let result = v1.cross(&v2);
    assert_eq!(result.x, 0.0);
    assert_eq!(result.y, 0.0);
    assert_eq!(result.z, 0.0); // Parallel vectors have cross product of zero
}

#[test]
fn test_vec3_cross_product_general() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(4.0, 5.0, 6.0);
    let result = v1.cross(&v2);
    // Manual calculation:
    // x: 2*6 - 3*5 = 12 - 15 = -3
    // y: 3*4 - 1*6 = 12 - 6 = 6
    // z: 1*5 - 2*4 = 5 - 8 = -3
    assert_eq!(result.x, -3.0);
    assert_eq!(result.y, 6.0);
    assert_eq!(result.z, -3.0);
}

#[test]
fn test_vec3_cross_product_perpendicular() {
    let v1 = Vec3::new(1.0, 0.0, 0.0);
    let v2 = Vec3::new(0.0, 1.0, 0.0);
    let result = v1.cross(&v2);
    // Cross product of perpendicular vectors should be perpendicular to both
    assert_eq!(v1.dot(&result), 0.0);
    assert_eq!(v2.dot(&result), 0.0);
}

#[test]
fn test_vec3_cross_product_magnitude() {
    // For unit vectors at 90 degrees, |a × b| = |a| * |b| * sin(90°) = 1
    let v1 = Vec3::new(1.0, 0.0, 0.0);
    let v2 = Vec3::new(0.0, 1.0, 0.0);
    let result = v1.cross(&v2);
    assert_eq!(result.magnitude(), 1.0);
}

#[test]
fn test_vec3_cross_product_with_zero() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::zero();
    let result = v1.cross(&v2);
    assert_eq!(result, Vec3::zero());
}
