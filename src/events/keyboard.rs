use bevy::{ecs::system::Query, window::Window};
use rand::{distributions::Alphanumeric, Rng};

pub fn keyboard_type(mut windows: Query<&mut Window>) {
    let keyboard_enigo = enigo::Enigo::new();

    let mut rng = rand::thread_rng();
    let random_character_count: u8 = rng.gen_range(16..56);

    let random_characters: String = rng.sample_iter(&Alphanumeric)
        .take(random_character_count.into())
        .map(char::from)
        .collect();

    println!(">> {}", random_characters);
}