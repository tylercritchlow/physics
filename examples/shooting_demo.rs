use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use physics::{Body, CollisionShape, PhysicsWorld, RigidBody, StaticBody, Vec3};

// Bevy resource to hold our physics world
#[derive(Resource)]
struct OurPhysicsWorld(PhysicsWorld);

// Bevy component to link a Bevy entity to a physics body index
#[derive(Component)]
struct PhysicsBodyId(usize);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut physics_world: ResMut<OurPhysicsWorld>,
) {
    // --- Bevy scene setup ---
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 15.0).looking_at(bevy::math::Vec3::ZERO, bevy::math::Vec3::Y),
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

    // --- Physics and visual setup ---

    // Create and spawn the ground plane
    let ground_shape = CollisionShape::Plane {
        normal: Vec3::new(0.0, 1.0, 0.0),
        distance: 0.0,
    };
    let ground_body = StaticBody::new(Vec3::zero(), ground_shape);
    physics_world.0.add_body(Body::Static(ground_body));

    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0).build()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
        ..default()
    });
}

fn physics_update(
    mut physics_world: ResMut<OurPhysicsWorld>,
    mut query: Query<(&PhysicsBodyId, &mut Transform)>,
) {
    physics_world.0.update(1.0 / 60.0);

    for (physics_body_id, mut transform) in query.iter_mut() {
        if let Some(body) = physics_world.0.bodies.get(physics_body_id.0) {
            let body_pos = body.position();
            transform.translation = bevy::math::Vec3::new(body_pos.x, body_pos.y, body_pos.z);
        }
    }
}

fn shoot_balls(
    mut commands: Commands,
    mut physics_world: ResMut<OurPhysicsWorld>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = camera_query.single();
        let window = window_query.single();

        if let Some(cursor_pos) = window.cursor_position() {
            if let Some(ray) = camera.viewport_to_world(camera_transform, cursor_pos) {
                let ball_radius = 0.3;
                let ball_shape = CollisionShape::Sphere { radius: ball_radius };
                
                let spawn_pos_bevy = ray.origin + ray.direction * 2.0;
                let spawn_pos_physics = Vec3::new(spawn_pos_bevy.x, spawn_pos_bevy.y, spawn_pos_bevy.z);

                let velocity_bevy = ray.direction * 20.0;
                let velocity_physics = Vec3::new(velocity_bevy.x, velocity_bevy.y, velocity_bevy.z);

                let mut ball_body = RigidBody::with_shape(
                    spawn_pos_physics,
                    1.0,
                    ball_shape,
                );
                
                ball_body.velocity = velocity_physics;
                ball_body.friction = 0.5;

                let body_index = physics_world.0.bodies.len();
                physics_world.0.add_body(Body::Rigid(ball_body));

                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(Sphere::new(ball_radius).mesh().build()),
                        material: materials.add(Color::rgb(0.8, 0.1, 0.1)),
                        transform: Transform::from_translation(spawn_pos_bevy),
                        ..default()
                    },
                    PhysicsBodyId(body_index),
                ));
            }
        }
    }
}

fn main() {
    let mut physics_world = PhysicsWorld::new(1.0 / 60.0);
    physics_world.restitution = 0.6;

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(OurPhysicsWorld(physics_world))
        .add_systems(Startup, setup)
        .add_systems(Update, (physics_update, shoot_balls))
        .run();
}
