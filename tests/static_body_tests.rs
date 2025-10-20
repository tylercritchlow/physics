use physics::{Body, CollisionShape, PhysicsWorld, RigidBody, StaticBody, Vec3};

#[test]
fn test_static_body_creation() {
    let static_body = StaticBody::new(Vec3::new(1.0, 2.0, 3.0), CollisionShape::Sphere { radius: 0.5 });
    assert_eq!(static_body.position, Vec3::new(1.0, 2.0, 3.0));
    assert_eq!(static_body.shape, CollisionShape::Sphere { radius: 0.5 });
}

#[test]
fn test_static_body_immovability() {
    let fixed_timestep = 1.0 / 60.0;
    let mut world = PhysicsWorld::new(fixed_timestep);
    world.gravity = Vec3::new(0.0, -9.8, 0.0);

    let initial_pos = Vec3::new(0.0, 0.0, 0.0);
    let static_body = StaticBody::new(initial_pos, CollisionShape::Sphere { radius: 0.5 });
    world.add_body(Body::Static(static_body));

    // Simulate for a few steps
    for _ in 0..10 {
        world.update(fixed_timestep);
    }

    // Assert static body position remains unchanged
    assert_eq!(*world.bodies[0].position(), initial_pos);
    // Assert that trying to get a rigid body from it returns None
    assert!(world.bodies[0].as_rigid_body().is_none());
}

#[test]
fn test_rigid_static_collision_response() {
    let fixed_timestep = 1.0 / 60.0;
    let mut world = PhysicsWorld::new(fixed_timestep);
    world.gravity = Vec3::new(0.0, -9.8, 0.0);
    world.restitution = 0.1;
    world.friction = 0.8; // Increased friction for this test

    // Static sphere at the bottom
    let static_pos = Vec3::new(0.0, 0.0, 0.0);
    let static_radius = 0.5;
    let static_body = StaticBody::new(static_pos, CollisionShape::Sphere { radius: static_radius });
    world.add_body(Body::Static(static_body));

    // Dynamic sphere dropped from above
    let dynamic_initial_pos = Vec3::new(0.0, 1.001, 0.0);
    let dynamic_radius = 0.5;
    let mut dynamic_rb = RigidBody::new(dynamic_initial_pos);
    dynamic_rb.shape = CollisionShape::Sphere { radius: dynamic_radius };
    dynamic_rb.velocity = Vec3::new(0.0, 0.0, 0.0);
    world.add_body(Body::Rigid(dynamic_rb));

    // Simulate until collision and settling
    let num_steps = 200; // Enough steps for it to fall and settle
    for step in 0..num_steps {
        world.update(fixed_timestep);
        let dynamic_pos = world.bodies[1].position();
        let dynamic_vel = world.bodies[1].as_rigid_body().unwrap().velocity;
        println!("[DEBUG TEST] Step {}: Pos={:?}, Vel={:?}", step, dynamic_pos, dynamic_vel);
    }

    // Assert static body has not moved
    assert_eq!(*world.bodies[0].position(), static_pos);

    // Assert dynamic body is on top of static body and has settled
    let final_dynamic_pos = world.bodies[1].position();
    let final_dynamic_vel = world.bodies[1].as_rigid_body().unwrap().velocity;

    // Expected y position: static_pos.y + static_radius + dynamic_radius
    let expected_y = static_pos.y + static_radius + dynamic_radius;
    assert!((final_dynamic_pos.y - expected_y).abs() < 0.01, "Dynamic body did not settle at expected height");
    assert!(final_dynamic_vel.magnitude() < 0.1, "Dynamic body did not settle (velocity too high)");
}

#[test]
fn test_static_static_no_interaction() {
    let fixed_timestep = 1.0 / 60.0;
    let mut world = PhysicsWorld::new(fixed_timestep);
    world.friction = 0.5; // Explicitly set friction for this test

    let static_pos1 = Vec3::new(0.0, 0.0, 0.0);
    let static_pos2 = Vec3::new(0.0, 0.5, 0.0); // Overlapping

    let static_body1 = StaticBody::new(static_pos1, CollisionShape::Sphere { radius: 0.5 });
    let static_body2 = StaticBody::new(static_pos2, CollisionShape::Sphere { radius: 0.5 });

    world.add_body(Body::Static(static_body1));
    world.add_body(Body::Static(static_body2));

    // Simulate for a few steps
    for _ in 0..10 {
        world.update(fixed_timestep);
    }

    // Assert both static bodies remain unchanged
    assert_eq!(*world.bodies[0].position(), static_pos1);
    assert_eq!(*world.bodies[1].position(), static_pos2);
}
