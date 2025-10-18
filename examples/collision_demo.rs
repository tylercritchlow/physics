use bevy::prelude::*;
use bevy::math::Vec3 as BevyVec3;
use physics::{Body, PhysicsWorld, RigidBody, Vec3};

// Bevy resource to hold our physics world
#[derive(Resource)]
struct OurPhysicsWorld(PhysicsWorld);

// Bevy component to link a Bevy entity to a physics rigid body
#[derive(Component)]
struct PhysicsBodyId(usize);

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 10.0).looking_at(BevyVec3::ZERO, BevyVec3::Y),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 10000.0,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 1.0, -0.6)),
        ..default()
    });

    // Sphere meshes and materials
    let sphere_mesh = meshes.add(Sphere::new(0.5).mesh());
    let red_material = materials.add(StandardMaterial::from(Color::rgb(1.0, 0.3, 0.3)));
    let blue_material = materials.add(StandardMaterial::from(Color::rgb(0.3, 0.3, 1.0)));

    // Spawn Bevy entities for the spheres
    commands.spawn(
        (PbrBundle {
            mesh: sphere_mesh.clone(),
            material: red_material,
            transform: Transform::from_xyz(-5.0, 0.0, 0.0),
            ..default()
        },
        PhysicsBodyId(0), // Link to the first physics body
    ));

    commands.spawn(
        (PbrBundle {
            mesh: sphere_mesh,
            material: blue_material,
            transform: Transform::from_xyz(5.0, 0.0, 0.0),
            ..default()
        },
        PhysicsBodyId(1), // Link to the second physics body
    ));
}

fn physics_update(mut physics_world: ResMut<OurPhysicsWorld>, mut query: Query<(&PhysicsBodyId, &mut Transform)>) {
    // Update physics world
    physics_world.0.update(1.0 / 60.0);

    // Update Bevy entity transforms based on physics simulation
    for (physics_body_id, mut transform) in query.iter_mut() {
        if let Some(body) = physics_world.0.bodies.get(physics_body_id.0) {
            transform.translation = BevyVec3::new(body.position().x, body.position().y, body.position().z);
        }
    }
}

fn main() {
    // Create physics world with no gravity and low-ish bounciness
    let mut physics_world = PhysicsWorld::with_gravity(1.0 / 60.0, Vec3::zero());
    physics_world.restitution = 0.3;

    // Create two spheres shooting at each other
    let mut sphere1 = RigidBody::with_mass(Vec3::new(-5.0, 0.0, 0.0), 1.0);
    sphere1.velocity = Vec3::new(3.0, 0.0, 0.0);

    let mut sphere2 = RigidBody::with_mass(Vec3::new(5.0, 0.0, 0.0), 1.0);
    sphere2.velocity = Vec3::new(-3.0, 0.0, 0.0);

    physics_world.add_body(Body::Rigid(sphere1));
    physics_world.add_body(Body::Rigid(sphere2));

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(OurPhysicsWorld(physics_world))
        .add_systems(Startup, setup)
        .add_systems(Update, physics_update)
        .run();
}
