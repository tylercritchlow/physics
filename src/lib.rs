pub mod body;
pub mod collision;
pub mod physics_loop;
pub mod vector;

pub use body::{Body, RigidBody, StaticBody};
pub use collision::{
    aabb_vs_aabb, sphere_vs_aabb, sphere_vs_plane, sphere_vs_sphere, CollisionInfo,
    CollisionShape,
};
pub use physics_loop::PhysicsWorld;
pub use vector::Vec3;