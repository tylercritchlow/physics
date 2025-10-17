use blue_engine::{ObjectSettings, prelude::Engine, primitive_shapes::uv_sphere};
use physics::{PhysicsWorld, RigidBody, Vec3};

// This is the classic, "if this thing is heavier, 
// it will reach the ground at the same time as the lighter one falling from a shorter distance"
// GOTCHA, it actually falls at the same speed buddy

fn main() -> Result<(), blue_engine::error::Error> {
    let mut engine = Engine::new()?;

    // Create physics world with default gravity (-9.8 m/s²)
    let mut world = PhysicsWorld::new(1.0 / 60.0);

    // Create three spheres with different masses
    let light_sphere = RigidBody::with_mass(Vec3::new(-2.0, 5.0, 0.0), 0.5);
    let medium_sphere = RigidBody::with_mass(Vec3::new(0.0, 8.0, 0.0), 2.0);
    let heavy_sphere = RigidBody::with_mass(Vec3::new(2.0, 10.0, 0.0), 5.0);

    world.add_body(light_sphere);
    world.add_body(medium_sphere);
    world.add_body(heavy_sphere);

    // Create visual spheres
    uv_sphere(
        "light_sphere",
        ObjectSettings::default(),
        (32, 32, 0.3),
        &mut engine.renderer,
        &mut engine.objects,
    )?;

    uv_sphere(
        "medium_sphere",
        ObjectSettings::default(),
        (32, 32, 0.5),
        &mut engine.renderer,
        &mut engine.objects,
    )?;

    uv_sphere(
        "heavy_sphere",
        ObjectSettings::default(),
        (32, 32, 0.7),
        &mut engine.renderer,
        &mut engine.objects,
    )?;

    // Set colors - lighter to darker based on mass
    engine
        .objects
        .get_mut("light_sphere")
        .unwrap()
        .set_color(0.3, 0.8, 1.0, 1.0);

    engine
        .objects
        .get_mut("medium_sphere")
        .unwrap()
        .set_color(0.2, 0.5, 0.9, 1.0);

    engine
        .objects
        .get_mut("heavy_sphere")
        .unwrap()
        .set_color(0.1, 0.3, 0.7, 1.0);

    engine.update_loop(move |engine| {
        world.update(1.0 / 60.0);

        // Update sphere positions
        let light_pos = &world.bodies[0].position;
        engine
            .objects
            .get_mut("light_sphere")
            .unwrap()
            .set_position((light_pos.x, light_pos.y, light_pos.z));

        let medium_pos = &world.bodies[1].position;
        engine
            .objects
            .get_mut("medium_sphere")
            .unwrap()
            .set_position((medium_pos.x, medium_pos.y, medium_pos.z));

        let heavy_pos = &world.bodies[2].position;
        engine
            .objects
            .get_mut("heavy_sphere")
            .unwrap()
            .set_position((heavy_pos.x, heavy_pos.y, heavy_pos.z));
    })?;

    Ok(())
}
