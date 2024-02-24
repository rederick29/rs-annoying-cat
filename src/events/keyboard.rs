use bevy::{ecs::system::Query, window::Window};
use rand::{distributions::Alphanumeric, Rng};

fn keyboard(mut windows: Query<&mut Window>) {
    let keyboard = enigo::Enigo::new();

    let randomized_characters: String = rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(7)
    .map(char::from)
    .collect();

    println!("{}", randomized_characters);
}