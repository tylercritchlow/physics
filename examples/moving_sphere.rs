use blue_engine::{ObjectSettings, prelude::Engine, primitive_shapes::uv_sphere};
use physics::{PhysicsWorld, RigidBody, Vec3};

fn main() -> Result<(), blue_engine::error::Error> {
    let mut engine = Engine::new()?;

    let mut world = PhysicsWorld::new(1.0 / 60.0);

    let mut sphere = RigidBody::new(Vec3::new(0.0, 0.0, 0.0));
    sphere.velocity = Vec3::new(1.0, 0.5, 0.0);

    world.add_body(sphere);

    uv_sphere(
        "sphere",
        ObjectSettings::default(),
        (64, 64, 0.5),
        &mut engine.renderer,
        &mut engine.objects,
    )?;

    engine
        .objects
        .get_mut("sphere")
        .unwrap()
        .set_color(0.2, 0.6, 1.0, 1.0);

    engine.update_loop(move |engine| {
        world.update(1.0 / 60.0);

        let pos = &world.bodies[0].position;

        engine
            .objects
            .get_mut("sphere")
            .unwrap()
            .set_position((pos.x, pos.y, pos.z));
    })?;

    Ok(())
}
