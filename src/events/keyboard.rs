use bevy::{asset::AssetServer, audio::AudioBundle, ecs::system::{Commands, Res}, prelude::default};
use enigo::KeyboardControllable;
use rand::{distributions::Alphanumeric, Rng};

pub fn keyboard_type(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut keyboard_enigo = enigo::Enigo::new();

    let mut rng = rand::thread_rng();

    if rng.gen_bool(1.0 / 3.0) {
        let random_character_count: u8 = rng.gen_range(16..56);

        let random_characters: String = rng.sample_iter(&Alphanumeric)
            .take(random_character_count.into())
            .map(char::from)
            .collect();

        // NOTE: We should probably move this to something like a meow method in the cat object.
        commands.spawn(AudioBundle {
            source: asset_server.load("./audio/meow_1.ogg"),
            ..default()
        });

        keyboard_enigo.key_sequence(&random_characters);
    }

}