use bevy::{prelude::*, render::camera::ScalingMode, window::Window};

const SQUARE_SIZE: f32 = 100.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Chess".into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ChessBoard { board: Vec::new() })
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut board: ResMut<ChessBoard>) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 400.,
        min_height: 400.,
    };

    camera.projection.scale *= 2.25;

    camera.projection.scale = camera.projection.scale.clamp(0.5, 5.0);

    commands.spawn(camera);
    //local background
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::BEIGE,
            custom_size: Some(Vec2::new(13200., 13200.)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..default()
    });

    let mut color: Color;

    for row in 0..8 {
        for column in 0..8 {
            if (row + column) % 2 == 0 {
                color = Color::rgb(0., 0., 0.);
            } else {
                color = Color::rgb(1., 1., 1.);
            }
            board.board.push(Square {
                chess_pos: (
                    match row {
                        0 => Row::A,
                        1 => Row::B,
                        2 => Row::C,
                        3 => Row::D,
                        4 => Row::E,
                        5 => Row::F,
                        6 => Row::G,
                        7 => Row::H,
                        _ => Row::Invalid,
                    },
                    match column {
                        0 => Column::One,
                        1 => Column::Two,
                        2 => Column::Three,
                        3 => Column::Four,
                        4 => Column::Five,
                        5 => Column::Six,
                        6 => Column::Seven,
                        7 => Column::Eight,
                        _ => Column::Invalid,
                    },
                ),
                pos: Vec3::new(
                    row as f32 * SQUARE_SIZE - (SQUARE_SIZE / 2.),
                    column as f32 * SQUARE_SIZE - (SQUARE_SIZE / 2.),
                    1.,
                ),
                color,
            });
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(
                    row as f32 * SQUARE_SIZE - (SQUARE_SIZE * 4. - 50.),
                    column as f32 * SQUARE_SIZE - (SQUARE_SIZE * 4. - 50.),
                    0.,
                )),
                ..default()
            });
        }
    }
}

#[derive(Resource)]
struct ChessBoard {
    board: Vec<Square>,
}

#[derive(Component)]
struct Square {
    chess_pos: (Row, Column),
    pos: Vec3,
    color: Color,
}

enum Row {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    Invalid,
}

enum Column {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Invalid,
}
