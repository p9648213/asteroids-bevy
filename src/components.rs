use bevy::{ecs::component::Component, math::Quat};

#[derive(Component, Debug)]
pub struct Position(pub f32, pub f32);

#[derive(Component)]
pub struct Velocity(pub f32, pub f32);

#[derive(Component, Debug)]
pub struct RotateSpeed(pub f32);

#[derive(Component)]
pub struct Rotation(pub Quat);

#[derive(Component)]
pub struct Thrust(pub bool);
