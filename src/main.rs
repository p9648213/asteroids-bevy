use bevy::{
    DefaultPlugins,
    app::{App, Startup, Update},
    ecs::{
        component::Component,
        query::With,
        schedule::IntoSystemConfigs,
        system::{Commands, Query},
    },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, (update_people, greet_people).chain()))
        .run();
}

fn hello_world() {
    println!("hello world");
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Custard_Beard".to_owned())));
    commands.spawn((Person, Name("Stacking".to_owned())));
    commands.spawn((Person, Name("epicblaargh".to_owned())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for Name(name) in &query {
        println!("hello {}!", name);
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0.contains('_') {
            name.0 = name.0.replace('_', " ");
            break;
        }
    }
}
