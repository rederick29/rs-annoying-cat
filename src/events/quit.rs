use bevy::{app::AppExit, ecs::{event::EventWriter, system::Res}, input::{keyboard::KeyCode, ButtonInput}, log::info};

pub fn quit_program(keys: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.pressed(KeyCode::KeyQ) && keys.pressed(KeyCode::ControlLeft) && keys.pressed(KeyCode::ShiftLeft) {
        info!("Application quiting...");
        exit.send(AppExit);
    }
}