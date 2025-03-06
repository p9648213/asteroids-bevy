use bevy::ecs::component::Component;

#[derive(Component, Debug)]
pub struct Position(pub f32, pub f32);

#[derive(Component, Debug)]
pub struct RotateSpeed(pub f32);
