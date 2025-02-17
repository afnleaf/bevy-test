use bevy::prelude::*;

// bevy is an Entity Component System
// Systems are made up of Entities that contain within them Components?
// or more like you have Entities and systems are stuff that run on them?

// components are Rust structs that implement the Component trait
#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

// a system is a normal rust function
fn print_position_system(query: Query<&Position>) {
    for position in &query {
        println!("position: {} {}", position.x, position.y);
    }
}
// an entity is a simple type that contains a unique integer
// hwat?
struct Entity(u64);

fn hello_world() {
    println!("hello world!");
}

// some components
#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Guy Fleury".to_string())));
    commands.spawn((Person, Name("Shinzo Abe".to_string())));
    commands.spawn((Person, Name("Lionel Messi".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}

fn main() {
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, greet_people))
        .run();
}
