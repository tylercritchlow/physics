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
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(BevyVec3::ZERO, BevyVec3::Y),
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

    // Sphere (Bevy entity)
    let sphere_mesh = meshes.add(Sphere::new(0.5).mesh());
    let sphere_material = materials.add(StandardMaterial::from(Color::rgb(0.2, 0.6, 1.0)));

    commands.spawn(
        (PbrBundle {
            mesh: sphere_mesh,
            material: sphere_material,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        PhysicsBodyId(0), // Link to the first physics body
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

    let mut sphere_body = RigidBody::new(Vec3::new(0.0, 0.0, 0.0));
    sphere_body.velocity = Vec3::new(1.0, 0.5, 0.0);
    physics_world.add_body(Body::Rigid(sphere_body)); // Wrap RigidBody in Body::Rigid

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(OurPhysicsWorld(physics_world))
        .add_systems(Startup, setup)
        .add_systems(Update, physics_update)
        .run();
}
