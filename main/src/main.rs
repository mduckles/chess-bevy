use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::Window};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Chess"),

                ..default()
            }),
            ..Default::default()
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
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(
                (board_dimension as f32 * size) / 2.,
                (board_dimension as f32 * size) / 2.,
                5.,
            ),
            ..default()
        },
        MainCamera,
    ));
    for row in 0..board_dimension {
        for column in 0..board_dimension {
            if (column + row) % 2 == 0 {
                commands.spawn(draw_square(
                    size,
                    (0.0, 0.0, 0.0),
                    (row as f32) * size,
                    (column as f32) * size,
                ));
            } else {
                commands.spawn(draw_square(
                    size,
                    (1., 1., 1.),
                    (row as f32) * size,
                    (column as f32) * size,
                ));
            }
        }
    }
    //square/rectangle
}
