use bevy::{prelude::*, window::{Cursor, WindowResolution}};
use bevy::window::WindowLevel;
use enigo::MouseControllable;

fn main() {
    let window = Window {
        // Enable transparent support for the window
        transparent: true,
        decorations: false,
        window_level: WindowLevel::AlwaysOnTop,
        resizable: false,
        resolution: WindowResolution::new(100.0, 100.0),
        cursor: Cursor {
            // Allow inputs to pass through to apps behind this app.
            hit_test: false,
            ..default()
        },
        ..default()
    };

    App::new()
        // Make it render background as transparent
        .insert_resource(ClearColor(Color::NONE))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(window),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, move_window)
    .run();

    println!("Hello, world!");
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("cat.png"),
        ..default()
    });
}

fn move_window(mut windows: Query<&mut Window>) {
    let e_mouse = enigo::Enigo::new();
    let (x, y) = e_mouse.mouse_location();
    windows.get_single_mut().unwrap().position = WindowPosition::At(IVec2::new(x + 10, y));
}
