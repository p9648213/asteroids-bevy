mod components;
mod system;

use bevy::{
    DefaultPlugins,
    app::{App, Plugin, Startup, Update},
    ecs::schedule::IntoSystemConfigs,
};
use system::{add_camera, add_player, draw_ship, input_rotate_ship, rotate_ship};

pub fn run() {
    App::new().add_plugins((DefaultPlugins, Game)).run();
}

struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (add_player, add_camera));
        app.add_systems(
            Update,
            (draw_ship, (input_rotate_ship, rotate_ship).chain()),
        );
    }
}
