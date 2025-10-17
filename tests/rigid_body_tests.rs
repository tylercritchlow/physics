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

    // Apply force to create acceleration: F = ma, F = 1 * 10 = 10
    body.apply_force(Vec3::new(10.0, 0.0, 0.0));

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

    // Apply gravity force: F = ma, F = 1 * -9.8 = -9.8
    body.apply_force(Vec3::new(0.0, -9.8, 0.0));

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

// Force and Integration Tests
#[test]
fn test_rigid_body_default_mass() {
    let body = RigidBody::new(Vec3::zero());
    assert_eq!(body.mass, 1.0);
}

#[test]
fn test_rigid_body_with_mass() {
    let body = RigidBody::with_mass(Vec3::zero(), 5.0);
    assert_eq!(body.mass, 5.0);
    assert_eq!(body.position, Vec3::zero());
    assert_eq!(body.velocity, Vec3::zero());
}

#[test]
fn test_apply_force() {
    let mut body = RigidBody::new(Vec3::zero());
    let force = Vec3::new(10.0, 0.0, 0.0);

    body.apply_force(force);
    body.update(1.0);

    // F = ma, a = F/m = 10/1 = 10
    // v = a*t = 10*1 = 10
    // p = v*t = 10*1 = 10
    assert_eq!(body.acceleration.x, 10.0);
    assert_eq!(body.velocity.x, 10.0);
    assert_eq!(body.position.x, 10.0);
}

#[test]
fn test_apply_multiple_forces() {
    let mut body = RigidBody::new(Vec3::zero());

    body.apply_force(Vec3::new(10.0, 0.0, 0.0));
    body.apply_force(Vec3::new(5.0, 20.0, 0.0));
    body.update(1.0);

    // Total force: (15, 20, 0)
    // a = F/m = (15, 20, 0)
    assert_eq!(body.acceleration.x, 15.0);
    assert_eq!(body.acceleration.y, 20.0);
    assert_eq!(body.velocity.x, 15.0);
    assert_eq!(body.velocity.y, 20.0);
}

#[test]
fn test_forces_cleared_after_update() {
    let mut body = RigidBody::new(Vec3::zero());

    body.apply_force(Vec3::new(10.0, 0.0, 0.0));
    body.update(1.0);

    // First update: acceleration should be 10
    assert_eq!(body.acceleration.x, 10.0);

    // Second update with no new forces: acceleration should be 0
    body.update(1.0);
    assert_eq!(body.acceleration.x, 0.0);
    // But velocity should remain (no damping)
    assert_eq!(body.velocity.x, 10.0);
}

#[test]
fn test_newtons_second_law_different_masses() {
    let mut light_body = RigidBody::with_mass(Vec3::zero(), 1.0);
    let mut heavy_body = RigidBody::with_mass(Vec3::zero(), 10.0);

    let force = Vec3::new(100.0, 0.0, 0.0);

    light_body.apply_force(force);
    heavy_body.apply_force(force);

    light_body.update(1.0);
    heavy_body.update(1.0);

    // Light body: a = 100/1 = 100
    assert_eq!(light_body.acceleration.x, 100.0);
    assert_eq!(light_body.velocity.x, 100.0);

    // Heavy body: a = 100/10 = 10
    assert_eq!(heavy_body.acceleration.x, 10.0);
    assert_eq!(heavy_body.velocity.x, 10.0);
}

#[test]
fn test_gravity_force() {
    let mut body = RigidBody::with_mass(Vec3::new(0.0, 100.0, 0.0), 2.0);

    // Apply gravity: F = mg = 2 * -9.8 = -19.6
    let gravity_force = Vec3::new(0.0, -9.8, 0.0) * body.mass;
    body.apply_force(gravity_force);

    body.update(1.0);

    // a = F/m = -19.6/2 = -9.8
    assert_eq!(body.acceleration.y, -9.8);
    assert_eq!(body.velocity.y, -9.8);
    assert_eq!(body.position.y, 100.0 - 9.8);
}

#[test]
fn test_euler_integration_accuracy() {
    let mut body = RigidBody::new(Vec3::zero());

    // Apply constant force
    let force = Vec3::new(10.0, 0.0, 0.0);

    // Simulate for 1 second with small timesteps
    let dt = 0.01;
    for _ in 0..100 {
        body.apply_force(force);
        body.update(dt);
    }

    // After 1 second with constant acceleration of 10:
    // v = a*t = 10*1 = 10
    // x = 0.5*a*t^2 = 0.5*10*1 = 5
    assert!((body.velocity.x - 10.0).abs() < 0.01);
    assert!((body.position.x - 5.0).abs() < 0.1);
}

#[test]
fn test_clear_forces() {
    let mut body = RigidBody::new(Vec3::zero());

    body.apply_force(Vec3::new(100.0, 200.0, 300.0));
    body.clear_forces();
    body.update(1.0);

    // Forces were cleared, so no acceleration
    assert_eq!(body.acceleration, Vec3::zero());
    assert_eq!(body.velocity, Vec3::zero());
}
