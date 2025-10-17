use blue_engine::{ObjectSettings, prelude::Engine, primitive_shapes::uv_sphere};
use physics::{PhysicsWorld, RigidBody, Vec3};

fn main() -> Result<(), blue_engine::error::Error> {
    let mut engine = Engine::new()?;

    // Create physics world with no gravity and low-ish bounciness
    let mut world = PhysicsWorld::with_gravity(1.0 / 60.0, Vec3::zero());
    world.restitution = 0.3;

    // Create two spheres shooting at each other
    let mut sphere1 = RigidBody::with_mass(Vec3::new(-5.0, 0.0, 0.0), 1.0);
    sphere1.velocity = Vec3::new(3.0, 0.0, 0.0); // Moving right

    let mut sphere2 = RigidBody::with_mass(Vec3::new(5.0, 0.0, 0.0), 1.0);
    sphere2.velocity = Vec3::new(-3.0, 0.0, 0.0); // Moving left

    world.add_body(sphere1);
    world.add_body(sphere2);

    // Create visual spheres
    uv_sphere(
        "sphere1",
        ObjectSettings::default(),
        (32, 32, 0.5),
        &mut engine.renderer,
        &mut engine.objects,
    )?;

    uv_sphere(
        "sphere2",
        ObjectSettings::default(),
        (32, 32, 0.5),
        &mut engine.renderer,
        &mut engine.objects,
    )?;

    engine.objects.get_mut("sphere1").unwrap().set_color(1.0, 0.3, 0.3, 1.0); // Red
    engine.objects.get_mut("sphere2").unwrap().set_color(0.3, 0.3, 1.0, 1.0); // Blue

    engine.update_loop(move |engine| {
        world.update(1.0 / 60.0);

        // Update sphere positions
        engine
            .objects
            .get_mut("sphere1")
            .unwrap()
            .set_position((world.bodies[0].position.x, world.bodies[0].position.y, world.bodies[0].position.z));

        engine
            .objects
            .get_mut("sphere2")
            .unwrap()
            .set_position((world.bodies[1].position.x, world.bodies[1].position.y, world.bodies[1].position.z));
    })?;

    Ok(())
}
