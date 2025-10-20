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
    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 10.0, 15.0).looking_at(BevyVec3::ZERO, BevyVec3::Y),
        ..default()
    });

    // Light
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
    let light_mesh = meshes.add(Sphere::new(0.3).mesh());
    let medium_mesh = meshes.add(Sphere::new(0.5).mesh());
    let heavy_mesh = meshes.add(Sphere::new(0.7).mesh());

    let light_material = materials.add(StandardMaterial::from(Color::rgb(0.3, 0.8, 1.0))); // Light blue
    let medium_material = materials.add(StandardMaterial::from(Color::rgb(0.2, 0.5, 0.9))); // Medium blue
    let heavy_material = materials.add(StandardMaterial::from(Color::rgb(0.1, 0.3, 0.7))); // Dark blue

    // Light Sphere (Bevy entity)
    commands.spawn(
        (PbrBundle {
            mesh: light_mesh,
            material: light_material,
            transform: Transform::from_xyz(-2.0, 5.0, 0.0),
            ..default()
        },
        PhysicsBodyId(0), // Link to the first physics body
    ));

    // Medium Sphere (Bevy entity)
    commands.spawn(
        (PbrBundle {
            mesh: medium_mesh,
            material: medium_material,
            transform: Transform::from_xyz(0.0, 8.0, 0.0),
            ..default()
        },
        PhysicsBodyId(1), // Link to the second physics body
    ));

    // Heavy Sphere (Bevy entity)
    commands.spawn(
        (PbrBundle {
            mesh: heavy_mesh,
            material: heavy_material,
            transform: Transform::from_xyz(2.0, 10.0, 0.0),
            ..default()
        },
        PhysicsBodyId(2), // Link to the third physics body
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
    let mut physics_world = PhysicsWorld::new(1.0 / 60.0);

    // Create three spheres with different masses
    let light_sphere = RigidBody::with_mass(Vec3::new(-2.0, 5.0, 0.0), 0.5);
    let medium_sphere = RigidBody::with_mass(Vec3::new(0.0, 8.0, 0.0), 2.0);
    let heavy_sphere = RigidBody::with_mass(Vec3::new(2.0, 10.0, 0.0), 5.0);

    physics_world.add_body(Body::Rigid(light_sphere));
    physics_world.add_body(Body::Rigid(medium_sphere));
    physics_world.add_body(Body::Rigid(heavy_sphere));

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(OurPhysicsWorld(physics_world))
        .add_systems(Startup, setup)
        .add_systems(Update, physics_update)
        .run();
}