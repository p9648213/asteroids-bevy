use bevy::{
    asset::Assets,
    color::Color,
    core_pipeline::core_2d::Camera2d,
    ecs::system::{Commands, Query, Res, ResMut},
    input::{ButtonInput, keyboard::KeyCode},
    math::{Vec3, primitives::Triangle2d},
    render::mesh::{Mesh, Mesh2d},
    sprite::{ColorMaterial, MeshMaterial2d},
    time::Time,
    transform::components::Transform,
};

use crate::components::{Position, RotateSpeed};

pub fn add_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn add_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let ship_mesh = Mesh2d(meshes.add(Triangle2d::default()));
    let ship_mesh_material = MeshMaterial2d(materials.add(Color::WHITE));
    let ship_transform = Transform::default().with_scale(Vec3::splat(100.0));

    commands.spawn((
        Position(50.0, 50.0),
        ship_mesh,
        ship_mesh_material,
        ship_transform,
        RotateSpeed(0.0),
    ));
}

pub fn draw_ship(mut query: Query<(&mut Transform, &Position)>) {
    for (mut tranforms, position) in &mut query {
        tranforms.translation.x = position.0;
        tranforms.translation.y = position.1;
    }
}

pub fn rotate_ship(time: Res<Time>, mut query: Query<(&mut Transform, &RotateSpeed)>) {
    for (mut transform, rotate_speed) in &mut query {
        transform.rotate_z(rotate_speed.0 * time.delta_secs());
    }
}

pub fn input_rotate_ship(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut RotateSpeed>,
) {
    for mut rotate_speed in &mut query {
        rotate_speed.0 = if keyboard_input.pressed(KeyCode::ArrowLeft) {
            1.0
        } else if keyboard_input.pressed(KeyCode::ArrowRight) {
            -1.0
        } else {
            0.0
        };

        rotate_speed.0 = rotate_speed.0.clamp(-5.0, 5.0);
    }
}
