use core::time;
use std::thread;

use bevy::{asset::AssetServer, audio::AudioBundle, ecs::system::{Commands, Query, Res}, prelude::default, window::Window};
use enigo::{Key, KeyboardControllable};
use rand::{distributions::Alphanumeric, Rng};

pub fn keyboard_type(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut keyboard_enigo = enigo::Enigo::new();

    let mut rng = rand::thread_rng();
    let random_character_count: u8 = rng.gen_range(16..56);

    let random_characters: String = rng.sample_iter(&Alphanumeric)
        .take(random_character_count.into())
        .map(char::from)
        .collect();

    commands.spawn(AudioBundle {
        source: asset_server.load("./audio/pop_1.ogg"),
        ..default()
    });

    keyboard_enigo.key_sequence(&random_characters);

    commands.spawn(AudioBundle {
        source: asset_server.load("./audio/pop_1.ogg"),
        ..default()
    });
}
