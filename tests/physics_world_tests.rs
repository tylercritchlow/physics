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

    for i in 0..5 {
        let mut body = RigidBody::new(Vec3::zero());
        body.velocity = Vec3::new(i as f32, 0.0, 0.0);
        world.add_body(body);
    }

    world.update(1.0);

    for (i, body) in world.bodies.iter().enumerate() {
        assert!((body.position.x - i as f32).abs() < 0.0001);
    }
}
