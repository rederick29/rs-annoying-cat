use bevy::{ecs::entity, prelude::*, render::{camera::RenderTarget, view::window}, utils::tracing::Instrument, window::{Cursor, PrimaryWindow, WindowRef, WindowResolution}, winit::WinitWindows};
use bevy::window::WindowLevel;
use enigo::MouseControllable;
use rand::Rng;

use events::quit::quit_program;
use events::keyboard::keyboard_type;

mod events;

const FIXED_UPDATE: f64 = 1.0/60.0;

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

#[derive(Resource, Deref, DerefMut)]
#[repr(transparent)]
struct ShouldMoveMouseAway(pub bool);

#[derive(Resource, Deref, DerefMut)]
#[repr(transparent)]
struct ShouldCatMoveRandomly(pub bool);

fn main() {
    let cat_window = Window {
        // Enable transparent support for the window
        transparent: true,
        decorations: false,
        window_level: WindowLevel::AlwaysOnTop,
        resizable: false,
        resolution: WindowResolution::new(300.0, 300.0),
        position: WindowPosition::Centered(MonitorSelection::Current),
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
        .add_systems(Update, quit_program)
        .add_systems(FixedUpdate, keyboard_type.run_if(run_every_10s))
        .add_systems(FixedFirst, mouse_on_x)
        .add_systems(FixedUpdate, move_cat.run_if(should_move_mouse))
        .add_systems(FixedUpdate, toggle_cat_random_move.run_if(move_cat_every_20s))
        .add_systems(FixedUpdate, move_cat_random.run_if(should_cat_move_random))
        .insert_resource(Time::<Fixed>::from_seconds(FIXED_UPDATE))
        .insert_resource(ShouldMoveMouseAway(false))
        .insert_resource(ShouldCatMoveRandomly(false))
    .run();
}

fn run_every_10s(time: Res<Time>) -> bool {
    time.elapsed_seconds() % 10.0 == 0.0
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    const CAT_X: f32 = 1000.0;
    const CAT_Y: f32 = 1000.0;
    const CAT_Z: f32 = -10.0;

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(CAT_X, CAT_Y, CAT_Z + 5.0),
        ..Default::default()
    });
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("./cat_sprites/neutral.png"),
            transform: Transform::from_xyz(CAT_X, CAT_Y, CAT_Z),
            ..default()
        },
    ));

    let second_window = commands.spawn(Window {
        decorations: true,
        resizable: false,
        position: WindowPosition::Centered(MonitorSelection::Current),
        resolution: WindowResolution::new(800.0, 600.0),
        ..Default::default()
    }).id();

    let second_camera = commands.spawn(Camera2dBundle {
        camera: Camera {
            target: RenderTarget::Window(WindowRef::Entity(second_window)),
            ..Default::default()
        },
        ..Default::default()
    }).id();

    commands
        .spawn((NodeBundle::default(), TargetCamera(second_camera)))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Second window",
                TextStyle::default(),
            ));
        });
}

fn move_cat(mut dont_reset: Local<bool>, mut mouse_c: Local<(i32, i32)>, mut cat_c: Local<(i32, i32)>, mut loop_idx: Local<(i32, i32)>, mut window: Query<&mut Window, With<PrimaryWindow>>, mut move_mouse: ResMut<ShouldMoveMouseAway>, main_window: Query<&Window, Without<PrimaryWindow>>) {
    info!("Move cat event taking place...");

    let mut window = window.get_single_mut().unwrap();
    let mut e_mouse = enigo::Enigo::new();
    let (i, j) = &mut *loop_idx;
    let (cat_x , cat_y) = &mut *cat_c;
    let (mouse_x, mouse_y) = &mut *mouse_c;
    let main_window = main_window.get_single().unwrap();
    (*mouse_x, *mouse_y) = match main_window.position {
        WindowPosition::At(v) => {
            (v.x + main_window.physical_width() as i32, v.y)
        },
        _ => { return; }
    };

    if !*dont_reset {
        *i = 0;
        *j = 0;
        (*cat_x, *cat_y) = match window.position {
            WindowPosition::At(v) => {
                (v.x, v.y)
            },
            _ => { return; }
        };
        **move_mouse = false;
        *dont_reset = true;
        return;
    }

    if *i < 100 && *j < 100 {
        window.position = WindowPosition::At(IVec2::new(*mouse_x + 10, *mouse_y));

        let dx = (*cat_x - *mouse_x) / 100;
        let dy = (*cat_y - *mouse_y) / 100;

        let new_pos_x = *mouse_x + *i * dx;
        let new_pos_y = *mouse_y + *j * dy;

        e_mouse.mouse_move_to(new_pos_x, new_pos_y);
        window.position = WindowPosition::At(IVec2::new(new_pos_x + 10, new_pos_y));
        *i += 1;
        *j += 1;
    } else {
        *dont_reset = false;
    }

    println!("i: {i}, wpos: {:?}", window.position);
}

