use bevy::prelude::*;
use bevy::math::Vec3 as BevyVec3;
use physics::{Body, PhysicsWorld, RigidBody, StaticBody, Vec3, CollisionShape};

#[derive(Resource)]
struct OurPhysicsWorld(PhysicsWorld);

#[derive(Component)]
struct PhysicsBodyId(usize);

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 3.0, 8.0).looking_at(BevyVec3::ZERO, BevyVec3::Y),
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

    let sphere_mesh = meshes.add(Sphere::new(0.5).mesh());
    let static_material = materials.add(StandardMaterial::from(Color::rgb(0.8, 0.8, 0.8))); // Grey for static
    let dynamic_material = materials.add(StandardMaterial::from(Color::rgb(0.2, 0.6, 1.0))); // Blue for dynamic

    commands.spawn(
        (PbrBundle {
            mesh: sphere_mesh.clone(),
            material: static_material,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        PhysicsBodyId(0),
    ));

    commands.spawn(
        (PbrBundle {
            mesh: sphere_mesh,
            material: dynamic_material,
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        },
        PhysicsBodyId(1),
    ));
}

fn physics_update(mut physics_world: ResMut<OurPhysicsWorld>, mut query: Query<(&PhysicsBodyId, &mut Transform)>) {
    physics_world.0.update(1.0 / 60.0);

    for (physics_body_id, mut transform) in query.iter_mut() {
        if let Some(body) = physics_world.0.bodies.get(physics_body_id.0) {
            transform.translation = BevyVec3::new(body.position().x, body.position().y, body.position().z);
        }
    }
}

fn main() {
    let mut physics_world = PhysicsWorld::new(1.0 / 60.0);
    physics_world.restitution = 0.1; // Reduced restitution

    let static_sphere = StaticBody::new(Vec3::new(0.0, -2.0, 0.0), CollisionShape::Sphere { radius: 0.5 });
    physics_world.add_body(Body::Static(static_sphere));

    let mut dynamic_sphere = RigidBody::new(Vec3::new(0.1, 4.0, 0.0));
    dynamic_sphere.velocity = Vec3::new(0.0, 0.0, 0.0);
    physics_world.add_body(Body::Rigid(dynamic_sphere));

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(OurPhysicsWorld(physics_world))
        .add_systems(Startup, setup)
        .add_systems(Update, physics_update)
        .run();
}
