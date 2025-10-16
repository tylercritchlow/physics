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
