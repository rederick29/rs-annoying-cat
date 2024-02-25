use bevy::{prelude::*, render::camera::RenderTarget, window::{Cursor, PrimaryWindow, WindowRef, WindowResolution}, winit::WinitWindows};
use bevy::window::WindowLevel;
use enigo::MouseControllable;
use rand::Rng;

use events::quit::quit_program;
use events::keyboard::keyboard_type;

mod events;

const FIXED_UPDATE: f64 = 1.0/60.0;

#[derive(Component)]
struct Cat;

#[derive(Resource, Deref, DerefMut)]
#[repr(transparent)]
struct IsCatHungry(pub bool);

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
        resolution: WindowResolution::new(100.0, 100.0).with_scale_factor_override(1.0),
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
        // .add_systems(FixedFirst, is_cat_hungry)
        .add_systems(FixedUpdate, move_cat_away_from_x.run_if(should_move_mouse))
        .add_systems(FixedUpdate, toggle_cat_random_move.run_if(move_cat_every_20s))
        // .add_systems(FixedUpdate, cat_hungry)
        .add_systems(FixedUpdate, move_cat_random.run_if(should_cat_move_random))
        .insert_resource(Time::<Fixed>::from_seconds(FIXED_UPDATE))
        .insert_resource(ShouldMoveMouseAway(false))
        .insert_resource(ShouldCatMoveRandomly(false))
        // .insert_resource(IsCatHungry(false))
    .run();
}

