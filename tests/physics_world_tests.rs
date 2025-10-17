use physics::{PhysicsWorld, RigidBody, Vec3};

#[test]
fn test_physics_world_creation() {
    let world = PhysicsWorld::new(1.0 / 60.0);
    assert_eq!(world.bodies.len(), 0);
}

#[test]
fn test_physics_world_add_body() {
    let mut world = PhysicsWorld::new(1.0 / 60.0);
    let body = RigidBody::new(Vec3::new(1.0, 2.0, 3.0));

    world.add_body(body);
    assert_eq!(world.bodies.len(), 1);
    assert_eq!(world.bodies[0].position.x, 1.0);
}

#[test]
fn test_physics_world_add_multiple_bodies() {
    let mut world = PhysicsWorld::new(1.0 / 60.0);

    for i in 0..10 {
        let body = RigidBody::new(Vec3::new(i as f32, 0.0, 0.0));
        world.add_body(body);
    }

    assert_eq!(world.bodies.len(), 10);
}

#[test]
fn test_physics_world_fixed_timestep_update() {
    let mut world = PhysicsWorld::new(1.0 / 60.0);
    let mut body = RigidBody::new(Vec3::zero());
    body.velocity = Vec3::new(1.0, 0.0, 0.0);
    world.add_body(body);

    let dt = 1.0 / 60.0;
    world.update(dt);

    assert!((world.bodies[0].position.x - (1.0 / 60.0)).abs() < 0.0001);
}

#[test]
fn test_physics_world_accumulator() {
    let mut world = PhysicsWorld::new(1.0 / 60.0);
    let mut body = RigidBody::new(Vec3::zero());
    body.velocity = Vec3::new(60.0, 0.0, 0.0);
    world.add_body(body);

    let small_dt = 1.0 / 120.0;
    world.update(small_dt);

    assert_eq!(world.bodies[0].position.x, 0.0);

    world.update(small_dt);

    assert!((world.bodies[0].position.x - 1.0).abs() < 0.0001);
}

#[test]
fn test_physics_world_multiple_fixed_steps_per_frame() {
    let mut world = PhysicsWorld::new(1.0 / 60.0);
    let mut body = RigidBody::new(Vec3::zero());
    body.velocity = Vec3::new(60.0, 0.0, 0.0);
    world.add_body(body);

    let large_dt = 3.0 / 60.0;
    world.update(large_dt);

    assert!((world.bodies[0].position.x - 3.0).abs() < 0.0001);
}

#[test]
fn test_physics_world_determinism() {
    let mut world1 = PhysicsWorld::new(1.0 / 60.0);
    let mut body1 = RigidBody::new(Vec3::zero());
    body1.velocity = Vec3::new(1.0, 2.0, 3.0);
    world1.add_body(body1);

    let mut world2 = PhysicsWorld::new(1.0 / 60.0);
    let mut body2 = RigidBody::new(Vec3::zero());
    body2.velocity = Vec3::new(1.0, 2.0, 3.0);
    world2.add_body(body2);

    for _ in 0..100 {
        world1.update(1.0 / 60.0);
        world2.update(1.0 / 60.0);
    }

    assert_eq!(world1.bodies[0].position.x, world2.bodies[0].position.x);
    assert_eq!(world1.bodies[0].position.y, world2.bodies[0].position.y);
    assert_eq!(world1.bodies[0].position.z, world2.bodies[0].position.z);
}

#[test]
fn test_physics_world_variable_timestep_stability() {
    let mut world = PhysicsWorld::new(1.0 / 60.0);
    let mut body = RigidBody::new(Vec3::zero());
    body.velocity = Vec3::new(1.0, 0.0, 0.0);
    world.add_body(body);

    world.update(1.0 / 30.0);
    world.update(1.0 / 120.0);
    world.update(1.0 / 45.0);
    world.update(1.0 / 90.0);

    let total_time = 1.0 / 30.0 + 1.0 / 120.0 + 1.0 / 45.0 + 1.0 / 90.0;
    let expected_position = total_time;

    assert!((world.bodies[0].position.x - expected_position).abs() < 0.01);
}

#[test]
fn test_physics_world_zero_delta_time() {
    let mut world = PhysicsWorld::new(1.0 / 60.0);
    let mut body = RigidBody::new(Vec3::zero());
    body.velocity = Vec3::new(1.0, 0.0, 0.0);
    world.add_body(body);

    world.update(0.0);

    assert_eq!(world.bodies[0].position.x, 0.0);
}

