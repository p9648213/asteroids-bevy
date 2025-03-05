use bevy::{
    DefaultPlugins,
    app::{App, Plugin, Startup, Update},
    asset::Assets,
    color::Color,
    core_pipeline::core_2d::Camera2d,
    ecs::{
        component::Component,
        system::{Commands, Query, ResMut},
    },
    math::{Vec3, primitives::Triangle2d},
    render::mesh::{Mesh, Mesh2d},
    sprite::{ColorMaterial, MeshMaterial2d},
    transform::components::Transform,
};

pub fn run() {
    App::new().add_plugins((DefaultPlugins, Game)).run();
}

struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (add_player, add_camera));
        app.add_systems(Update, draw_ship);
    }
}

#[derive(Component)]
struct Position(f32, f32);

fn add_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn add_player(
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
    ));
}

fn draw_ship(mut query: Query<(&mut Transform, &Position)>) {
    for (mut tranforms, position) in &mut query {
        tranforms.translation.x = position.0;
        tranforms.translation.y = position.1;
    }
}
