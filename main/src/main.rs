use bevy::{prelude::*, render::camera::ScalingMode, window::Window};
use Column::*;
use Row::*;

const SQUARE_SIZE: f32 = 100.;
const ROW: [Row; 8] = [A, B, C, D, E, F, G, H];
const COLUMN: [Column; 8] = [One, Two, Three, Four, Five, Six, Seven, Eight];

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Chess".into(),
                ..default()
            }),
            ..default()
        }))
        // .insert_resource(ChessBoard {
        //     board: vec![vec![Square {
        //         color: Color::WHITE,
        //         pos: Vec3::new(0., 0., 0.),
        //         chess_pos: (Column::One, Row::A),
        //     }]],
        // })
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 800.,
        min_height: 800.,
    };

    camera.transform = Transform::from_translation(Vec3::new(0., 0., 0.));

    commands.spawn(camera);

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(1., 1., 1.),
            custom_size: Some(Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..default()
    });

    // let mut color: Color;
    // for (i, row) in ROW.iter().enumerate() {
    //     board.board.push(Vec::new());
    //     for (j, column) in COLUMN.iter().enumerate() {
    //         if (i + j) % 2 == 0 {
    //             color = Color::rgb(1., 1., 1.)
    //         } else {
    //             color = Color::rgb(0., 0., 0.)
    //         }
    //         board.board[i].push(Square {
    //             chess_pos: (column.clone(), row.clone()),
    //             pos: Vec3::new(i as f32 * SQUARE_SIZE, j as f32 * SQUARE_SIZE, 0.),
    //             color,
    //         })
    //     }
    // }
    // //draw board
    // for row in board.board.clone() {
    //     for square in row {
    //         commands.spawn(draw_square(SQUARE_SIZE, square.color, square.pos));
    //     }
    // }
}

#[derive(Resource, Debug)]
pub struct ChessBoard {
    pub board: Vec<Vec<Square>>,
}

fn draw_square(size: f32, color: Color, pos: Vec3) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            color,
            custom_size: Some(Vec2::new(size, size)),
            ..default()
        },
        transform: Transform::from_translation(pos),
        ..default()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Row {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Column {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Square {
    pub chess_pos: (Column, Row),
    pub pos: Vec3,
    pub color: Color,
}
