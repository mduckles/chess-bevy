use bevy::{prelude::*, window::Window};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Chess".into(),
                resolution: (800., 800.).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .run();
}

fn draw_square(size: f32, color: (f32, f32, f32), posx: f32, posy: f32) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(color.0, color.1, color.2),
            custom_size: Some(Vec2::new(size, size)),
            ..default()
        },
        transform: Transform::from_xyz(posx, posy, 0.),
        ..default()
    }
}

#[derive(Component)]
struct MainCamera;

fn setup(mut commands: Commands) {
    let board_dimension = 8;
    let size = 100.0;
    let mut color: (f32, f32, f32);

    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(350., 350., 5.),
            ..default()
        },
        MainCamera,
    ));

    for row in 0..board_dimension {
        for column in 0..board_dimension {
            if (column + row) % 2 == 0 {
                color = (0.0, 0.0, 0.0);
            } else {
                color = (1., 1., 1.);
            }
            commands.spawn(draw_square(
                size,
                color,
                (row as f32) * size,
                (column as f32) * size,
            ));
        }
    }
    //square/rectangle
}
