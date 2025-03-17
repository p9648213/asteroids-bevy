use bevy::color::Color;
use bevy::prelude::{BuildChildren, Children, Rectangle};
use bevy::sprite::{ColorMaterial, MeshMaterial2d};
use bevy::{
    asset::{Assets, Handle},
    core_pipeline::core_2d::Camera2d,
    ecs::system::{Commands, Query, Res, ResMut},
    input::{ButtonInput, keyboard::KeyCode},
    math::{Quat, Vec3, primitives::Triangle2d},
    render::mesh::{Mesh, Mesh2d},
    time::Time,
    transform::components::Transform,
};

use crate::components::{Position, RotateSpeed, Rotation, Thrust, Velocity};

const NORMAL_SHIP_COLOR_ID: Handle<ColorMaterial> = Handle::weak_from_u128(1234123);
const THRUSTING_SHIP_COLOR_ID: Handle<ColorMaterial> = Handle::weak_from_u128(1234125);

pub fn add_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn add_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    materials.insert(
        NORMAL_SHIP_COLOR_ID.id(),
        ColorMaterial::from_color(Color::WHITE),
    );
    materials.insert(
        THRUSTING_SHIP_COLOR_ID.id(),
        ColorMaterial::from_color(Color::BLACK),
    );

    let ship_mesh = Mesh2d(meshes.add(Triangle2d::default()));
    let ship_mesh_material = MeshMaterial2d(NORMAL_SHIP_COLOR_ID);
    let ship_transform = Transform::default().with_scale(Vec3::splat(100.0));
    let rotation = Rotation(Quat::default());
    let thrust = Thrust(false);
    let velocity = Velocity(Vec3::ZERO);

    let thrust_mesh = Mesh2d(meshes.add(Rectangle::new(0.3, 0.5)));
    let thrust_mesh_material = MeshMaterial2d(NORMAL_SHIP_COLOR_ID);
    let thrust_mesh_transform = Transform::default().with_translation(Vec3::new(0.0, -0.3, -0.1));

    let ship_thruster = commands
        .spawn((thrust_mesh, thrust_mesh_material, thrust_mesh_transform))
        .id();

    let mut ship = commands.spawn((
        Position(Vec3::ZERO),
        ship_mesh,
        ship_mesh_material,
        ship_transform,
        RotateSpeed(0.0),
        rotation,
        thrust,
        velocity,
    ));

    ship.add_child(ship_thruster);
}

pub fn draw_ship(
    mut query: Query<(&mut Transform, &Position, &Thrust, &Children)>,
    mut commands: Commands,
) {
    for (mut tranforms, position, thrust, children) in &mut query {
        tranforms.translation.x = position.0.x;
        tranforms.translation.y = position.0.y;

        let thrusters = children
            .first()
            .expect("Couldn't find the first child, which should be thrusters");

        if thrust.0 {
            commands
                .entity(*thrusters)
                .insert(MeshMaterial2d(THRUSTING_SHIP_COLOR_ID));
        } else {
            commands
                .entity(*thrusters)
                .insert(MeshMaterial2d(NORMAL_SHIP_COLOR_ID));
        }
    }
}

pub fn rotate_ship(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &RotateSpeed, &mut Rotation)>,
) {
    for (mut transform, rotate_speed, mut rotation) in &mut query {
        transform.rotate_z(rotate_speed.0 * time.delta_secs());
        rotation.0 = transform.rotation;
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

pub fn input_thrust_ship(keyboard_input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Thrust>) {
    for mut thrust in &mut query {
        thrust.0 = keyboard_input.pressed(KeyCode::ArrowUp);
    }
}

pub fn apply_thrust(mut query: Query<(&Thrust, &mut Velocity, &Transform)>) {
    for (thrust, mut velocity, transform) in &mut query {
        if thrust.0 {
            let acceleration = 1.0;

            let direction = transform.rotation * Vec3::Y;
            let direction = direction.normalize();

            velocity.0 += acceleration * direction;
        }
    }
}

pub fn apply_velocity(time: Res<Time>, mut query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in &mut query {
        position.0 += velocity.0 * time.delta_secs();
    }
}
