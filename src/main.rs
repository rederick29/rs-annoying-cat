use bevy::{prelude::*, window::{Cursor, PrimaryWindow, WindowResolution}};
use bevy::window::WindowLevel;
use enigo::MouseControllable;

use events::keyboard::keyboard_type;

mod events;

#[derive(Component)]
struct Cat {
    pub hungry: bool,
    pub thirsty: bool,
    pub angry: bool,
}

impl Cat {
    pub fn new(hungry: bool, thirsty: bool, angry: bool) -> Self {
        Self {
            hungry,
            thirsty,
            angry,
        }
    }
}

fn main() {
    let cat_window = Window {
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
            primary_window: Some(cat_window),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, move_window)
        .add_systems(FixedUpdate, keyboard_type).insert_resource(Time::<Fixed>::from_seconds(2.0))
    .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("cat.png"),
        ..default()
    });

    let second_window = commands.spawn(Window {
        decorations: true,
        resizable: false,
        resolution: WindowResolution::new(800.0, 600.0),
        ..Default::default()
    }).id();
}

fn move_window(mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    let e_mouse = enigo::Enigo::new();
    let (x, y) = e_mouse.mouse_location();
    windows.get_single_mut().unwrap().position = WindowPosition::At(IVec2::new(x + 10, y));
}