fn mouse_on_x(main_window: Query<&Window, Without<PrimaryWindow>>, mut move_mouse: ResMut<ShouldMoveMouseAway>) {
    let main_window = main_window.get_single().unwrap();
    let main_window_pos = main_window.position;

    let (mouse_x, mouse_y) = enigo::Enigo::new().mouse_location();
    let (close_x, close_y) = match main_window_pos {
        WindowPosition::At(v) => {
            (v.x + main_window.physical_width() as i32, v.y)
        }
        _ => { return; }
    };

    if mouse_x >= close_x - 35 && mouse_x <= close_x + 10 && mouse_y <= close_y + 30 && mouse_y >= close_y {
        **move_mouse = true;
    }
}

fn should_move_mouse(move_mouse: Res<ShouldMoveMouseAway>) -> bool {
    **move_mouse
}

fn should_cat_move_random(move_cat: Res<ShouldCatMoveRandomly>) -> bool {
    **move_cat
}

fn toggle_cat_random_move(mut move_cat: ResMut<ShouldCatMoveRandomly>) {
    **move_cat = true;
}

fn move_cat_every_20s(time: Res<Time>, move_mouse: Res<ShouldMoveMouseAway>) -> bool {
    time.elapsed_seconds() % 20.0 == 0.0 && !**move_mouse
}

fn get_random_coordinates(winit_windows: NonSend<WinitWindows>, entity: Entity, mut window: &mut Window) -> (i32, i32) {
    let monitor_size = winit_windows.get_window(entity).unwrap().current_monitor().unwrap().size();
    let random_x = rand::thread_rng().gen_range(0..monitor_size.width);
    let random_y = rand::thread_rng().gen_range(0..monitor_size.height);
    (random_x as i32, random_y as i32)
}

fn move_cat_random(mut moving: Local<bool>, mut start_pos: Local<(i32, i32)>, mut end_pos: Local<(i32, i32)>, mut loop_idx: Local<(i32, i32)>, winit_windows: NonSend<WinitWindows>, mut window: Query<(Entity, &mut Window), With<PrimaryWindow>>, mut random_move: ResMut<ShouldCatMoveRandomly>) {
    if !**random_move { return };
    let (startx, starty) = &mut *start_pos;
    let (endx, endy) = &mut *end_pos;
    let (i, j) = &mut *loop_idx;
    let (entity, mut window) = window.get_single_mut().unwrap();

    if !*moving {
        (*startx, *starty) = match window.position {
            WindowPosition::At(v) => {
                (v.x, v.y)
            },
            _ => { return; }
        };
        (*endx, *endy) = get_random_coordinates(winit_windows, entity, &mut window);
        *i = 0;
        *j = 0;
        *moving = true;
    }

    if *i < 100 && *j < 100 {
        let dx = (*endx - *startx) / 100;
        let dy = (*endy - *starty) / 100;

        let new_pos_x = *startx + *i * dx;
        let new_pos_y = *starty + *j * dy;

        window.position = WindowPosition::At(IVec2::new(new_pos_x + 10, new_pos_y));
        *i += 1;
        *j += 1;
    } else {
        *moving = false;
        **random_move = false;
    }
}