#[test]
fn test_physics_world_all_bodies_update() {
    let mut world = PhysicsWorld::new(1.0 / 60.0);

    // Space bodies apart to avoid collisions (default sphere radius is 0.5, so use spacing > 1.0)
    for i in 0..5 {
        let mut body = RigidBody::new(Vec3::new(i as f32 * 2.0, 0.0, 0.0));
        body.velocity = Vec3::new(1.0, 0.0, 0.0);
        world.add_body(body);
    }

    world.update(1.0);

    for (i, body) in world.bodies.iter().enumerate() {
        let expected_x = i as f32 * 2.0 + 1.0; // initial + velocity * time
        assert!((body.position.x - expected_x).abs() < 0.0001);
    }
}

// Gravity and Force Tests
#[test]
fn test_physics_world_default_gravity() {
    let world = PhysicsWorld::new(1.0 / 60.0);
    assert_eq!(world.gravity, Vec3::new(0.0, -9.8, 0.0));
}

#[test]
fn test_physics_world_custom_gravity() {
    let custom_gravity = Vec3::new(0.0, -20.0, 0.0);
    let world = PhysicsWorld::with_gravity(1.0 / 60.0, custom_gravity);
    assert_eq!(world.gravity, custom_gravity);
}

#[test]
fn test_physics_world_applies_gravity() {
    let mut world = PhysicsWorld::new(1.0 / 60.0);
    let body = RigidBody::with_mass(Vec3::new(0.0, 100.0, 0.0), 2.0);
    world.add_body(body);

    // Update for 1 second
    world.update(1.0);

    // Gravity force = mass * gravity = 2.0 * -9.8 = -19.6
    // Acceleration = force / mass = -19.6 / 2.0 = -9.8
    // Semi-implicit Euler: v = -9.8, position = 100 - 0.5*9.8*1^2 ≈ 95.1
    assert!((world.bodies[0].velocity.y - (-9.8)).abs() < 0.01);
    assert!((world.bodies[0].position.y - 95.1).abs() < 0.1);
}

#[test]
fn test_physics_world_gravity_affects_all_bodies() {
    let mut world = PhysicsWorld::new(1.0 / 60.0);

    // Add bodies with different masses
    world.add_body(RigidBody::with_mass(Vec3::new(0.0, 100.0, 0.0), 1.0));
    world.add_body(RigidBody::with_mass(Vec3::new(0.0, 100.0, 0.0), 5.0));
    world.add_body(RigidBody::with_mass(Vec3::new(0.0, 100.0, 0.0), 10.0));

    world.update(1.0);

    // All bodies should fall at the same rate (a = g = -9.8) regardless of mass
    for body in &world.bodies {
        assert!((body.acceleration.y - (-9.8)).abs() < 0.0001);
        assert!((body.velocity.y - (-9.8)).abs() < 0.01);
    }
}

#[test]
fn test_physics_world_no_gravity() {
    let mut world = PhysicsWorld::with_gravity(1.0 / 60.0, Vec3::zero());
    let body = RigidBody::new(Vec3::new(0.0, 100.0, 0.0));
    world.add_body(body);

    world.update(1.0);

    // No gravity, body should stay at same height
    assert_eq!(world.bodies[0].position.y, 100.0);
    assert_eq!(world.bodies[0].velocity.y, 0.0);
}

#[test]
fn test_physics_world_falling_simulation() {
    let mut world = PhysicsWorld::new(0.01); // 100 Hz
    let body = RigidBody::with_mass(Vec3::new(0.0, 100.0, 0.0), 1.0);
    world.add_body(body);

    // Simulate falling for 3 seconds
    for _ in 0..300 {
        world.update(0.01);
    }

    // After 3 seconds of free fall from 100m with g = -9.8:
    // v = gt = -9.8 * 3 = -29.4
    // y = y0 + 0.5*g*t^2 = 100 + 0.5*(-9.8)*9 = 100 - 44.1 = 55.9
    assert!((world.bodies[0].velocity.y - (-29.4)).abs() < 0.5);
    assert!((world.bodies[0].position.y - 55.9).abs() < 2.0);
}

#[test]
fn test_physics_world_gravity_with_initial_velocity() {
    let mut world = PhysicsWorld::new(1.0 / 60.0);
    let mut body = RigidBody::new(Vec3::zero());
    body.velocity = Vec3::new(0.0, 50.0, 0.0); // Throw upward
    world.add_body(body);

    world.update(1.0);

    // After 1 second: v = v0 + at = 50 + (-9.8)*1 = 40.2
    assert!((world.bodies[0].velocity.y - 40.2).abs() < 0.1);
}