fn run_every_10s(time: Res<Time>) -> bool {
    time.elapsed_seconds() % 4.0 == 0.0
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
        Cat,
        SpriteBundle {
            texture: asset_server.load("./cat_sprites/neutral.png"),
            transform: Transform::from_xyz(CAT_X, CAT_Y, CAT_Z).with_scale(Vec3::new(0.6, 0.6, 1.0)),
            ..default()
        },
    ));

    let second_window = commands.spawn(Window {
        decorations: true,
        resizable: false,
        position: WindowPosition::Centered(MonitorSelection::Current),
        resolution: WindowResolution::new(400.0, 300.0),
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

fn move_cat_away_from_x(mut dont_reset: Local<bool>, mut mouse_c: Local<(i32, i32)>, mut cat_c: Local<(i32, i32)>, mut loop_idx: Local<(i32, i32)>, mut window: Query<&mut Window, With<PrimaryWindow>>, mut move_mouse: ResMut<ShouldMoveMouseAway>, main_window: Query<&Window, Without<PrimaryWindow>>) {
    let (startx, starty) = &mut *mouse_c;
    let (endx, endy) = &mut *cat_c;
    let (i, j) = &mut *loop_idx;
    let main_window = main_window.get_single().unwrap();
    let window = &mut *window.get_single_mut().unwrap();
    (*startx, *starty) = match main_window.position {
        WindowPosition::At(v) => {
            (v.x + main_window.physical_width() as i32, v.y)
        },
        _ => { return; }
    };


    move_cat(&mut dont_reset, (startx, starty), (endx, endy), (i, j), window, &mut move_mouse.0);
}

fn move_cat(dont_reset: &mut bool, start_pos: (&mut i32, &mut i32), end_pos: (&mut i32, &mut i32), loop_idx: (&mut i32, &mut i32), window: &mut Window, move_condition: &mut bool) {    
    let mut e_mouse = enigo::Enigo::new();
    let (i, j) = loop_idx;
    let (end_x , end_y) = end_pos;
    let (start_x, start_y) = start_pos;
    if !*dont_reset {
        *i = 0;
        *j = 0;
        (*end_x, *end_y) = match window.position {
            WindowPosition::At(v) => {
                (v.x, v.y)
            },
            _ => { return; }
        };
        *move_condition = false;
        *dont_reset = true;
        return;
    }

    if *i < 100 && *j < 100 {
        window.position = WindowPosition::At(IVec2::new(*start_x + 10, *start_y));

        let dx = (*end_x - *start_x) / 100;
        let dy = (*end_y - *start_y) / 100;

        let new_pos_x = *start_x + *i * dx;
        let new_pos_y = *start_y + *j * dy;

        e_mouse.mouse_move_to(new_pos_x, new_pos_y);
        window.position = WindowPosition::At(IVec2::new(new_pos_x + 10, new_pos_y));
        *i += 1;
        *j += 1;
    } else {
        *dont_reset = false;
    }
}

fn mouse_on_x(main_window: Query<&Window, Without<PrimaryWindow>>, mut move_mouse: ResMut<ShouldMoveMouseAway>, mut material: Query<&mut Handle<Image>, With<Cat>>, asset_server: Res<AssetServer>) {
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
        let mut cat_sprite = material.get_single_mut().unwrap();
        *cat_sprite = asset_server.load("./cat_sprites/angry.png");
        **move_mouse = true;
    }
}

fn should_move_mouse(move_mouse: Res<ShouldMoveMouseAway>) -> bool {
    **move_mouse
}

fn should_cat_move_random(move_cat: Res<ShouldCatMoveRandomly>) -> bool {
    **move_cat
}

fn toggle_cat_random_move(mut commands: Commands, mut move_cat: ResMut<ShouldCatMoveRandomly>, asset_server: Res<AssetServer>) {
    commands.spawn(AudioBundle {
        source: asset_server.load("./audio/meow_4.ogg"),
        ..default()
    });
    **move_cat = true;
}

fn move_cat_every_20s(time: Res<Time>, move_mouse: Res<ShouldMoveMouseAway>) -> bool {
    time.elapsed_seconds() % 20.0 == 0.0 && !**move_mouse
}

fn is_cat_hungry(time: Res<Time>, move_mouse: Res<ShouldMoveMouseAway>, mut is_cat_hungry: ResMut<IsCatHungry>) {
    if time.elapsed_seconds() % 30.0 == 0.0 && !**move_mouse {
        **is_cat_hungry = true;
    }
}


fn get_random_coordinates(winit_windows: NonSend<WinitWindows>, entity: Entity, window: &mut Window) -> (i32, i32) {
    let monitor_size = winit_windows.get_window(entity).unwrap().current_monitor().unwrap().size();
    let random_x = rand::thread_rng().gen_range(0..monitor_size.width - window.physical_width() / 2);
    let random_y = rand::thread_rng().gen_range(0..monitor_size.height - window.physical_height() / 2);
    (random_x as i32, random_y as i32)
}

fn move_cat_random((mut moving, mut random_move): (Local<bool>, ResMut<ShouldCatMoveRandomly>), mut start_pos: Local<(i32, i32)>, mut end_pos: Local<(i32, i32)>, mut loop_idx: Local<(i32, i32)>, winit_windows: NonSend<WinitWindows>, mut window: Query<(Entity, &mut Window), With<PrimaryWindow>>, (mut material, asset_server, mut sprite): (Query<&mut Handle<Image>, With<Cat>>, Res<AssetServer>, Query<&mut Sprite, With<Cat>>)) {
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
        let mut cat_sprite = material.get_single_mut().unwrap();
        let mut sprite = sprite.get_single_mut().unwrap();
        *cat_sprite = asset_server.load("./cat_sprites/walking.png");
        sprite.flip_x = (*endx - *startx) < 0;
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

        let mut cat_sprite = material.get_single_mut().unwrap();
        *cat_sprite = asset_server.load("./cat_sprites/neutral.png");
    }
}

fn cat_hungry(mut main_window: Query<&mut Window, Without<PrimaryWindow>>, mut cat_window: Query<&mut Window, With<PrimaryWindow>>, mut dont_reset: Local<bool>, mut start_pos: Local<(i32, i32)>, mut end_pos: Local<(i32, i32)>, mut loop_idx: Local<(i32, i32)>, mut move_condition: ResMut<IsCatHungry>) {
    if !**move_condition { return; }
    let (startx, starty) = &mut *start_pos;
    let (endx, endy) = &mut *end_pos;
    let (i, j) = &mut *loop_idx;
    let mut cat_window = cat_window.get_single_mut().unwrap();
    let mut main_window = main_window.get_single_mut().unwrap();

    if !*dont_reset {
        (*startx, *starty) = match cat_window.position {
            WindowPosition::At(v) => {
                (v.x, v.y)
            },
            _ => { return; }
        };
        (*endx, *endy) = match main_window.position {
            WindowPosition::At(v) => {
                (v.x + main_window.physical_width() as i32 / 2, v.y + main_window.physical_height() as i32 / 2)
            },
            _ => { return; }
        };
    }

    main_window.set_minimized(false);

    move_cat(&mut dont_reset, (startx, starty), (endx, endy), (i, j), &mut cat_window, &mut move_condition);

}
