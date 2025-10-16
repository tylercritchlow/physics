use physics::{RigidBody, Vec3};

#[test]
fn test_rigid_body_creation() {
    let position = Vec3::new(1.0, 2.0, 3.0);
    let body = RigidBody::new(position);

    assert_eq!(body.position, Vec3::new(1.0, 2.0, 3.0));
    assert_eq!(body.velocity, Vec3::zero());
    assert_eq!(body.acceleration, Vec3::zero());
}

#[test]
fn test_rigid_body_update_with_velocity() {
    let mut body = RigidBody::new(Vec3::zero());
    body.velocity = Vec3::new(1.0, 0.0, 0.0);

    let dt = 1.0;
    body.update(dt);

    assert_eq!(body.position, Vec3::new(1.0, 0.0, 0.0));
}

#[test]
fn test_rigid_body_update_with_acceleration() {
    let mut body = RigidBody::new(Vec3::zero());
    body.acceleration = Vec3::new(10.0, 0.0, 0.0);

    let dt = 0.1;
    body.update(dt);

    assert_eq!(body.velocity.x, 1.0);
    assert_eq!(body.position.x, 0.1);
}

#[test]
fn test_rigid_body_multiple_updates() {
    let mut body = RigidBody::new(Vec3::zero());
    body.velocity = Vec3::new(1.0, 1.0, 1.0);

    let dt = 0.1;
    body.update(dt);
    body.update(dt);
    body.update(dt);

    assert!((body.position.x - 0.3).abs() < 0.0001);
    assert!((body.position.y - 0.3).abs() < 0.0001);
    assert!((body.position.z - 0.3).abs() < 0.0001);
}

#[test]
fn test_rigid_body_gravity_simulation() {
    let mut body = RigidBody::new(Vec3::new(0.0, 10.0, 0.0));
    body.acceleration = Vec3::new(0.0, -9.8, 0.0);

    let dt = 1.0;
    body.update(dt);

    assert_eq!(body.velocity.y, -9.8);
    assert_eq!(body.position.y, 10.0 - 9.8);
}

#[test]
fn test_rigid_body_timestep_independence() {
    let mut body1 = RigidBody::new(Vec3::zero());
    body1.velocity = Vec3::new(1.0, 0.0, 0.0);

    let mut body2 = RigidBody::new(Vec3::zero());
    body2.velocity = Vec3::new(1.0, 0.0, 0.0);

    body1.update(1.0);

    body2.update(0.5);
    body2.update(0.5);

    assert!((body1.position.x - body2.position.x).abs() < 0.0001);
}

#[test]
fn test_rigid_body_zero_timestep() {
    let mut body = RigidBody::new(Vec3::new(1.0, 2.0, 3.0));
    body.velocity = Vec3::new(5.0, 5.0, 5.0);

    let original_pos = body.position;
    body.update(0.0);

    assert_eq!(body.position, original_pos);
}
